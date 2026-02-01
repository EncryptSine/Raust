mod core;
use crate::core::models::{Vault, PasswordEntry};
use crate::core::{crypto, storage};
use eframe::egui;
use std::path::PathBuf;

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 600.0])
            .with_resizable(true),
        ..Default::default()

    };

    eframe::run_native(
        "Raust Mdp",
        options,
        Box::new(|_cc| Ok(Box::new(RaustApp::default()))),
    )
}

struct RaustApp {
    is_unlocked: bool,
    master_password: String,
    error_msg: String,
    vault: Vault,
    salt: Vec<u8>,
    master_key: Vec<u8>,
    path: PathBuf,
    new_s: String,
    new_u: String,
    new_p: String,
}

impl Default for RaustApp {

    fn default() -> Self {
        Self {
            is_unlocked: false,
            master_password: String::new(),
            error_msg: String::new(),
            vault: Vault::new(),
            salt: Vec::new(),
            master_key: Vec::new(),
            path: PathBuf::from("pass.raust"),
            new_s: String::new(),
            new_u: String::new(),
            new_p: String::new(),

        }
    }
}

impl RaustApp {

    fn try_unlock(&mut self) {
        if self.path.exists() {
            match storage::load_from_file(&self.path) {
                Ok((s, n, data)) => {
                    self.salt = s;
                    match crypto::derive_key(&self.master_password, &self.salt) {
                        Ok(key) => {
                            match crypto::decrypt_vault(&data, &key, &n) {
                                Ok(v) => {
                                    self.vault = v;
                                    self.master_key = key;
                                    self.is_unlocked = true;
                                    self.error_msg = String::new();
                                }
                                Err(_) => self.error_msg = "Mot de passe incorrect".into(),
                            }
                        }
                        Err(e) => self.error_msg = format!("Erreur clé: {}", e),
                    }
                }
                Err(e) => self.error_msg = format!("Erreur fichier: {}", e),
            }
        } else {
            self.salt = crypto::generate_random_bytes(16);
            if let Ok(key) = crypto::derive_key(&self.master_password, &self.salt) {
                self.master_key = key;
                self.vault = Vault::new();
                self.is_unlocked = true;
                self.error_msg = String::new();
            }
        }
    }

    fn save_vault(&mut self) {
        if let Ok((enc, n)) = crypto::encrypt_vault(&self.vault, &self.master_key) {
            let _ = storage::save_to_file(&self.path, &self.salt, &n, &enc);
        }
    }
}

impl eframe::App for RaustApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.is_unlocked {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading("Raust Mdp");
                    ui.add_space(20.0);
                    
                    let label = if self.path.exists() { "Mot de passe" } else { "Pas de sauvegarde, Créez un mdp" };
                    ui.label(label);
                    
                    let edit = ui.add(egui::TextEdit::singleline(&mut self.master_password)
                        .password(true)
                        .hint_text("Mdp maitre"));

                    if ui.button("Déverrouiller").clicked() || (edit.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                        self.try_unlock();
                    }

                    if !self.error_msg.is_empty() {
                        ui.add_space(10.0);
                        ui.colored_label(egui::Color32::LIGHT_RED, &self.error_msg);
                    }
                });
            } else {

                ui.horizontal(|ui| {
                    ui.heading("Vos accès");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Verrouiller").clicked() {
                            self.save_vault();
                            self.is_unlocked = false;
                            self.master_password.clear();
                            self.master_key.clear();
                        }
                    });
                });
                
                ui.separator();

                ui.group(|ui| {
                    ui.label("Nouveau mdp");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.new_s).hint_text("Plateforme"));
                        ui.add(egui::TextEdit::singleline(&mut self.new_u).hint_text("Utilisateur"));
                        ui.add(egui::TextEdit::singleline(&mut self.new_p).hint_text("Mdp").password(true));
                        
                        if ui.button("+").clicked() {
                            if !self.new_s.is_empty() {
                                self.vault.add_entry(PasswordEntry::new(self.new_s.clone(), self.new_u.clone(), self.new_p.clone()));
                                self.new_s.clear(); self.new_u.clear(); self.new_p.clear();
                                self.save_vault();
                            }
                        }
                    });
                });

                ui.add_space(15.0);

                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    let mut suppr = None;
                    
                    for (i, e) in self.vault.entries.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.strong(&e.service);
                                    ui.label(egui::RichText::new(&e.username).color(egui::Color32::GRAY));
                                });
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("Supprimer").clicked() { suppr = Some(i); }
                                    if ui.button("Copier").clicked() {
                                        ctx.copy_text(e.password.clone());
                                    }
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }

                    if let Some(i) = suppr {
                        self.vault.delete_at(i);
                        self.save_vault();
                    }
                });
            }
        });
    }
}