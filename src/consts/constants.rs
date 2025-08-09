pub const C0: f64 = 16.35;
pub const CSHARP0: f64 = 17.32;
pub const D0: f64 = 18.35;
pub const DSHARP0: f64 = 19.45;
pub const E0: f64 = 20.60;
pub const F0: f64 = 21.83;
pub const FSHARP0: f64 = 23.12;
pub const G0: f64 = 24.50;
pub const GSHARP0: f64 = 25.96;
pub const A0: f64 = 27.50;
pub const ASHARP0: f64 = 29.14;
pub const B0: f64 = 30.87;
pub const C1: f64 = 32.70;
pub const CSHARP1: f64 = 34.65;
pub const D1: f64 = 36.71;
pub const DSHARP1: f64 = 38.89;
pub const E1: f64 = 41.20;
pub const F1: f64 = 43.65;
pub const FSHARP1: f64 = 46.25;
pub const G1: f64 = 49.00;
pub const GSHARP1: f64 = 51.91;
pub const A1: f64 = 55.00;
pub const ASHARP1: f64 = 58.27;
pub const B1: f64 = 61.74;
pub const C2: f64 = 65.41;
pub const CSHARP2: f64 = 69.30;
pub const D2: f64 = 73.42;
pub const DSHARP2: f64 = 77.78;
pub const E2: f64 = 82.41;
pub const F2: f64 = 87.31;
pub const FSHARP2: f64 = 92.50;
pub const G2: f64 = 98.00;
pub const GSHARP2: f64 = 103.83;
pub const A2: f64 = 110.00;
pub const ASHARP2: f64 = 116.54;
pub const B2: f64 = 123.47;
pub const C3: f64 = 130.81;
pub const CSHARP3: f64 = 138.59;
pub const D3: f64 = 146.83;
pub const DSHARP3: f64 = 155.56;
pub const E3: f64 = 164.81;
pub const F3: f64 = 174.61;
pub const FSHARP3: f64 = 185.00;
pub const G3: f64 = 196.00;
pub const GSHARP3: f64 = 207.65;
pub const A3: f64 = 220.00;
pub const ASHARP3: f64 = 233.08;
pub const B3: f64 = 246.94;
pub const C4: f64 = 261.63;
pub const CSHARP4: f64 = 277.18;
pub const D4: f64 = 293.66;
pub const DSHARP4: f64 = 311.13;
pub const E4: f64 = 329.63;
pub const F4: f64 = 349.23;
pub const FSHARP4: f64 = 369.99;
pub const G4: f64 = 392.00;
pub const GSHARP4: f64 = 415.30;
pub const A4: f64 = 440.00;
pub const ASHARP4: f64 = 466.16;
pub const B4: f64 = 493.88;
pub const C5: f64 = 523.25;
pub const CSHARP5: f64 = 554.37;
pub const D5: f64 = 587.33;
pub const DSHARP5: f64 = 622.25;
pub const E5: f64 = 659.26;
pub const F5: f64 = 698.46;
pub const FSHARP5: f64 = 739.99;
pub const G5: f64 = 783.99;
pub const GSHARP5: f64 = 830.61;
pub const A5: f64 = 880.00;
pub const ASHARP5: f64 = 932.33;
pub const B5: f64 = 987.77;
pub const C6: f64 = 1046.50;
pub const CSHARP6: f64 = 1108.73;
pub const D6: f64 = 1174.66;
pub const DSHARP6: f64 = 1244.51;
pub const E6: f64 = 1318.51;
pub const F6: f64 = 1396.91;
pub const FSHARP6: f64 = 1479.98;
pub const G6: f64 = 1567.98;
pub const GSHARP6: f64 = 1661.22;
pub const A6: f64 = 1760.00;
pub const ASHARP6: f64 = 1864.66;
pub const B6: f64 = 1975.53;
pub const C7: f64 = 2093.00;
pub const CSHARP7: f64 = 2217.46;
pub const D7: f64 = 2349.32;
pub const DSHARP7: f64 = 2489.02;
pub const E7: f64 = 2637.02;
pub const F7: f64 = 2793.83;
pub const FSHARP7: f64 = 2959.96;
pub const G7: f64 = 3135.96;
pub const GSHARP7: f64 = 3322.44;
pub const A7: f64 = 3520.00;
pub const ASHARP7: f64 = 3729.31;
pub const B7: f64 = 3951.07;
pub const C8: f64 = 4186.01;
pub const CSHARP8: f64 = 4434.92;
pub const D8: f64 = 4698.64;
pub const DSHARP8: f64 = 4978.03;
pub const E8: f64 = 5274.04;
pub const F8: f64 = 5587.65;
pub const FSHARP8: f64 = 5919.91;
pub const G8: f64 = 6271.93;
pub const GSHARP8: f64 = 6644.88;
pub const A8: f64 = 7040.00;
pub const ASHARP8: f64 = 7458.62;
pub const B8: f64 = 7902.13;

fn note_frequency(semitone_distance_from_a4: i32) -> f64 {
    // A4 = 440 Hz, 12-TET formula
    440.0 * 2f64.powf(semitone_distance_from_a4 as f64 / 12.0)
}

fn get_chromatic_scale() -> Vec<&'static str> {
    vec![
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ]
}

fn print_frequencies_for_range(start_octave: i32, end_octave: i32) {
    let scale = get_chromatic_scale();
    for octave in start_octave..=end_octave {
        for (i, note) in scale.iter().enumerate() {
            let semitone_distance = (octave - 4) * 12 + (i as i32 - 9);
            let freq = note_frequency(semitone_distance);
            println!("{}{}: {:.2} Hz", note, octave, freq);
        }
    }
}
