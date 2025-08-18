use std::sync::{Arc, Mutex};
use synthesizer_emulation::audio::{note_manager, setup_realtime_audio};
use synthesizer_emulation::consts::constants::PROJECT_NAME;
use synthesizer_emulation::gui::SynthesizerApp;
use synthesizer_emulation::synths;

fn main() -> eframe::Result<()> {
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
            .with_inner_size([1350.0, 1150.0]) 
            .with_min_inner_size([400.0, 300.0])
            .with_title(PROJECT_NAME),
        ..Default::default()
    };

    eframe::run_native(
        PROJECT_NAME,
        options,
        Box::new(move |cc| {
            // 🎨 Modifier le style global AVANT de créer l'app
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
