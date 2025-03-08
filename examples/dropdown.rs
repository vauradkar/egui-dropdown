use eframe::{App, Frame, NativeOptions};
use egui::{text_edit::TextEditOutput, Context};
use egui_dropdown::DropDownBox;

struct ExampleApp {
    items: Vec<String>,
    current: String,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let dd: DropDownBox<'_, TextEditOutput, String> =
                    DropDownBox::from_iter(&mut self.items, "test_dropbox", &mut self.current)
                        // choose whether to filter the box items based on what is in the text edit already
                        // default is true when this is not used
                        .filter_by_input(true);
                ui.add(dd);

                if ui.button("Add").clicked() {
                    self.items.push(self.current.clone());
                }
            });
        });
    }
}

fn main() {
    eframe::run_native(
        "egui-dropdown",
        NativeOptions::default(),
        Box::new(|_| {
            Ok(Box::new(ExampleApp {
                items: vec![
                    "First".into(),
                    "Second".into(),
                    "Third".into(),
                    "Other".into(),
                ],
                current: String::new(),
            }))
        }),
    )
    .unwrap();
}
