use eframe::egui;
use egui::ViewportBuilder;
mod tabs;
use tabs::{ConnectTab, DevicesTab, ReverseTcpTab};

struct AdbManagerApp {
    connect_tab: ConnectTab,
    devices_tab: DevicesTab,
    reverse_tcp_tab: ReverseTcpTab,
    selected_tab: usize,
    status_message: String,
}

impl Default for AdbManagerApp {
    fn default() -> Self {
        Self {
            connect_tab: ConnectTab::default(),
            devices_tab: DevicesTab::default(),
            reverse_tcp_tab: ReverseTcpTab::default(),
            selected_tab: 0,
            status_message: String::new(),
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
                    self.devices_tab.refresh_devices();
                }
                if ui.selectable_label(self.selected_tab == 2, "Reverse TCP").clicked() {
                    self.selected_tab = 2;
                    self.reverse_tcp_tab.refresh_devices();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(message) = match self.selected_tab {
                0 => self.connect_tab.show(ui),
                1 => self.devices_tab.show(ui),
                2 => self.reverse_tcp_tab.show(ui),
                _ => unreachable!(),
            } {
                self.status_message = message;
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
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "ADB Manager",
        options,
        Box::new(|_cc| Box::new(AdbManagerApp::default())),
    )
}
