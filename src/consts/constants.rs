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

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static NOTES: Lazy<HashMap<u8, HashMap<&'static str, f64>>> = Lazy::new(|| {
    let mut notes = HashMap::new();

    let mut octave0 = HashMap::new();
    octave0.insert("C0", 16.35);
    octave0.insert("CSHARP0", 17.32);
    octave0.insert("D0", 18.35);
    octave0.insert("DSHARP0", 19.45);
    octave0.insert("E0", 20.60);
    octave0.insert("F0", 21.83);
    octave0.insert("FSHARP0", 23.12);
    octave0.insert("G0", 24.50);
    octave0.insert("GSHARP0", 25.96);
    octave0.insert("A0", 27.50);
    octave0.insert("ASHARP0", 29.14);
    octave0.insert("B0", 30.87);
    notes.insert(1, octave0);

    let mut octave1 = HashMap::new();
    octave1.insert("C1", 32.70);
    octave1.insert("CSHARP1", 34.65);
    octave1.insert("D1", 36.71);
    octave1.insert("DSHARP1", 38.89);
    octave1.insert("E1", 41.20);
    octave1.insert("F1", 43.65);
    octave1.insert("FSHARP1", 46.25);
    octave1.insert("G1", 49.00);
    octave1.insert("GSHARP1", 51.91);
    octave1.insert("A1", 55.00);
    octave1.insert("ASHARP1", 58.27);
    octave1.insert("B1", 61.74);
    notes.insert(2, octave1);

    let mut octave2 = HashMap::new();
    octave2.insert("C2", 65.41);
    octave2.insert("CSHARP2", 69.30);
    octave2.insert("D2", 73.42);
    octave2.insert("DSHARP2", 77.78);
    octave2.insert("E2", 82.41);
    octave2.insert("F2", 87.31);
    octave2.insert("FSHARP2", 92.50);
    octave2.insert("G2", 98.00);
    octave2.insert("GSHARP2", 103.83);
    octave2.insert("A2", 110.00);
    octave2.insert("ASHARP2", 116.54);
    octave2.insert("B2", 123.47);
    notes.insert(2, octave2);

    let mut octave3 = HashMap::new();
    octave3.insert("C3", 130.81);
    octave3.insert("CSHARP3", 138.59);
    octave3.insert("D3", 146.83);
    octave3.insert("DSHARP3", 155.56);
    octave3.insert("E3", 164.81);
    octave3.insert("F3", 174.61);
    octave3.insert("FSHARP3", 185.00);
    octave3.insert("G3", 196.00);
    octave3.insert("GSHARP3", 207.65);
    octave3.insert("A3", 220.00);
    octave3.insert("ASHARP3", 233.08);
    octave3.insert("B3", 246.94);
    notes.insert(3, octave3);

    let mut octave4 = HashMap::new();
    octave4.insert("C4", 261.63);
    octave4.insert("CSHARP4", 277.18);
    octave4.insert("D4", 293.66);
    octave4.insert("DSHARP4", 311.13);
    octave4.insert("E4", 329.63);
    octave4.insert("F4", 349.23);
    octave4.insert("FSHARP4", 369.99);
    octave4.insert("G4", 392.00);
    octave4.insert("GSHARP4", 415.30);
    octave4.insert("A4", 440.00);
    octave4.insert("ASHARP4", 466.16);
    octave4.insert("B4", 493.88);
    notes.insert(4, octave4);

    let mut octave5 = HashMap::new();
    octave5.insert("C5", 523.25);
    octave5.insert("CSHARP5", 554.37);
    octave5.insert("D5", 587.33);
    octave5.insert("DSHARP5", 622.25);
    octave5.insert("E5", 659.26);
    octave5.insert("F5", 698.46);
    octave5.insert("FSHARP5", 739.99);
    octave5.insert("G5", 783.99);
    octave5.insert("GSHARP5", 830.61);
    octave5.insert("A5", 880.00);
    octave5.insert("ASHARP5", 932.33);
    octave5.insert("B5", 987.77);
    notes.insert(5, octave5);

    let mut octave6 = HashMap::new();
    octave6.insert("C6", 1046.50);
    octave6.insert("CSHARP6", 1108.73);
    octave6.insert("D6", 1174.66);
    octave6.insert("DSHARP6", 1244.51);
    octave6.insert("E6", 1318.51);
    octave6.insert("F6", 1396.91);
    octave6.insert("FSHARP6", 1479.98);
    octave6.insert("G6", 1567.98);
    octave6.insert("GSHARP6", 1661.22);
    octave6.insert("A6", 1760.00);
    octave6.insert("ASHARP6", 1864.66);
    octave6.insert("B6", 1975.53);
    notes.insert(6, octave6);

    let mut octave7 = HashMap::new();
    octave7.insert("C7", 2093.00);
    octave7.insert("CSHARP7", 2217.46);
    octave7.insert("D7", 2349.32);
    octave7.insert("DSHARP7", 2489.02);
    octave7.insert("E7", 2637.02);
    octave7.insert("F7", 2793.83);
    octave7.insert("FSHARP7", 2959.96);
    octave7.insert("G7", 3135.96);
    octave7.insert("GSHARP7", 3322.44);
    octave7.insert("A7", 3520.00);
    octave7.insert("ASHARP7", 3729.31);
    octave7.insert("B7", 3951.07);
    notes.insert(7, octave7);

    let mut octave8 = HashMap::new();
    octave8.insert("C8", 4186.01);
    octave8.insert("CSHARP8", 4434.92);
    octave8.insert("D8", 4698.64);
    octave8.insert("DSHARP8", 4978.03);
    octave8.insert("E8", 5274.04);
    octave8.insert("F8", 5587.65);
    octave8.insert("FSHARP8", 5919.91);
    octave8.insert("G8", 6271.93);
    octave8.insert("GSHARP8", 6644.88);
    octave8.insert("A8", 7040.00);
    octave8.insert("ASHARP8", 7458.62);
    octave8.insert("B8", 7902.13);
    notes.insert(8, octave8);

    notes
});

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
