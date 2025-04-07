use eframe::egui;
use std::sync::Arc;
use crate::adb::AdbManager;
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Debug)]
struct FileEntry {
    name: String,
    is_dir: bool,
}

#[derive(Debug, PartialEq)]
enum StorageLocation {
    Internal,
    SDCard,
    Custom,
}

impl StorageLocation {
    fn get_path(&self) -> &str {
        match self {
            StorageLocation::Internal => "/storage/emulated/0",
            StorageLocation::SDCard => "/storage/self/primary",
            StorageLocation::Custom => "",
        }
    }
}

pub struct FileManagerTab {
    adb_manager: Arc<AdbManager>,
    current_device: String,
    current_path: String,
    device_files: Vec<FileEntry>,
    selected_file: Option<String>,
    storage_location: StorageLocation,
}

impl Default for FileManagerTab {
    fn default() -> Self {
        Self {
            adb_manager: Arc::new(AdbManager::new()),
            current_device: String::new(),
            current_path: "/storage/emulated/0".to_string(),
            device_files: Vec::new(),
            selected_file: None,
            storage_location: StorageLocation::Internal,
        }
    }
}

impl FileManagerTab {
    pub fn new(adb_manager: Arc<AdbManager>) -> Self {
        Self {
            adb_manager,
            current_device: String::new(),
            current_path: "/storage/emulated/0".to_string(),
            device_files: Vec::new(),
            selected_file: None,
            storage_location: StorageLocation::Internal,
        }
    }

    fn update_file_list(&mut self) {
        if !self.current_device.is_empty() {
            // List files using ls -la to get file type information
            if let Ok(output) = self.adb_manager.run_adb_command(&[
                "-s", &self.current_device,
                "shell",
                "ls", "-la",
                &self.current_path
            ]) {
                self.device_files.clear();
                for line in output.lines() {
                    if line.is_empty() || line.starts_with("total") {
                        continue;
                    }
                    
                    let is_dir = line.chars().nth(0) == Some('d');
                    if let Some(name) = line.split_whitespace().last() {
                        if name != "." && name != ".." {
                            self.device_files.push(FileEntry {
                                name: name.to_string(),
                                is_dir,
                            });
                        }
                    }
                }
                self.device_files.sort_by(|a, b| {
                    if a.is_dir == b.is_dir {
                        a.name.cmp(&b.name)
                    } else {
                        b.is_dir.cmp(&a.is_dir)
                    }
                });
            }
        }
    }

    fn copy_to_pc(&self) {
        if let Some(selected) = &self.selected_file {
            if let Some(path) = FileDialog::new()
                .set_title("Save file to PC")
                .set_file_name(selected)
                .save_file() {
                let source_path = format!("{}/{}", self.current_path, selected);
                let _ = self.adb_manager.run_adb_command(&[
                    "-s", &self.current_device,
                    "pull",
                    &source_path,
                    path.to_str().unwrap_or("")
                ]);
            }
        }
    }

    fn copy_to_device(&self) {
        if let Some(path) = FileDialog::new()
            .set_title("Select file to copy to device")
            .pick_file() {
            let destination = format!("{}/", self.current_path);
            let _ = self.adb_manager.run_adb_command(&[
                "-s", &self.current_device,
                "push",
                path.to_str().unwrap_or(""),
                &destination
            ]);
        }
    }

    fn delete_file(&mut self) {
        if let Some(selected) = &self.selected_file {
            let path = format!("{}/{}", self.current_path, selected);
            let _ = self.adb_manager.run_adb_command(&[
                "-s", &self.current_device,
                "shell",
                "rm", "-rf",
                &path
            ]);
            self.selected_file = None;
            self.update_file_list();
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        ui.vertical(|ui| {
            // Header section
            ui.heading("File Manager");
            
            // Controls section - using a fixed area for controls
            egui::Frame::none().show(ui, |ui| {
                // Device selection
                ui.horizontal(|ui| {
                    ui.label("Device:");
                    if let Ok(devices) = self.adb_manager.get_devices() {
                        egui::ComboBox::from_label("")
                            .selected_text(&self.current_device)
                            .show_ui(ui, |ui| {
                                for device in devices {
                                    if ui.selectable_label(self.current_device == device, &device).clicked() {
                                        self.current_device = device;
                                        self.update_file_list();
                                    }
                                }
                            });
                    }
                });

                // Storage location selection
                ui.horizontal(|ui| {
                    ui.label("Storage:");
                    if ui.selectable_label(self.storage_location == StorageLocation::Internal, "Internal Storage").clicked() {
                        self.storage_location = StorageLocation::Internal;
                        self.current_path = self.storage_location.get_path().to_string();
                        self.update_file_list();
                    }
                    if ui.selectable_label(self.storage_location == StorageLocation::SDCard, "SD Card").clicked() {
                        self.storage_location = StorageLocation::SDCard;
                        self.current_path = self.storage_location.get_path().to_string();
                        self.update_file_list();
                    }
                    if ui.selectable_label(self.storage_location == StorageLocation::Custom, "Custom").clicked() {
                        self.storage_location = StorageLocation::Custom;
                    }
                });

                // Current path with navigation buttons
                ui.horizontal(|ui| {
                    if ui.button("⬆ Up").clicked() {
                        let path = PathBuf::from(&self.current_path);
                        if let Some(parent) = path.parent() {
                            if let Some(parent_str) = parent.to_str() {
                                if parent_str.starts_with(self.storage_location.get_path()) 
                                    || self.storage_location == StorageLocation::Custom {
                                    self.current_path = parent_str.to_string();
                                    self.update_file_list();
                                }
                            }
                        }
                    }
                    ui.label("Path:");
                    let response = ui.text_edit_singleline(&mut self.current_path);
                    if response.lost_focus() {
                        self.update_file_list();
                    }
                });
            });

            ui.separator();

            // File list in a scrollable area that takes available space
            let available_height = ui.available_height() - 40.0; // Reserve space for buttons
            egui::ScrollArea::vertical()
                .max_height(available_height)
                .show(ui, |ui| {
                    let mut clicked_dir: Option<String> = None;
                    
                    for file in &self.device_files {
                        let label = if file.is_dir {
                            format!("📁 {}", file.name)
                        } else {
                            format!("📄 {}", file.name)
                        };
                        
                        if ui.selectable_label(self.selected_file.as_ref() == Some(&file.name), label).clicked() {
                            if file.is_dir {
                                clicked_dir = Some(file.name.clone());
                            } else {
                                self.selected_file = Some(file.name.clone());
                            }
                        }
                    }
                    
                    if let Some(dir_name) = clicked_dir {
                        self.current_path = format!("{}/{}", self.current_path, dir_name);
                        self.selected_file = None;
                        self.update_file_list();
                    }
                });

            ui.separator();

            // File operations - always at the bottom
            ui.horizontal(|ui| {
                if ui.button("Copy to PC").clicked() && self.selected_file.is_some() {
                    self.copy_to_pc();
                }
                if ui.button("Copy to Device").clicked() {
                    self.copy_to_device();
                    self.update_file_list();
                }
                if ui.button("Delete").clicked() && self.selected_file.is_some() {
                    self.delete_file();
                }
                if ui.button("Refresh").clicked() {
                    self.update_file_list();
                }
            });
        });

        None
    }
} 