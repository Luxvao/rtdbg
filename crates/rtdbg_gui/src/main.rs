mod preload;
mod run;

use std::{f32, sync::mpsc::Receiver};

use egui::{CentralPanel, Layout, ScrollArea, TextEdit, Vec2};
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
        if !self.inject_menu_enabled && !self.display_error {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Add the PID label
                    ui.label(format!("PID: {}", self.pid));

                    // Text editors
                    ui.horizontal(|ui| {
                        // Output text
                        ui.vertical(|ui| {
                            ui.label("Output:");

                            ui.allocate_ui_with_layout(
                                Vec2::new(ui.available_width(), 30f32),
                                Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ScrollArea::vertical()
                                        .scroll_bar_visibility(
                                            egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                        )
                                        .id_salt("Output scroll area")
                                        .show(ui, |ui| {
                                            ui.add(
                                                TextEdit::multiline(&mut self.output)
                                                    .desired_rows(30),
                                            );
                                        })
                                },
                            );
                            let response = ui.button("Inject");

                            if response.clicked() {
                                self.inject_menu_enabled = true;
                            }
                        });

                        // Script editor
                        ui.vertical(|ui| {
                            ui.label("Script:");

                            ui.allocate_ui_with_layout(
                                Vec2::new(ui.available_width(), 31f32),
                                Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ScrollArea::vertical()
                                        .scroll_bar_visibility(
                                            egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                        )
                                        .id_salt("Script scroll area")
                                        .show(ui, |ui| {
                                            ui.add(
                                                TextEdit::multiline(&mut self.script)
                                                    .desired_rows(30)
                                                    .code_editor(),
                                            );
                                        })
                                },
                            );

                            let run_response = ui.button("Run");

                            if run_response.clicked() {
                                let res = run::run_script(self.pid, self.script.clone());

                                if let Err(e) = res {
                                    self.error = format!("{e}");
                                    self.display_error = true;
                                }
                            }
                        });
                    });
                });
            });
        }

        // Inject window
        if self.inject_menu_enabled && !self.display_error {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Label
                    ui.label("PID/Path:");
                    ui.text_edit_singleline(&mut self.inject_pid_path);
                    ui.horizontal(|ui| {
                        // Buttons
                        let preload_resopnse = ui.button("Preload");

                        let ptrace_reseponse = ui.button("Ptrace");

                        let return_response = ui.button("Return");

                        if preload_resopnse.clicked() {
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
            CentralPanel::default().show(ctx, |ui| {
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
