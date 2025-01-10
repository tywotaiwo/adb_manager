use eframe::egui;
use std::process::Command;
use super::devices_tab::Device;

pub struct ReverseTcpTab {
    port: String,
    devices: Vec<Device>,
    status_messages: Vec<String>,
}

impl Default for ReverseTcpTab {
    fn default() -> Self {
        Self {
            port: String::from("8081"),
            devices: Vec::new(),
            status_messages: Vec::new(),
        }
    }
}

impl ReverseTcpTab {
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

    pub fn setup_reverse_tcp_all(&mut self) {
        self.status_messages.clear();
        
        for device in &self.devices {
            match Command::new("adb")
                .args(["-s", &device.id, "reverse", &format!("tcp:{}", self.port), &format!("tcp:{}", self.port)])
                .output()
            {
                Ok(output) => {
                    let message = format!(
                        "Device {}: {}", 
                        device.id, 
                        String::from_utf8_lossy(&output.stdout).trim()
                    );
                    self.status_messages.push(message);
                }
                Err(e) => {
                    let error_message = format!("Error for device {}: {}", device.id, e);
                    self.status_messages.push(error_message);
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        ui.heading("Reverse TCP Setup");
        
        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.text_edit_singleline(&mut self.port);
        });

        ui.horizontal(|ui| {
            if ui.button("Refresh Devices").clicked() {
                self.refresh_devices();
            }
            if ui.button("Setup Reverse TCP for All").clicked() {
                self.setup_reverse_tcp_all();
            }
        });

        if !self.devices.is_empty() {
            ui.separator();
            ui.heading("Connected Devices:");
            for device in &self.devices {
                ui.label(format!("• {} ({})", device.id, device.status));
            }
        }

        if !self.status_messages.is_empty() {
            ui.separator();
            ui.heading("Status:");
            for message in &self.status_messages {
                ui.label(message);
            }
        }

        None
    }
} 