
use egui_multiselect::MultiSelect;
use eframe::egui;
use egui_notify::Toasts;


#[derive(Default)]
struct ExampleApp {
    options: Vec<String>,
    items: Vec<String>,
    ms_answers: Vec<String>,
    max_opt: u8,
    toasted: bool,
    toasts: Toasts,
}

impl ExampleApp {
    fn new() -> Self {
        Self {
            options: vec![
                "First item".into(),
                "Second thing".into(),
                "Third choice".into(),
                "Anther option with longer string".into(),
            ],
            ms_answers: Vec::new(),
            items: Vec::new(),
            max_opt: 3,
            toasted: false,
            toasts: Toasts::default(),
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(MultiSelect::new(
                    format!("test_multiselect {}", &self.max_opt),
                    &mut self.items,
                    &mut self.ms_answers,
                    &self.options,
                    |ui, _text| ui.selectable_label(false, _text),
                    &self.max_opt,
                    &mut self.toasted,
                ));
            });
            if self.toasted {
                let caption = format!("Max: {} selections", &self.max_opt);
                self.toasts.custom(caption, egui_phosphor::regular::LIGHTBULB.to_owned(), egui::Color32::from_rgba_premultiplied(0, 255, 0, 255));
                self.toasted = false;
            }
        });
        self.toasts.show(ctx);
    }
}

fn main() {
    eframe::run_native(
        "egui-multiselect",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            let mut font_def = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut font_def, egui_phosphor::Variant::Regular);
            for data in font_def.font_data.values_mut() {
                data.tweak.scale = 1.20;
            };
            cc.egui_ctx.set_fonts(font_def);
            cc.egui_ctx.style_mut(|style| style.spacing.item_spacing = egui::vec2(8.0, 5.0));
            Ok(Box::new(ExampleApp::new()))
        }),
    )
    .unwrap();
}