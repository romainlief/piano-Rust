// Copyright (c) 2025 Romain Lief
// Licensed under the MIT License

use synthesizer_emulation::launcher::app_launcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_launcher::launch_terminal_application()
}
