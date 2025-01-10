use eframe::egui;
use std::process::Command;

struct AdbManagerApp {
    ip: String,
    port: String,
    status_message: String,
}

impl Default for AdbManagerApp {
    fn default() -> Self {
        Self {
            ip: String::from("192.168.1.105"),
            port: String::new(),
            status_message: String::new(),
        }
    }
}

impl eframe::App for AdbManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
            }
            
            if !self.status_message.is_empty() {
                ui.label(&self.status_message);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "ADB Manager",
        options,
        Box::new(|_cc| Box::new(AdbManagerApp::default())),
    )
}
