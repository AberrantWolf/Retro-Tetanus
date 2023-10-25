use std::path::PathBuf;

use egui::Widget;
use rfd::FileDialog;

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use walkdir::WalkDir;

struct SourceListing {
    path: PathBuf,
    stem: String,
    extension: String,
}

struct TetanusApp {
    source_path: String,
    source_listings: Vec<SourceListing>,
}

impl Default for TetanusApp {
    fn default() -> Self {
        TetanusApp {
            source_path: "".to_owned(),
            source_listings: Default::default(),
        }
    }
}

impl eframe::App for TetanusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Retro Tetanus");
            ui.horizontal(|ui| {
                ui.label("Source Path");
                ui.add(
                    egui::TextEdit::singleline(&mut self.source_path)
                        .hint_text("Path to where all your stuff is at..."),
                );
                if ui.button("Browse").clicked() {
                    let picked = FileDialog::new()
                        .set_directory(&self.source_path)
                        .pick_folder();
                    if let Some(directory) = picked {
                        self.source_path = directory.to_str().unwrap().to_owned();
                    }
                }
            });
            if ui.button("Update").clicked() {
                // TODO: Move to a thread...
                self.source_listings.clear();
                for entry in WalkDir::new(&self.source_path) {
                    match entry {
                        Ok(valid_entry) => {
                            if valid_entry.file_type().is_dir() {
                                continue;
                            }

                            let stem = if let Some(stem) = &valid_entry.path().file_stem() {
                                stem.to_string_lossy().into()
                            } else {
                                "".to_owned()
                            };

                            let extension = if let Some(extension) = valid_entry.path().extension()
                            {
                                extension.to_string_lossy().into()
                            } else {
                                "".into()
                            };

                            // TODO: Filter out non-ROM-types...
                            self.source_listings.push(SourceListing {
                                path: valid_entry.path().to_path_buf(),
                                stem,
                                extension,
                            });
                        }
                        Err(err) => println!("Error entry: {}", err),
                    }
                }
                println!("Found something like {}", &self.source_listings.len());
            }
            ui.separator();
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true).clip(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("File");
                    });
                    header.col(|ui| {
                        ui.heading("Ext");
                    });
                })
                .body(|mut body| {
                    for listing in &self.source_listings {
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                egui::Label::new(&listing.stem).wrap(false).ui(ui);
                            });
                            row.col(|ui| {
                                egui::Label::new(&listing.extension).wrap(false).ui(ui);
                            });
                        })
                    }
                });
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let app_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Retro Tetanus",
        app_options,
        Box::new(|cc| Box::<TetanusApp>::default()),
    )
}
