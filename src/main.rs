use eframe::egui;
use std::process::Command;

struct Device {
    id: String,
    status: String,
}

struct AdbManagerApp {
    ip: String,
    port: String,
    status_message: String,
    devices: Vec<Device>,
    selected_tab: usize,
}

impl Default for AdbManagerApp {
    fn default() -> Self {
        Self {
            ip: String::from("192.168.1.105"),
            port: String::new(),
            status_message: String::new(),
            devices: Vec::new(),
            selected_tab: 0,
        }
    }
}

impl AdbManagerApp {
    fn refresh_devices(&mut self) {
        self.devices.clear();
        
        if let Ok(output) = Command::new("adb").args(["devices"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Skip the first line which is "List of devices attached"
            for line in output_str.lines().skip(1) {
                if !line.trim().is_empty() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        self.devices.push(Device {
                            id: parts[0].to_string(),
                            status: parts[1].to_string(),
                        });
                    }
                }
            }
        }
    }

    fn disconnect_device(&mut self, device_id: &str) {
        if let Ok(output) = Command::new("adb")
            .args(["disconnect", device_id])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            self.status_message = result.to_string();
            self.refresh_devices();
        }
    }
}

impl eframe::App for AdbManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == 0, "Connect").clicked() {
                    self.selected_tab = 0;
                }
                if ui.selectable_label(self.selected_tab == 1, "Devices").clicked() {
                    self.selected_tab = 1;
                    self.refresh_devices();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                0 => {
                    // Connect Tab
                    ui.heading("ADB Connection Manager");
                    
                    ui.horizontal(|ui| {
                        ui.label("IP Address:");
                        ui.text_edit_singleline(&mut self.ip);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.text_edit_singleline(&mut self.port);
                    });
                    
                    if ui.button("Connect").clicked() {
                        let address = format!("{}:{}", self.ip, self.port);
                        match Command::new("adb")
                            .args(["connect", &address])
                            .output() 
                        {
                            Ok(output) => {
                                let result = String::from_utf8_lossy(&output.stdout);
                                self.status_message = result.to_string();
                                self.refresh_devices();
                            }
                            Err(e) => {
                                self.status_message = format!("Error: {}", e);
                            }
                        }
                    }
                }
                1 => {
                    // Devices Tab
                    ui.heading("Connected Devices");
                    
                    if ui.button("Refresh").clicked() {
                        self.refresh_devices();
                    }
                    
                    for device in &self.devices {
                        ui.horizontal(|ui| {
                            ui.label(format!("{} ({})", device.id, device.status));
                            if ui.button("Disconnect").clicked() {
                                self.disconnect_device(&device.id);
                            }
                        });
                    }
                }
                _ => unreachable!(),
            }
            
            if !self.status_message.is_empty() {
                ui.separator();
                ui.label(&self.status_message);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "ADB Manager",
        options,
        Box::new(|_cc| Box::new(AdbManagerApp::default())),
    )
}
