use eframe::egui;
use std::process::Command;

pub struct ConnectTab {
    ip: String,
    port: String,
}

impl Default for ConnectTab {
    fn default() -> Self {
        Self {
            ip: String::from("192.168.1.105"),
            port: String::new(),
        }
    }
}

impl ConnectTab {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        ui.heading("ADB Connection Manager");
        
        ui.horizontal(|ui| {
            ui.label("IP Address:");
            ui.text_edit_singleline(&mut self.ip);
        });
        
        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.text_edit_singleline(&mut self.port);
        });
        
        let mut message = None;
        if ui.button("Connect").clicked() {
            let address = format!("{}:{}", self.ip, self.port);
            match Command::new("adb")
                .args(["connect", &address])
                .output() 
            {
                Ok(output) => {
                    message = Some(String::from_utf8_lossy(&output.stdout).to_string());
                }
                Err(e) => {
                    message = Some(format!("Error: {}", e));
                }
            }
        }
        message
    }
} 