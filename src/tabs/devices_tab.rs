use eframe::egui;
use std::process::Command;

#[derive(Clone)]
pub struct Device {
    pub id: String,
    pub status: String,
}

pub struct DevicesTab {
    devices: Vec<Device>,
}

impl Default for DevicesTab {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}

impl DevicesTab {
    pub fn refresh_devices(&mut self) {
        self.devices.clear();
        
        if let Ok(output) = Command::new("adb").args(["devices"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
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

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        ui.heading("Connected Devices");
        
        if ui.button("Refresh").clicked() {
            self.refresh_devices();
        }
        
        let devices_to_process: Vec<Device> = self.devices.clone();
        let mut message = None;

        for device in devices_to_process {
            ui.horizontal(|ui| {
                ui.label(format!("{} ({})", device.id, device.status));
                if ui.button("Disconnect").clicked() {
                    if let Ok(output) = Command::new("adb")
                        .args(["disconnect", &device.id])
                        .output()
                    {
                        message = Some(String::from_utf8_lossy(&output.stdout).to_string());
                        self.refresh_devices();
                    }
                }
            });
        }
        
        message
    }
} 