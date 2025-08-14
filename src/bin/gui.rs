use std::sync::{Arc, Mutex};
use synthesizer_emulation::audio::{note_manager, setup_realtime_audio};
use synthesizer_emulation::gui::SynthesizerApp;
use synthesizer_emulation::synths;
use synthesizer_emulation::synths::manager::SynthType;

fn main() -> Result<(), eframe::Error> {
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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 1000.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Synthétiseur Rust - GUI avec Audio"),
        ..Default::default()
    };

    eframe::run_native(
        "Synthétiseur Rust",
        options,
        Box::new(move |cc| {
            Ok(Box::new(
                SynthesizerApp::new(cc)
                    .with_audio(note_manager)
                    .with_synth_control(current_synth_type),
            ))
        }),
    )
}
