use crate::audio::{note_manager, setup_realtime_audio};
use crate::consts::constants::PROJECT_NAME;
use crate::gui::SynthesizerApp;
use crate::input::key_logic;
use crate::{prints, synths};
use device_query::DeviceState;
use display_info::DisplayInfo;
use std::collections::HashSet;
use std::ptr;
use std::sync::{Arc, Mutex};

/// Launch the terminal application
pub fn launch_terminal_application() -> Result<(), Box<dyn std::error::Error>> {
    let note_manager = note_manager::create_note_manager();
    let current_synth_type: Arc<Mutex<synths::manager::SynthType>> =
        Arc::new(Mutex::new(synths::manager::SynthType::n_sine()));

    // Clone for the audio thread
    let notes_clone = Arc::clone(&note_manager);
    let synth_type_clone = Arc::clone(&current_synth_type);

    // Run the audio output in a separate thread
    setup_realtime_audio::run_output_polyphonic_realtime(notes_clone, synth_type_clone);

    prints::printfn::print_intro();

    let device_state = DeviceState::new();
    let mut previous_keys = HashSet::new();

    loop {
        key_logic::key_management(
            &device_state,
            &mut previous_keys,
            &current_synth_type,
            &note_manager,
        );
    }
}

/// Launch the GUI application
pub fn launch_gui_application() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // Initialiser le système audio comme dans main.rs
    let note_manager = note_manager::create_note_manager();
    let current_synth_type: Arc<Mutex<synths::manager::SynthType>> =
        Arc::new(Mutex::new(synths::manager::SynthType::n_sine()));

    // Lancer l'audio en arrière-plan
    setup_realtime_audio::run_output_polyphonic_realtime(
        Arc::clone(&note_manager),
        Arc::clone(&current_synth_type),
    );

    // Détection multiplateforme de la taille d'écran principale
    let display = DisplayInfo::all()
        .unwrap()
        .into_iter()
        .find(|d| d.is_primary)
        .unwrap_or(DisplayInfo {
            id: 0,
            name: String::new(),
            friendly_name: String::new(),
            width: 1200,
            height: 800,
            width_mm: 0,
            height_mm: 0,
            frequency: 60.0,
            rotation: 0.0,
            scale_factor: 1.0,
            is_primary: true,
            raw_handle: unsafe { std::mem::zeroed() },
            x: 0,
            y: 0,
        });
    let width = display.width as f32 * 0.8;
    let height = display.height as f32 * 0.8;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([width, height])
            .with_min_inner_size([width, height])
            .with_title(PROJECT_NAME),
        ..Default::default()
    };

    eframe::run_native(
        PROJECT_NAME,
        options,
        Box::new(move |cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(32.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(22.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(20.0, egui::FontFamily::Monospace),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(22.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            cc.egui_ctx.set_style(style);

            Ok(Box::new(
                SynthesizerApp::new(cc)
                    .with_audio(note_manager)
                    .with_synth_control(current_synth_type),
            ))
        }),
    )?;
    Ok(())
}
