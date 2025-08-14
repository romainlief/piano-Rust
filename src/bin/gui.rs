use synthesizer_emulation::gui::SynthesizerApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Synthétiseur Rust - GUI"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Synthétiseur Rust",
        options,
        Box::new(|cc| Ok(Box::new(SynthesizerApp::new(cc)))),
    )
}
