# 🎹 Synthesizer in Rust

This project is a synthesizer in Rust made with cpal, device_query, once_cell, serde, serde_json and rand

## Features

- **Polyphonic Piano**: Play multiple notes simultaneously
- **Multiple Synthesizers**: 5 different types of synthesizers with their own characteristics
- **Real-time**: Instant response to keyboard keys
- **Advanced Control**: Customize parameters for each synthesizer
- **High-quality audio**: Uses CPAL for professional audio reproduction

## Available Synthesizers

| Type                | Touche  | Description                                             |
|---------------------|---------|---------------------------------------------------------|
| **Sine**            |   `W`   | Pure and clear sine wave                                |
| **Square**          |   `X`   | Classic square wave                                     |
| **Sawtooth**        |   `S`   | Sawtooth wave, rich in harmonics                        |
| **Hammond**         |   `H`   | Hammond organ simulation with controlled harmonics      |
| **FM**              |   `K`   | Soft Frequency Modulation Synthesis                     |

## 🛠️ Installation and launch of the program

### Requirements

- Rust 1.70+ (recommended)
- Cargo
- Supported audio system (ALSA on Linux, CoreAudio on macOS, WASAPI on Windows)

### Launch the application

#### Linux and Mac

To run the code in the terminal:
```bash
cargo run --bin synthesizer_emulation
```

To run the GUI:
```bash
cargo run --bin gui
```

## Dependencies

- **[cpal](https://crates.io/crates/cpal)** `0.16.0` - Audio cross-platform
- **[device_query](https://crates.io/crates/device_query)** `4.0.1` - Note management in reql time
- **[once_cell](https://crates.io/crates/once_cell)** `1.18.0` - For the JSON note file
- **[serde](https://serde.rs/)** `1.0` - For the JSON note file
- **[serde_json](https://crates.io/crates/serde_json/1.0.1/dependencies)** `1.0` - For the JSON note file
- **[rand](https://crates.io/crates/rand)** `0.9.2` - To generate random numbers
- **[eframe](https://crates.io/crates/eframe)** `0.32.1` - For the GUI
- **[egui](https://crates.io/crates/egui)** `0.32.1` - For the GUI
- **[egui_knob](https://crates.io/crates/egui_knob)** `0.3.3` - For the knob widget
- **[env_logger](https://crates.io/crates/env_logger)** `0.11.8` - For logging
- **[display-info](https://crates.io/crates/display-info)** `0.5.5` - For the screen size

## 📄 License

Copyright (c) 2025 Romain Lief

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📞 Contact

- Author: Romain Lief
- Repository: [piano-Rust](https://github.com/romainlief/piano-Rust)
