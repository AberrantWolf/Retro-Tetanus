use std::path::PathBuf;

use rfd::FileDialog;

use eframe::egui;
use egui_extras::{TableBuilder, Column};
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
            ui.horizontal(|ui|{
                ui.label("Source Path");
                ui.add(egui::TextEdit::singleline(&mut self.source_path).hint_text("Path to where all your stuff is at..."));
                if ui.button("Browse").clicked() {
                    let picked = FileDialog::new().set_directory(&self.source_path).pick_folder();
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
                            if valid_entry.file_type().is_dir() { continue; }


                            let stem = if let Some(stem) = &valid_entry.path().file_stem() {
                                stem.to_string_lossy().into()
                            } else {
                                "".to_owned()
                            };

                            let extension = if let Some(extension) = valid_entry.path().extension() {
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
                        },
                        Err(err) => println!("Error entry: {}", err),
                    }
                }
                print!("Found something like {}", &self.source_listings.len());
            }
            ui.separator();
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            egui::ScrollArea::both().auto_shrink([false, false]).show_rows(ui, row_height, self.source_listings.len(), |ui, row_range| {
                egui::Grid::new("source_listing_grid").start_row(row_range.start).num_columns(2).show(ui, |ui|{
                    for row in row_range {
                        let listing = &self.source_listings[row];

                        ui.label(&listing.stem);
                        ui.label(&listing.extension);
                        ui.end_row();
                    }
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let app_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Retro Tetanus",
        app_options,
        Box::new(|cc| {
            Box::<TetanusApp>::default()
        }),
    )
}
