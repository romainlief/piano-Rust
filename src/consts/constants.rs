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
