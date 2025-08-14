use crate::audio::note_manager::ActiveNoteManager;
use crate::synths::manager::SynthType;
use eframe::egui;

pub struct SynthesizerApp {
    // État du synthétiseur
    current_synth_type: SynthType,
    current_octave: i32,
    
    // Interface audio (optionnel pour l'instant)
    notes: Option<ActiveNoteManager>,
    
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
}

impl eframe::App for SynthesizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Panel du haut - Contrôles principaux
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Synthétiseur", |ui| {
                    if ui.button("Sine").clicked() {
                        self.current_synth_type = SynthType::n_sine();
                    }
                    if ui.button("Square").clicked() {
                        self.current_synth_type = SynthType::n_square();
                    }
                    if ui.button("Sawtooth").clicked() {
                        self.current_synth_type = SynthType::n_sawtooth();
                    }
                    if ui.button("FM").clicked() {
                        self.current_synth_type = SynthType::n_fm();
                    }
                    if ui.button("Hammond").clicked() {
                        self.current_synth_type = SynthType::n_hammond();
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
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.heading("🎛️ Contrôles");
                    
                    ui.separator();
                    
                    // Volume général
                    ui.horizontal(|ui| {
                        ui.label("Volume:");
                        ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0)
                            .text("Vol"));
                    });
                    
                    // Octave
                    ui.horizontal(|ui| {
                        ui.label("Octave:");
                        ui.add(egui::Slider::new(&mut self.current_octave, 1..=8)
                            .text("Oct"));
                    });
                    
                    ui.separator();
                    
                    // Section Reverb
                    ui.heading("🌊 Reverb");
                    ui.horizontal(|ui| {
                        ui.label("Wet:");
                        ui.add(egui::Slider::new(&mut self.reverb_wet, 0.0..=1.0)
                            .text("Wet"));
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
                let button = egui::Button::new(*key)
                    .min_size(egui::vec2(40.0, 80.0));
                
                if ui.add(button).clicked() {
                    // Ici on pourrait déclencher la note
                    println!("Note cliquée: {}", key);
                }
            }
        });
        
        // Touches noires (dièses)
        ui.horizontal(|ui| {
            let black_keys = ["C#", "", "D#", "", "", "F#", "", "G#", "", "A#", ""];
            for (_i, key) in black_keys.iter().enumerate() {
                if key.is_empty() {
                    ui.add_space(40.0); // Espace pour alignement
                } else {
                    let button = egui::Button::new(*key)
                        .min_size(egui::vec2(30.0, 50.0))
                        .fill(egui::Color32::from_rgb(50, 50, 50));
                    
                    if ui.add(button).clicked() {
                        println!("Note dièse cliquée: {}", key);
                    }
                }
            }
        });
        
        ui.label(format!("Octave actuelle: {}", self.current_octave));
    }
}

fn configure_fonts(ctx: &egui::Context) {
    // Configuration de base des polices
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}
