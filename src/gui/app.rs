use crate::audio::note_manager::ActiveNoteManager;
use crate::consts::constants::{self, BLACK_KEYS, USED_KEYS, WHITE_KEYS};
use crate::input::key_handlers::NOTES;
use crate::synths::manager::SynthType;
use crate::synths::modules::lfo::LfoWaveform;
use eframe::egui;
use egui::RichText;
//use egui::{Color32, RichText};
use std::collections::HashSet;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

pub struct SynthesizerApp {
    // État du synthétiseur
    current_synth_type: SynthType,

    // Interface audio
    notes: Option<ActiveNoteManager>,
    synth_control: Option<Arc<Mutex<SynthType>>>,

    // Suivi des notes actuellement pressées
    pressed_notes: HashSet<String>,         // Pour le clavier virtuel
    pressed_physical_keys: HashSet<String>, // Pour le clavier physique
    active_notes: HashSet<String>,          // Notes réellement actives (unifiées)

    // États de l'interface
    // NOISE
    noise_activation: bool,
    noise: f64,

    // GAIN
    gain_activation: bool,
    gain: f64,

    // ADSR
    attack: f64,
    decay: f64,
    sustain: f64,
    release: f64,

    // FILTER
    filter_activation: bool,

    // COMPRESSOR
    compressor_activation: bool,

    // LFO
    lfo_activation: bool,
    freq: f64,
    waveform: LfoWaveform,

    // REVERB
    reverb_activation: bool,
    reverb_dry_wet: f64,

    // OCTAVE
    current_octave: usize,

    show_keyboard: bool,
    show_effects: bool,
}

impl SynthesizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configuration du thème
        configure_fonts(&cc.egui_ctx);
        Self {
            current_synth_type: SynthType::n_sine(),
            notes: None,
            synth_control: None,
            pressed_notes: HashSet::new(),
            pressed_physical_keys: HashSet::new(),
            active_notes: HashSet::new(),

            noise_activation: constants::ACTIVATION_NOISE,
            noise: constants::CURRENT_NOISE,

            gain_activation: constants::ACTIVATION_GAIN,
            gain: constants::CURRENT_GAIN,

            attack: constants::ADSR_ATTACK,
            decay: constants::ADSR_DECAY,
            sustain: constants::ADSR_SUSTAIN,
            release: constants::ADSR_RELEASE,

            lfo_activation: constants::ACTIVATION_LFO,
            freq: constants::CURRENT_LFO_FREQ,
            waveform: constants::CURRENT_LFO_WAVEFORM,

            reverb_activation: constants::ACTIVATION_REVERB,
            reverb_dry_wet: constants::CURRENT_DRY_WET,

            filter_activation: constants::ACTIVATION_FILTER,

            compressor_activation: constants::ACTIVATION_COMPRESSOR,

            current_octave: constants::VECTEUR_NOTES
                [constants::CURRENT_OCTAVE_INDEX.load(Ordering::Relaxed)]
                as usize,

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
        // Pour que la fenêtre recoive les événements clavier
        ctx.request_repaint();
        // Gérer les événements clavier (comme dans le terminal)
        self.handle_keyboard_input(ctx);

        // Panel du haut - Contrôles principaux
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
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
                    ui.heading("Contrôles");

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.heading("⏱ ADSR");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Attack:");
                        if ui
                            .add(egui::Slider::new(&mut self.attack, 0.0..=60.0))
                            .changed()
                        {
                            //update attack
                        };
                    });

                    ui.horizontal(|ui| {
                        ui.label("Decay:");
                        if ui
                            .add(egui::Slider::new(&mut self.decay, 0.0..=30.0))
                            .changed()
                        {
                            //update decay
                        };
                    });

                    ui.horizontal(|ui| {
                        ui.label("Sustain:");
                        if ui
                            .add(egui::Slider::new(&mut self.sustain, 0.0..=1.0))
                            .changed()
                        {
                            //update decay
                        };
                    });

                    ui.horizontal(|ui| {
                        ui.label("Release:");
                        if ui
                            .add(egui::Slider::new(&mut self.release, 0.0..=60.0))
                            .changed()
                        {
                            //update release
                        };
                    });
                    ui.separator();

                    // Noise
                    ui.horizontal(|ui| {
                        ui.heading("📻 Noise");
                        ui.add_space(10.0); // Espace pour séparer le heading de la checkbox
                        if ui.checkbox(&mut self.noise_activation, "ON").changed() {
                            self.update_noise_activation();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Niveau de bruit:");
                        if ui
                            .add(egui::Slider::new(&mut self.noise, 0.0..=1.0))
                            .changed()
                        {
                            self.update_synth_noise();
                        };
                    });

                    ui.separator();

                    // LFO
                    ui.horizontal(|ui| {
                        ui.heading("🔄 LFO");
                        ui.add_space(10.0);
                        if ui.checkbox(&mut self.lfo_activation, "ON").changed() {
                            self.update_lfo_activation();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Fréquence:");
                        if ui
                            .add(egui::Slider::new(&mut self.freq, 0.01..=1000.0))
                            .changed()
                        {
                            self.update_synth_lfo();
                        };
                    });
                    ui.horizontal(|ui| {
                        ui.label("Forme d'onde:");
                        let old_waveform = self.waveform;
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.waveform))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.waveform, LfoWaveform::Sine, "Sine");
                                ui.selectable_value(
                                    &mut self.waveform,
                                    LfoWaveform::Triangle,
                                    "Triangle",
                                );
                                ui.selectable_value(
                                    &mut self.waveform,
                                    LfoWaveform::Square,
                                    "Square",
                                );
                                ui.selectable_value(
                                    &mut self.waveform,
                                    LfoWaveform::SawUp,
                                    "Sawtooth Up",
                                );
                                ui.selectable_value(
                                    &mut self.waveform,
                                    LfoWaveform::SawDown,
                                    "Sawtooth Down",
                                );
                            });
                        if old_waveform != self.waveform {
                            println!(
                                "Waveform changée de {:?} vers {:?}",
                                old_waveform, self.waveform
                            );
                            self.update_synth_lfo_waveform();
                        }
                    });

                    ui.separator();
                    // Filter
                    ui.horizontal(|ui| {
                        ui.heading("⬇ Filter");
                        ui.add_space(10.0);
                        if ui.checkbox(&mut self.filter_activation, "ON").changed() {
                            self.update_filter_activation();
                        }
                    });

                    ui.separator();

                    // Gain général
                    ui.horizontal(|ui| {
                        ui.heading("🔊 Gain");
                        ui.add_space(10.0); // Espace pour séparer le heading de la checkbox
                        if ui.checkbox(&mut self.gain_activation, "ON").changed() {
                            self.update_gain_activation();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Gain:");
                        if ui
                            .add(egui::Slider::new(&mut self.gain, -12.0..=6.0).text("dB"))
                            .changed()
                        {
                            self.update_synth_gain();
                        }
                    });

                    ui.separator();
                    // Compressor
                    ui.horizontal(|ui| {
                        ui.heading("🤏 Compressor");
                        ui.add_space(10.0);
                        if ui.checkbox(&mut self.compressor_activation, "ON").changed() {
                            //self.update_compressor_activation();
                        }
                    });

                    ui.separator();

                    // Section Reverb
                    ui.horizontal(|ui| {
                        ui.heading("🌊 Reverb");
                        ui.add_space(10.0);
                        if ui.checkbox(&mut self.reverb_activation, "ON").changed() {
                            //self.update_reverb_activation();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Dry Wet:");
                        ui.add(egui::Slider::new(&mut self.reverb_dry_wet, 0.0..=1.0).text("%"));
                    });

                    ui.separator();

                    // Octave (correspondant au système JSON 1-9)
                    ui.heading("🎵 Octave");
                    ui.horizontal(|ui| {
                        ui.label("Octave:");
                        let mut new_octave = self.current_octave;
                        if ui.add(egui::Slider::new(&mut new_octave, 1..=9)).changed() {
                            self.current_octave = new_octave;
                            self.update_global_octave();
                        }
                    });

                    ui.separator();

                    // Informations
                    ui.heading("ℹ Info");
                    if let Some(ref notes) = self.notes {
                        let notes_guard = notes.lock().unwrap();
                        let released_notes_count = notes_guard
                            .values()
                            .filter(|note| note.adsr.is_released())
                            .count();
                        ui.label(format!("Notes actives: {}", released_notes_count));
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
                ui.label(RichText::new("Clavier physique :"));
                ui.label(RichText::new(
                    "• Q,B,C,D,E,F,G - Notes naturelles (A,B,C,D,E,F,G)",
                ));
                ui.label(RichText::new("• 1,2,3,4,5 - Notes dièses (A#,C#,D#,F#,G#)"));
                ui.label(RichText::new("• Flèches ← → - Changer d'octave"));
                ui.label(RichText::new("• W,X,S,K,H - Changer de synthétiseur"));
                ui.label(RichText::new("• ESPACE - Arrêter toutes les notes"));
                ui.separator();
                ui.label(RichText::new("Clavier virtuel :"));
                ui.label(RichText::new("• Cliquez une fois pour démarrer une note"));
                ui.label(RichText::new("• Cliquez à nouveau pour l'arrêter"));
            });
        });
    }
}

impl SynthesizerApp {
    /// Gère les événements du clavier physique (comme dans le terminal)
    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        use egui::Key;

        ctx.input(|i| {
            // Touches pour les notes
            self.handle_note_key(i, Key::Q, "A"); // Q -> A
            self.handle_note_key(i, Key::B, "B"); // B -> B  
            self.handle_note_key(i, Key::C, "C"); // C -> C
            self.handle_note_key(i, Key::D, "D"); // D -> D
            self.handle_note_key(i, Key::E, "E"); // E -> E
            self.handle_note_key(i, Key::F, "F"); // F -> F
            self.handle_note_key(i, Key::G, "G"); // G -> G

            // Touches pour les dièses
            self.handle_note_key(i, Key::Num1, "A#"); // 1 -> A#
            self.handle_note_key(i, Key::Num2, "C#"); // 2 -> C#
            self.handle_note_key(i, Key::Num3, "D#"); // 3 -> D#
            self.handle_note_key(i, Key::Num4, "F#"); // 4 -> F#
            self.handle_note_key(i, Key::Num5, "G#"); // 5 -> G#

            // Changement d'octave
            if i.key_pressed(Key::ArrowLeft) && self.current_octave > 1 {
                self.current_octave -= 1;
                self.update_global_octave();
                println!("Octave changée vers: {}", self.current_octave);
            }
            if i.key_pressed(Key::ArrowRight) && self.current_octave < 9 {
                self.current_octave += 1;
                self.update_global_octave();
                println!("Octave changée vers: {}", self.current_octave);
            }

            // Changement de synthétiseur
            if i.key_pressed(Key::W) {
                self.current_synth_type = SynthType::n_sine();
                self.update_synth_type();
                println!("Synthétiseur changé: Modular Sine");
            }
            if i.key_pressed(Key::X) {
                self.current_synth_type = SynthType::n_square();
                self.update_synth_type();
                println!("Synthétiseur changé: Modular Square");
            }
            if i.key_pressed(Key::S) {
                self.current_synth_type = SynthType::n_sawtooth();
                self.update_synth_type();
                println!("Synthétiseur changé: Modular Sawtooth");
            }
            if i.key_pressed(Key::K) {
                self.current_synth_type = SynthType::n_fm();
                self.update_synth_type();
                println!("Synthétiseur changé: FM");
            }
            if i.key_pressed(Key::H) {
                self.current_synth_type = SynthType::n_hammond();
                self.update_synth_type();
                println!("Synthétiseur changé: Hammond Organ");
            }

            // Arrêter toutes les notes
            if i.key_pressed(Key::Space) {
                self.stop_all_notes();
                println!("Toutes les notes arrêtées");
            }
        });
    }

    /// Gère une touche de note (press/release)
    fn handle_note_key(&mut self, input: &egui::InputState, key: egui::Key, note: &str) {
        let key_string = format!("physical_{}", note);

        // Note pressée
        if input.key_pressed(key) && !self.pressed_physical_keys.contains(&key_string) {
            self.pressed_physical_keys.insert(key_string.clone());
            self.play_note(note);
            println!("Touche physique {} pressée -> {}", key_string, note);
        }

        // Note relâchée
        if input.key_released(key) && self.pressed_physical_keys.contains(&key_string) {
            self.pressed_physical_keys.remove(&key_string);
            self.stop_note(note);
            println!("Touche physique {} relâchée -> {}", key_string, note);
        }
    }

    /// Arrête toutes les notes en cours
    fn stop_all_notes(&mut self) {
        if let Some(ref notes) = self.notes {
            if let Ok(mut notes_guard) = notes.lock() {
                for note in notes_guard.values_mut() {
                    note.adsr.note_off();
                }
            }
        }
        // Vider tous les sets de notes pressées
        self.pressed_notes.clear();
        self.pressed_physical_keys.clear();
        self.active_notes.clear();
        println!("Toutes les notes arrêtées et sets vidés");
    }

    fn draw_virtual_keyboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎹 Clavier virtuel");

        // Touches blanches (notes naturelles)
        ui.horizontal(|ui| {
            let white_keys = ["C", "D", "E", "F", "G", "A", "B"];
            for key in &white_keys {
                let key_string = key.to_string();
                let is_active = self.pressed_notes.contains(&key_string);

                let button = egui::Button::new(*key)
                    .min_size(egui::vec2(120.0, 180.0))
                    .fill(if is_active {
                        egui::Color32::from_rgb(USED_KEYS.0, USED_KEYS.1, USED_KEYS.2)
                    } else {
                        egui::Color32::from_rgb(WHITE_KEYS.0, WHITE_KEYS.1, WHITE_KEYS.2)
                    });

                let response = ui.add(button);

                // Approche simple: utiliser clicked pour démarrer/arrêter en toggle
                if response.clicked() {
                    if self.pressed_notes.contains(&key_string) {
                        // Si déjà active, l'arrêter
                        self.pressed_notes.remove(&key_string);
                        self.stop_note(key);
                        println!("Note virtuelle arrêtée (toggle): {}", key);
                    } else {
                        // Si pas active, la démarrer
                        self.pressed_notes.insert(key_string.clone());
                        self.play_note(key);
                        println!("Note virtuelle démarrée (toggle): {}", key);
                    }
                }
            }
        });

        // Touches noires (dièses)
        ui.horizontal(|ui| {
            let black_keys = ["", "C#", "", "D#", "", "", "F#", "", "G#", "", "A#", ""];
            for key in black_keys.iter() {
                if key.is_empty() {
                    ui.add_space(67.0); // Espace pour alignement
                } else {
                    let key_string = key.to_string();
                    let is_active = self.pressed_notes.contains(&key_string);

                    let button = egui::Button::new(*key)
                        .min_size(egui::vec2(80.0, 150.0))
                        .fill(if is_active {
                            egui::Color32::from_rgb(USED_KEYS.0, USED_KEYS.1, USED_KEYS.2)
                        } else {
                            egui::Color32::from_rgb(BLACK_KEYS.0, BLACK_KEYS.1, BLACK_KEYS.2)
                        });

                    let response = ui.add(button);

                    // Utiliser clicked pour démarrer/arrêter en toggle
                    if response.clicked() {
                        if self.pressed_notes.contains(&key_string) {
                            // Si déjà active, l'arrêter
                            self.pressed_notes.remove(&key_string);
                            self.stop_note(key);
                            println!("Note virtuelle arrêtée (toggle): {}", key);
                        } else {
                            // Si pas active, démarrer la note
                            self.pressed_notes.insert(key_string.clone());
                            self.play_note(key);
                            println!("Note virtuelle démarrée (toggle): {}", key);
                        }
                    }
                }
            }
        });

        ui.label(format!("Octave actuelle: {}", self.current_octave));
    }

    /// Met à jour le type de synthétiseur
    fn update_synth_type(&mut self) {
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                *synth = self.current_synth_type.clone();
            }
        }
        // Synchroniser les valeurs après avoir relâché le lock
        self.sync_values_from_synth();
    }

    /// Synchronise les valeurs de l'interface avec le synthétiseur actuel
    fn sync_values_from_synth(&mut self) {
        self.gain = self.current_synth_type.get_current_gain();
        self.gain_activation = self.current_synth_type.is_gain_active();
        self.noise = self.current_synth_type.get_current_noise();
        self.noise_activation = self.current_synth_type.is_noise_active();
        self.waveform = self.current_synth_type.get_current_lfo_waveform();
        self.freq = self.current_synth_type.get_current_lfo_frequency();
        self.lfo_activation = self.current_synth_type.is_lfo_active();
        // self.attack = self.current_synth_type.get_current_attack();
        //self.decay = self.current_synth_type.get_current_decay();
        //self.sustain = self.current_synth_type.get_current_sustain();
        //self.release = self.current_synth_type.get_current_release();
    }

    /// Met à jour le gain dans le synthétiseur actuel
    fn update_synth_gain(&mut self) {
        // Mettre à jour le gain dans le synthétiseur local
        self.current_synth_type.set_current_gain(self.gain);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_current_gain(self.gain);
                println!("Gain mis à jour dans le contrôleur audio: {}", self.gain);
            }
        }
    }

    fn update_synth_noise(&mut self) {
        // Mettre à jour UNIQUEMENT le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_current_noise(self.noise);
                println!("Bruit mis à jour dans le contrôleur audio: {}", self.noise);

                // Synchroniser la copie locale avec l'état du contrôleur
                self.current_synth_type = synth.clone();
            }
        }
    }

    fn update_synth_lfo(&mut self) {
        // Mettre à jour la fréquence dans le synthétiseur local
        self.current_synth_type.set_current_lfo_frequency(self.freq);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_current_lfo_frequency(self.freq);
                println!(
                    "Fréquence LFO mise à jour dans le contrôleur audio: {}",
                    self.freq
                );
            }
        }
    }

    fn update_filter_activation(&mut self) {
        println!("Activation du filtre changée: {}", self.filter_activation);

        self.current_synth_type
            .set_filter_activation(self.filter_activation);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_filter_activation(self.filter_activation);
                println!(
                    "Activation du filtre mise à jour dans le contrôleur audio: {}",
                    self.filter_activation
                );
            }
        }
    }

    /// Met à jour l'activation du gain
    fn update_gain_activation(&mut self) {
        println!("Activation du gain changée: {}", self.gain_activation);

        self.current_synth_type
            .set_gain_activation(self.gain_activation);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_gain_activation(self.gain_activation);
                println!(
                    "Activation du gain mise à jour dans le contrôleur audio: {}",
                    self.gain_activation
                );
            }
        }
    }

    fn update_reverb_activation(&mut self) {
        println!(
            "Activation de la réverbération changée: {}",
            self.reverb_activation
        );

        self.current_synth_type
            .set_reverb_activation(self.reverb_activation);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_reverb_activation(self.reverb_activation);
                println!(
                    "Activation de la réverbération mise à jour dans le contrôleur audio: {}",
                    self.reverb_activation
                );
            }
        }
    }

    fn update_noise_activation(&mut self) {
        println!("Activation du bruit changée: {}", self.noise_activation);

        self.current_synth_type
            .set_noise_activation(self.noise_activation);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_noise_activation(self.noise_activation);
                println!(
                    "Activation du bruit mise à jour dans le contrôleur audio: {}",
                    self.noise_activation
                );
            }
        }
    }

    fn update_lfo_activation(&mut self) {
        println!("Activation du LFO changée: {}", self.lfo_activation);

        self.current_synth_type
            .set_lfo_activation(self.lfo_activation);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_lfo_activation(self.lfo_activation);
                println!(
                    "Activation du LFO mise à jour dans le contrôleur audio: {}",
                    self.lfo_activation
                );
            }
        }

        // Mettre à jour la fréquence du LFO si l'activation change
        if self.lfo_activation {
            self.update_synth_lfo();
        }
    }

    /// Met à jour la forme d'onde du LFO
    fn update_synth_lfo_waveform(&mut self) {
        // Mettre à jour la forme d'onde dans le synthétiseur local
        self.current_synth_type
            .set_current_lfo_waveform(self.waveform);

        // Mettre à jour aussi le synthétiseur dans le contrôleur audio
        if let Some(ref synth_control) = self.synth_control {
            if let Ok(mut synth) = synth_control.lock() {
                synth.set_current_lfo_waveform(self.waveform);
                println!(
                    "Forme d'onde LFO mise à jour dans le contrôleur audio: {:?}",
                    self.waveform
                );
            }
        }
    }

    /// Met à jour l'octave globale
    fn update_global_octave(&self) {
        // Convertir l'octave (1-9) en index (0-8) pour CURRENT_OCTAVE_INDEX
        let octave_index = (self.current_octave - 1).min(8);
        constants::CURRENT_OCTAVE_INDEX.store(octave_index, Ordering::Relaxed);
    }

    fn play_note(&mut self, note_name: &str) {
        // Créer une clé unique pour la note basée sur la note + octave
        let note_key = format!("{}_{}", note_name, self.current_octave);

        // Si la note n'est pas déjà active, l'ajouter
        if !self.active_notes.contains(&note_key) {
            self.active_notes.insert(note_key.clone());

            if let Some(ref notes) = self.notes {
                let frequency = self.note_to_frequency(note_name);
                self.add_note(notes, frequency);
                println!("Note démarrée: {} ({})", note_name, note_key);
            }
        } else {
            println!("Note déjà active: {} ({})", note_name, note_key);
        }
    }

    fn stop_note(&mut self, note_name: &str) {
        // Créer la même clé unique pour la note
        let note_key = format!("{}_{}", note_name, self.current_octave);

        // Vérifier que ni le clavier physique ni virtuel ne jouent cette note
        let physical_key = format!("physical_{}", note_name);
        let virtual_key = note_name.to_string();

        let still_pressed_physical = self.pressed_physical_keys.contains(&physical_key);
        let still_pressed_virtual = self.pressed_notes.contains(&virtual_key);

        // Si aucun des deux claviers ne presse la note, l'arrêter
        if !still_pressed_physical && !still_pressed_virtual {
            if self.active_notes.remove(&note_key) {
                if let Some(ref notes) = self.notes {
                    let frequency = self.note_to_frequency(note_name);
                    self.remove_note(notes, frequency);
                    println!("Note arrêtée: {} ({})", note_name, note_key);
                }
            }
        } else {
            println!(
                "Note maintenue par {} clavier(s): {} ({})",
                if still_pressed_physical && still_pressed_virtual {
                    "les deux"
                } else if still_pressed_physical {
                    "physique"
                } else {
                    "virtuel"
                },
                note_name,
                note_key
            );
        }
    }

    /// Convertit un nom de note en fréquence en utilisant le système JSON
    fn note_to_frequency(&self, note_name: &str) -> f64 {
        // Convertir le nom de note au format JSON
        let json_note = match note_name {
            "C" => "C",
            "C#" => "CSHARP",
            "D" => "D",
            "D#" => "DSHARP",
            "E" => "E",
            "F" => "F",
            "F#" => "FSHARP",
            "G" => "G",
            "G#" => "GSHARP",
            "A" => "A",
            "A#" => "ASHARP",
            "B" => "B",
            _ => "A",
        };

        // Utiliser l'octave actuelle et chercher dans le JSON
        let octave = self.current_octave as u8;

        // Chercher la fréquence dans le système JSON
        if let Some(octave_notes) = NOTES.0.get(&octave) {
            if let Some(&frequency) = octave_notes.get(json_note) {
                println!(
                    "Note trouvée: {} octave {} = {:.2} Hz",
                    note_name, octave, frequency
                );
                return frequency;
            }
        }

        println!(
            "Note non trouvée: {} octave {}, retour A4",
            note_name, octave
        );
        440.0 // If not found, return A4
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
