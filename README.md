# 🎹 Synthesizer in Rust

This project is a synthesizer in Rust made with cpal and device_query.

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
| **Square 50%**      |   `X`   | Classic square wave                                     |
| **Sawtooth**        |   `S`   | Sawtooth wave, rich in harmonics                        |
| **Hammond**         |   `N`   | Hammond organ simulation with controlled harmonics      |
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

## Dependencies

- **[cpal](https://crates.io/crates/cpal)** `0.16.0` - Audio cross-platform
- **[device_query](https://crates.io/crates/device_query)** `4.0.1` - Détection des touches en temps réel
