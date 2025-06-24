mod preload;
mod run;

use std::{f32, sync::mpsc::Receiver};

use egui::{CentralPanel, Layout, ScrollArea, TextEdit, Vec2, Window};
use preload::preload;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rtdbg GUI",
        options,
        Box::new(|_| Ok(Box::<RtdbgGui>::default())),
    )
}

#[derive(Default)]
struct RtdbgGui {
    pid: u32,
    output: String,
    script: String,
    inject_menu_enabled: bool,
    inject_pid_path: String,
    // Child output receiver
    receiver: Option<Receiver<String>>,

    // Error related
    display_error: bool,
    error: String,
}

impl eframe::App for RtdbgGui {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Read from the receiver
        if let Some(receiver) = self.receiver.as_mut() {
            if let Ok(output) = receiver.try_recv() {
                self.output.push_str(&output);
            }
        }

        // Main window
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(format!("PID: {}", self.pid));

                let total_width = ui.available_width();

                let total_height = ui.available_height();

                let spacing = ui.spacing().item_spacing.x;

                let column_width = (total_width - spacing) / 2.0;

                ui.horizontal(|ui| {
                    // Output
                    ui.vertical(|ui| {
                        ui.set_min_width(column_width);

                        ui.label("Output:");

                        ScrollArea::vertical()
                            .id_salt("Output scroller")
                            .min_scrolled_height(total_height - 40.0)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    Vec2 {
                                        x: column_width,
                                        y: total_height - 40.0,
                                    },
                                    TextEdit::multiline(&mut self.output).interactive(false),
                                )
                            });

                        let inject_button = ui.button("Inject");

                        if inject_button.clicked() {
                            self.inject_menu_enabled = true;
                        }
                    });

                    // Script
                    ui.vertical(|ui| {
                        ui.set_min_height(ui.available_height());

                        ui.set_min_width(column_width);

                        ui.label("Script:");

                        ScrollArea::vertical()
                            .id_salt("Script scroller")
                            .min_scrolled_height(total_height - 40.0)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    Vec2 {
                                        x: column_width,
                                        y: total_height - 40.0,
                                    },
                                    TextEdit::multiline(&mut self.script).code_editor(),
                                )
                            });
                        let run_button = ui.button("Run");

                        if run_button.clicked() {
                            let res = run::run_script(self.script.clone());

                            if let Err(e) = res {
                                self.error = format!("{}", e);
                                self.display_error = true;
                            }
                        }
                    });
                });
            });
        });

        // Inject window
        if self.inject_menu_enabled && !self.display_error {
            Window::new("Inject").auto_sized().show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Label
                    ui.label("PID/Path:");
                    ui.text_edit_singleline(&mut self.inject_pid_path);
                    ui.horizontal(|ui| {
                        // Buttons
                        let preload_resopnse = ui.button("Preload");

                        let ptrace_reseponse = ui.button("Ptrace");

                        let return_response = ui.button("Close");

                        if preload_resopnse.clicked() {
                            if self.pid != 0 {
                                self.error = "A process is already being controlled!".to_string();

                                self.display_error = true;

                                return;
                            }

                            let res = preload(
                                ctx,
                                self.inject_pid_path.clone(),
                                &mut self.pid,
                                &mut self.receiver,
                            );

                            if let Err(e) = res {
                                self.error = format!("{e}");
                                self.display_error = true;
                            }

                            // Return to main menu
                            self.inject_menu_enabled = false;
                        }

                        if ptrace_reseponse.clicked() {
                            if self.pid != 0 {
                                self.error = "A process is already being controlled!".to_string();

                                self.display_error = true;

                                return;
                            }

                            self.error = String::from("Not yet implemented!");
                            self.display_error = true;
                        }

                        if return_response.clicked() {
                            self.inject_menu_enabled = false;
                        }
                    });
                });
            });
        }

        // Errors
        if self.display_error {
            Window::new("Error").auto_sized().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(&self.error);
                    let close_response = ui.button("Close");

                    if close_response.clicked() {
                        self.display_error = false;
                    }
                });
            });
        }
    }
}
