use crate::audio::note_manager::ActiveNoteManager;
use crate::synths::manager::SynthType;
use eframe::egui;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct SynthesizerApp {
    // État du synthétiseur
    current_synth_type: SynthType,
    current_octave: i32,

    // Interface audio
    notes: Option<ActiveNoteManager>,
    synth_control: Option<Arc<Mutex<SynthType>>>,
    
    // Suivi des notes actuellement pressées
    pressed_notes: HashSet<String>,

    // États de l'interface
    volume: f32,
    reverb_wet: f32,
    show_keyboard: bool,
    show_effects: bool,
}

impl SynthesizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configuration du thème
        configure_fonts(&cc.egui_ctx);

        Self {
            current_synth_type: SynthType::n_sine(),
            current_octave: 5,
            notes: None,
            synth_control: None,
            pressed_notes: HashSet::new(),
            volume: 0.7,
            reverb_wet: 0.2,
            show_keyboard: true,
            show_effects: true,
        }
    }

    pub fn with_audio(mut self, notes: ActiveNoteManager) -> Self {
        self.notes = Some(notes);
        self
    }

    pub fn with_synth_control(mut self, synth_control: Arc<Mutex<SynthType>>) -> Self {
        self.synth_control = Some(synth_control);
        self
    }
}

impl eframe::App for SynthesizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Panel du haut - Contrôles principaux
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Synthétiseur", |ui| {
                    if ui.button("Sine").clicked() {
                        self.current_synth_type = SynthType::n_sine();
                        self.update_synth_type();
                    }
                    if ui.button("Square").clicked() {
                        self.current_synth_type = SynthType::n_square();
                        self.update_synth_type();
                    }
                    if ui.button("Sawtooth").clicked() {
                        self.current_synth_type = SynthType::n_sawtooth();
                        self.update_synth_type();
                    }
                    if ui.button("FM").clicked() {
                        self.current_synth_type = SynthType::n_fm();
                        self.update_synth_type();
                    }
                    if ui.button("Hammond").clicked() {
                        self.current_synth_type = SynthType::n_hammond();
                        self.update_synth_type();
                    }
                });

                ui.menu_button("Affichage", |ui| {
                    ui.checkbox(&mut self.show_keyboard, "Clavier virtuel");
                    ui.checkbox(&mut self.show_effects, "Panneau d'effets");
                });

                ui.separator();
                ui.label(format!("Type actuel: {:?}", self.current_synth_type));
            });
        });

        // Panel de gauche - Effets et contrôles
        if self.show_effects {
            egui::SidePanel::left("effects_panel")
                .resizable(true)
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.heading("🎛️ Contrôles");

                    ui.separator();

                    // Volume général
                    ui.horizontal(|ui| {
                        ui.label("Volume:");
                        ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Vol"));
                    });

                    // Octave
                    ui.horizontal(|ui| {
                        ui.label("Octave:");
                        ui.add(egui::Slider::new(&mut self.current_octave, 1..=8).text("Oct"));
                    });

                    ui.separator();

                    // Section Reverb
                    ui.heading("🌊 Reverb");
                    ui.horizontal(|ui| {
                        ui.label("Wet:");
                        ui.add(egui::Slider::new(&mut self.reverb_wet, 0.0..=1.0).text("Wet"));
                    });

                    ui.separator();

                    // Informations
                    ui.heading("ℹ️ Info");
                    if let Some(ref notes) = self.notes {
                        let notes_guard = notes.lock().unwrap();
                        ui.label(format!("Notes actives: {}", notes_guard.len()));
                    } else {
                        ui.label("Audio non connecté");
                    }
                });
        }

        // Panel principal - Clavier virtuel et visualisations
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎹 Synthétiseur Rust");

            if self.show_keyboard {
                ui.separator();
                self.draw_virtual_keyboard(ui);
            }

            ui.separator();

            // Zone d'informations
            ui.group(|ui| {
                ui.heading("💡 Instructions");
                ui.label("Touches du clavier:");
                ui.label("• Q,W,E,R,T,Y,U - Notes naturelles");
                ui.label("• 1,2,3,4,5 - Notes dièses");
                ui.label("• Flèches ← → - Changer d'octave");
                ui.label("• W,X,S,H,K - Changer de synthétiseur");
                ui.label("• ESPACE - Arrêter toutes les notes");
            });
        });
    }
}

impl SynthesizerApp {
    fn draw_virtual_keyboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎹 Clavier virtuel");

        // Touches blanches (notes naturelles)
        ui.horizontal(|ui| {
            let white_keys = ["C", "D", "E", "F", "G", "A", "B"];
            for key in &white_keys {
                let button = egui::Button::new(*key).min_size(egui::vec2(120.0, 180.0));

                let response = ui.add(button);
                let key_string = key.to_string();
                
                // Si le bouton est pressé et la note n'était pas déjà active
                if response.is_pointer_button_down_on() && !self.pressed_notes.contains(&key_string) {
                    self.pressed_notes.insert(key_string.clone());
                    self.play_note(key);
                }
                
                // Si le bouton n'est plus pressé et la note était active
                if !response.is_pointer_button_down_on() && self.pressed_notes.contains(&key_string) {
                    self.pressed_notes.remove(&key_string);
                    self.stop_note(key);
                }
            }
        });

        // Touches noires (dièses)
        ui.horizontal(|ui| {
            let black_keys = ["","C#", "", "D#", "", "", "F#", "", "G#", "", "A#", ""];
            for (_i, key) in black_keys.iter().enumerate() {
                if key.is_empty() {
                    ui.add_space(67.0); // Espace pour alignement
                } else {
                    let button = egui::Button::new(*key)
                        .min_size(egui::vec2(80.0, 150.0))
                        .fill(egui::Color32::from_rgb(50, 50, 50));

                    let response = ui.add(button);
                    let key_string = key.to_string();
                    
                    // Si le bouton est pressé et la note n'était pas déjà active
                    if response.is_pointer_button_down_on() && !self.pressed_notes.contains(&key_string) {
                        self.pressed_notes.insert(key_string.clone());
                        self.play_note(key);
                    }
                    
                    // Si le bouton n'est plus pressé et la note était active
                    if !response.is_pointer_button_down_on() && self.pressed_notes.contains(&key_string) {
                        self.pressed_notes.remove(&key_string);
                        self.stop_note(key);
                    }
                }
            }
        });

        ui.label(format!("Octave actuelle: {}", self.current_octave));
    }

    /// Met à jour le type de synthétiseur dans le système audio
    fn update_synth_type(&mut self) {
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                *synth = self.current_synth_type.clone();
            }
        }
    }

    /// Joue une note
    fn play_note(&mut self, note_name: &str) {
        if let Some(ref notes) = self.notes {
            let frequency = self.note_to_frequency(note_name);
            self.add_note(notes, frequency);
        }
    }
    
    /// Arrête une note
    fn stop_note(&mut self, note_name: &str) {
        if let Some(ref notes) = self.notes {
            let frequency = self.note_to_frequency(note_name);
            self.remove_note(notes, frequency);
        }
    }

    /// Convertit un nom de note en fréquence
    fn note_to_frequency(&self, note_name: &str) -> f64 {
        let base_freq = match note_name {
            "C" => 261.63,
            "C#" => 277.18,
            "D" => 293.66,
            "D#" => 311.13,
            "E" => 329.63,
            "F" => 349.23,
            "F#" => 369.99,
            "G" => 392.00,
            "G#" => 415.30,
            "A" => 440.00,
            "A#" => 466.16,
            "B" => 493.88,
            _ => 440.0, // Défaut
        };

        // Ajuster pour l'octave
        base_freq * (2.0_f64).powi(self.current_octave - 4)
    }

    /// Ajoute une note au système audio
    fn add_note(&self, notes: &crate::audio::note_manager::ActiveNoteManager, frequency: f64) {
        use crate::audio::note_manager::ActiveNote;
        use crate::consts::constants::SAMPLE_RATE;

        let frequency_key = (frequency * 100.0) as u64;
        let active_note = ActiveNote::new(frequency, SAMPLE_RATE);

        if let Ok(mut notes_guard) = notes.lock() {
            notes_guard.insert(frequency_key, active_note);
            println!("Note ajoutée: {:.2} Hz ({})", frequency, frequency_key);
        }
    }
    
    /// Supprime une note du système audio
    fn remove_note(&self, notes: &crate::audio::note_manager::ActiveNoteManager, frequency: f64) {
        let frequency_key = (frequency * 100.0) as u64;

        if let Ok(mut notes_guard) = notes.lock() {
            if let Some(note) = notes_guard.get_mut(&frequency_key) {
                // Déclencher le release de l'ADSR au lieu de supprimer directement
                note.adsr.note_off();
                println!("Note en release: {:.2} Hz ({})", frequency, frequency_key);
            }
        }
    }
}

fn configure_fonts(ctx: &egui::Context) {
    // Configuration de base des polices
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}
