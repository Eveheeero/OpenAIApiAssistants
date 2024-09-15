#![cfg_attr(not(test), windows_subsystem = "windows")]

mod api;
mod automation;
pub(crate) mod error;
mod fonts;
pub(crate) mod prelude;
use eframe::{egui, App};

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.maximized = Some(true);
    eframe::run_native(
        "OpenAI API Assistants",
        native_options,
        Box::new(|cc| Ok(Box::new(Assistants::new(cc)))),
    )
}

struct Assistants {
    api_key: String,
}

impl Assistants {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 폰트 설정
        cc.egui_ctx.set_fonts(fonts::get_fonts());

        let db = sled::open(".assistants").unwrap();
        let api_key = db.get("api_key").unwrap().unwrap_or_default();
        let api_key = String::from_utf8(api_key.to_vec()).unwrap();

        Self { api_key }
    }
}
impl App for Assistants {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
