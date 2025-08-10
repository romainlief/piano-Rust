use synthesizer_emulation::launcher::app_launcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_launcher::launch_application()
}
