use std::path::PathBuf;
use eframe::egui;

use clef_client_core::Client;

struct App {
    client: Option<Client>,
    identity_path: PathBuf,
}

impl App {
    fn new() -> Self {
        let identity_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("clef")
            .join("identity.key");

        Self {
            client: None,
            identity_path,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.client.is_none() {
                ui.label("No identity loaded");

                if ui.button("Create new identity").clicked() {
                    let client = Client::create_new(&self.identity_path).unwrap();
                    self.client = Some(client);
                }

                if ui.button("Load identity").clicked() {
                    if let Ok(client) = Client::load(&self.identity_path) {
                        self.client = Some(client);
                    }
                }
            } else {
                ui.label("Identity loaded ✅");
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Clef",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}