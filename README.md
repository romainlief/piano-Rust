# 🎹 Synthesizer in Rust

This project is a synthesizer in Rust made with cpal, device_query, once_cell, serde, serde_json and rand

## Features

- **Polyphonic Piano**: Play multiple notes simultaneously
- **Multiple Synthesizers**: 5 different types of synthesizers with their own characteristics
- **Real-time**: Instant response to keyboard keys
- **Advanced Control**: Customize parameters for each synthesizer
- **High-quality audio**: Uses CPAL for professional audio reproduction

## Synthétiseurs Disponibles

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

```bash
cargo run --release
```

or

```bash
cargo run
```

## Dependencies

- **[cpal](https://crates.io/crates/cpal)** `0.16.0` - Audio cross-platform
- **[device_query](https://crates.io/crates/device_query)** `4.0.1` - Note management in reql time
- **[once_cell](https://crates.io/crates/once_cell)** `1.18.0` - For the JSON note file
- **[serde](https://serde.rs/)** `1.0` - For the JSON note file
- **[serde_json](https://crates.io/crates/serde_json/1.0.1/dependencies)** `1.0` - For the JSON note file
- **[rand](https://crates.io/crates/rand)** `0.9.2` - To generate random numbers
