use eframe::{App, Frame, NativeOptions};
use egui::Context;
use egui_chip::{ChipEdit, ChipEditBuilder};
use egui_dropdown::DropDownBox;

struct ExampleApp {
    items: Vec<ChipEdit>,
    buf: ChipEdit,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    DropDownBox::from_iter(&mut self.items, "test_dropbox", &mut self.buf)
                        // choose whether to filter the box items based on what is in the text edit already
                        // default is true when this is not used
                        .filter_by_input(false),
                );

                if ui.button("Add").clicked() {
                    self.items.push(self.buf.clone());
                }
                //self.buf.show(ui)
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
                    ChipEditBuilder::new(",")
                        .unwrap()
                        .texts(vec!["First", "Second"])
                        .chip_size(Some([80., 20.]))
                        .build(),
                    ChipEditBuilder::new(",")
                        .unwrap()
                        .texts(vec!["Second", "Third"])
                        .chip_size(Some([80., 20.]))
                        .build(),
                    ChipEditBuilder::new(",")
                        .unwrap()
                        .texts(vec!["Third", "Other"])
                        .chip_size(Some([80., 20.]))
                        .build(),
                    ChipEditBuilder::new(",")
                        .unwrap()
                        .texts(vec!["Other", "Misc"])
                        .chip_size(Some([80., 20.]))
                        .build(),
                ],
                buf: ChipEditBuilder::new(",")
                    .unwrap()
                    .texts(vec![String::from("hello")])
                    .chip_size(Some([80., 20.]))
                    .build(),
            }))
        }),
    )
    .unwrap();
}
