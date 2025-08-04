use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, StreamConfig};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use fundsp::hacker::{
    Wave64, hammond_hz, multipass, reverb_stereo, sine, sine_hz, soft_saw_hz, square_hz, wave64,
};
use fundsp::prelude::AudioUnit64;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
mod consts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_frequencies_for_range(0, 8);

    // Activer le mode raw du terminal
    enable_raw_mode()?;

    // Utilise un HashMap pour stocker les fréquences avec leur timestamp
    // On utilise les bits de f64 comme clé pour éviter les problèmes de Hash/Eq
    let active_frequencies = Arc::new(Mutex::new(HashMap::<u64, (f64, Instant)>::new()));

    // Clone pour le thread audio
    let frequencies_clone = Arc::clone(&active_frequencies);

    // This function starts the thread that creates the audio and sends
    // it to CPAL so that we can hear it.
    run_output_polyphonic_with_timeout(frequencies_clone);

    println!("Audio lancé ! Appuyez sur les touches pour jouer les notes :");
    println!("Les notes s'arrêtent automatiquement après 0.5 seconde");
    println!("Maintenez une touche enfoncée pour continuer à jouer la note");
    println!();
    println!("a - A 440Hz         1 - A# 466.16Hz");
    println!("b - B 493.88Hz      2 - C# 554.37Hz");
    println!("c - C 523.25Hz      3 - D# 622.25Hz");
    println!("d - D 587.33Hz      4 - F# 739.99Hz");
    println!("e - E 659.26Hz      5 - G# 830.61Hz");
    println!("f - F 698.46Hz");
    println!("g - G 783.99Hz");
    println!();
    println!("ESPACE - Arrêter toutes les notes");
    println!("ESC ou Ctrl+C - Quitter");

    loop {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            println!("\rAu revoir !");
                            break;
                        }
                        KeyCode::Char('a') => {
                            add_frequency_with_timeout(&active_frequencies, consts::A4)
                        }
                        KeyCode::Char('b') => {
                            add_frequency_with_timeout(&active_frequencies, consts::B4)
                        }
                        KeyCode::Char('c') => {
                            add_frequency_with_timeout(&active_frequencies, consts::C5)
                        }
                        KeyCode::Char('d') => {
                            add_frequency_with_timeout(&active_frequencies, consts::D5)
                        }
                        KeyCode::Char('e') => {
                            add_frequency_with_timeout(&active_frequencies, consts::E5)
                        }
                        KeyCode::Char('f') => {
                            add_frequency_with_timeout(&active_frequencies, consts::F5)
                        }
                        KeyCode::Char('g') => {
                            add_frequency_with_timeout(&active_frequencies, consts::G5)
                        }
                        KeyCode::Char('1') => {
                            add_frequency_with_timeout(&active_frequencies, consts::ASharp4)
                        }
                        KeyCode::Char('2') => {
                            add_frequency_with_timeout(&active_frequencies, consts::CSharp5)
                        }
                        KeyCode::Char('3') => {
                            add_frequency_with_timeout(&active_frequencies, consts::DSharp5)
                        }
                        KeyCode::Char('4') => {
                            add_frequency_with_timeout(&active_frequencies, consts::FSharp5)
                        }
                        KeyCode::Char('5') => {
                            add_frequency_with_timeout(&active_frequencies, consts::GSharp5)
                        }
                        KeyCode::Char(' ') => {
                            stop_all_frequencies_with_timeout(&active_frequencies)
                        }
                        KeyCode::Esc => {
                            println!("\rAu revoir !");
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Désactiver le mode raw avant de sortir
    disable_raw_mode()?;
    Ok(())
}

/// Ajoute une fréquence avec un timestamp pour le timeout automatique
fn add_frequency_with_timeout(frequencies: &Arc<Mutex<HashMap<u64, (f64, Instant)>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    let freq_key = freq.to_bits();
    let is_new = !freqs.contains_key(&freq_key);
    
    // Toujours mettre à jour le timestamp, même si la note existe déjà
    freqs.insert(freq_key, (freq, Instant::now()));
    
    if is_new {
        println!(
            "\rNote ajoutée: {:.2} Hz. Notes actives: {}",
            freq,
            freqs.len()
        );
    }
    // Si la note existait déjà, on ne fait que rafraîchir silencieusement
}

/// Arrête toutes les fréquences avec timeout
fn stop_all_frequencies_with_timeout(frequencies: &Arc<Mutex<HashMap<u64, (f64, Instant)>>>) {
    let mut freqs = frequencies.lock().unwrap();
    freqs.clear();
    println!("\rToutes les notes arrêtées.");
}

/// Version polyphonique avec timeout automatique
fn run_output_polyphonic_with_timeout(frequencies: Arc<Mutex<HashMap<u64, (f64, Instant)>>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => {
            run_synth_polyphonic_with_timeout::<f32>(frequencies, device, config.into())
        }
        SampleFormat::I16 => {
            run_synth_polyphonic_with_timeout::<i16>(frequencies, device, config.into())
        }
        SampleFormat::U16 => {
            run_synth_polyphonic_with_timeout::<u16>(frequencies, device, config.into())
        }

        _ => panic!("Unsupported format"),
    }
}

/// Version polyphonique avec timeout qui génère plusieurs oscillateurs
fn run_synth_polyphonic_with_timeout<T: SizedSample + FromSample<f64>>(
    frequencies: Arc<Mutex<HashMap<u64, (f64, Instant)>>>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");

        // Variables pour les oscillateurs
        let mut phases: HashMap<u64, f64> = HashMap::new();

        let frequencies_clone = Arc::clone(&frequencies);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_polyphonic_with_timeout(
                        data,
                        channels,
                        &frequencies_clone,
                        &mut phases,
                        sample_rate,
                    )
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

/// Génère les échantillons audio polyphoniques avec timeout automatique
fn write_data_polyphonic_with_timeout<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    frequencies: &Arc<Mutex<HashMap<u64, (f64, Instant)>>>,
    phases: &mut HashMap<u64, f64>,
    sample_rate: f64,
) {
    // Nettoyer les fréquences expirées et obtenir les actives
    let active_freqs = {
        let mut freqs = frequencies.lock().unwrap();
        let now = Instant::now();
        let timeout = std::time::Duration::from_millis(500); // 0.5 seconde de timeout

        // Retirer les fréquences expirées
        freqs.retain(|_freq_key, (_freq, timestamp)| now.duration_since(*timestamp) < timeout);

        freqs
            .iter()
            .map(|(key, (freq, _))| (*key, *freq))
            .collect::<Vec<(u64, f64)>>()
    };

    for frame in output.chunks_mut(channels) {
        let mut sample_left = 0.0;
        let mut sample_right = 0.0;

        // Générer et sommer tous les oscillateurs actifs
        for &(freq_key, freq) in &active_freqs {
            let phase = phases.entry(freq_key).or_insert(0.0);
            let phase_increment = 2.0 * std::f64::consts::PI * freq / sample_rate;
            let sine_value = phase.sin();

            // Réduire l'amplitude quand il y a plusieurs notes (éviter la saturation)
            let amplitude = if active_freqs.len() > 1 {
                0.3 / active_freqs.len() as f64
            } else {
                0.3
            };

            sample_left += sine_value * amplitude;
            sample_right += sine_value * amplitude;

            *phase += phase_increment;
            if *phase > 2.0 * std::f64::consts::PI {
                *phase -= 2.0 * std::f64::consts::PI;
            }
        }

        // Nettoyer les phases des fréquences qui ne sont plus actives
        let active_keys: std::collections::HashSet<u64> =
            active_freqs.iter().map(|(k, _)| *k).collect();
        phases.retain(|k, _| active_keys.contains(k));

        let left: T = T::from_sample(sample_left);
        let right: T = T::from_sample(sample_right);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

/// Ajoute une fréquence à la liste des fréquences actives
fn add_frequency(frequencies: &Arc<Mutex<Vec<f64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    if !freqs.contains(&freq) {
        freqs.push(freq);
        println!(
            "\rNote ajoutée: {:.2} Hz. Notes actives: {}",
            freq,
            freqs.len()
        );
    }
}

/// Retire une fréquence de la liste des fréquences actives
fn remove_frequency(frequencies: &Arc<Mutex<Vec<f64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    freqs.retain(|&x| x != freq);
    println!(
        "\rNote retirée: {:.2} Hz. Notes actives: {}",
        freq,
        freqs.len()
    );
}

/// Toggle une fréquence (ajoute si absente, retire si présente)
fn toggle_frequency(frequencies: &Arc<Mutex<Vec<f64>>>, freq: f64) {
    let mut freqs = frequencies.lock().unwrap();
    if let Some(pos) = freqs.iter().position(|&x| x == freq) {
        freqs.remove(pos);
        println!(
            "\rNote arrêtée: {:.2} Hz. Notes actives: {}",
            freq,
            freqs.len()
        );
    } else {
        freqs.push(freq);
        println!(
            "\rNote démarrée: {:.2} Hz. Notes actives: {}",
            freq,
            freqs.len()
        );
    }
}

/// Arrête toutes les fréquences
fn stop_all_frequencies(frequencies: &Arc<Mutex<Vec<f64>>>) {
    let mut freqs = frequencies.lock().unwrap();
    freqs.clear();
    println!("\rToutes les notes arrêtées.");
}

/// Version polyphonique qui utilise un vecteur de fréquences
fn run_output_polyphonic(frequencies: Arc<Mutex<Vec<f64>>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth_polyphonic::<f32>(frequencies, device, config.into()),
        SampleFormat::I16 => run_synth_polyphonic::<i16>(frequencies, device, config.into()),
        SampleFormat::U16 => run_synth_polyphonic::<u16>(frequencies, device, config.into()),

        _ => panic!("Unsupported format"),
    }
}

/// Version polyphonique qui génère plusieurs oscillateurs
fn run_synth_polyphonic<T: SizedSample + FromSample<f64>>(
    frequencies: Arc<Mutex<Vec<f64>>>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");

        // Variables pour les oscillateurs
        let mut phases: Vec<f64> = Vec::new();
        let mut current_freqs: Vec<f64> = Vec::new();

        let frequencies_clone = Arc::clone(&frequencies);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_polyphonic(
                        data,
                        channels,
                        &frequencies_clone,
                        &mut phases,
                        &mut current_freqs,
                        sample_rate,
                    )
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

/// Génère les échantillons audio polyphoniques en temps réel
fn write_data_polyphonic<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    frequencies: &Arc<Mutex<Vec<f64>>>,
    phases: &mut Vec<f64>,
    current_freqs: &mut Vec<f64>,
    sample_rate: f64,
) {
    // Synchroniser les oscillateurs avec les fréquences actives
    let active_freqs = {
        let freqs = frequencies.lock().unwrap();
        freqs.clone()
    };

    // Mettre à jour les oscillateurs si les fréquences ont changé
    if *current_freqs != active_freqs {
        *current_freqs = active_freqs.clone();
        phases.clear();
        phases.resize(active_freqs.len(), 0.0);
    }

    for frame in output.chunks_mut(channels) {
        let mut sample_left = 0.0;
        let mut sample_right = 0.0;

        // Générer et sommer tous les oscillateurs actifs
        for (i, &freq) in current_freqs.iter().enumerate() {
            if i < phases.len() {
                let phase_increment = 2.0 * std::f64::consts::PI * freq / sample_rate;
                let sine_value = phases[i].sin();

                // Réduire l'amplitude quand il y a plusieurs notes (éviter la saturation)
                let amplitude = if current_freqs.len() > 1 {
                    0.3 / current_freqs.len() as f64
                } else {
                    0.3
                };

                sample_left += sine_value * amplitude;
                sample_right += sine_value * amplitude;

                phases[i] += phase_increment;
                if phases[i] > 2.0 * std::f64::consts::PI {
                    phases[i] -= 2.0 * std::f64::consts::PI;
                }
            }
        }

        let left: T = T::from_sample(sample_left);
        let right: T = T::from_sample(sample_right);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

/// Version partagée qui utilise Arc<Mutex<>> pour permettre le changement dynamique
fn run_output_shared(audio_graph: Arc<Mutex<Box<dyn AudioUnit64>>>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth_shared::<f32>(audio_graph, device, config.into()),
        SampleFormat::I16 => run_synth_shared::<i16>(audio_graph, device, config.into()),
        SampleFormat::U16 => run_synth_shared::<u16>(audio_graph, device, config.into()),

        _ => panic!("Unsupported format"),
    }
}

/// Version partagée qui utilise Arc<Mutex<>> pour permettre le changement dynamique
fn run_synth_shared<T: SizedSample + FromSample<f64>>(
    audio_graph: Arc<Mutex<Box<dyn AudioUnit64>>>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;

        // Initialiser le sample rate
        {
            let mut graph = audio_graph.lock().unwrap();
            graph.set_sample_rate(sample_rate);
        }

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");

        // Clone l'Arc pour l'utiliser dans la closure
        let audio_graph_clone = Arc::clone(&audio_graph);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data_shared(data, channels, &audio_graph_clone)
                },
                err_fn,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

/// Version partagée qui utilise Arc<Mutex<>> pour accéder à l'audio graph
fn write_data_shared<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    audio_graph: &Arc<Mutex<Box<dyn AudioUnit64>>>,
) {
    for frame in output.chunks_mut(channels) {
        let sample = {
            let mut graph = audio_graph.lock().unwrap();
            graph.get_stereo()
        };
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

// ------------------------------------------------------------------
// You can use any of the functions in this section to make the audio
// graph. Just replace the function call in `main` at the top.

/// Simple sine wave at 440 Hz which is standard tuning for A4
fn create_sine(freq: f64) -> Box<dyn AudioUnit64> {
    let synth = sine_hz(freq);

    Box::new(synth)
}

/// C major chord created by summing waves! Sine by default, but try uncommenting
/// the other wave types.
fn create_c_major() -> Box<dyn AudioUnit64> {
    let synth = sine_hz(consts::C4) + sine_hz(consts::E4) + sine_hz(consts::G4);
    //let synth = square_hz(consts::C4) + square_hz(consts::E4) + square_hz(consts::G4);
    // let synth = soft_saw_hz(consts::C4) + soft_saw_hz(consts::E4) + soft_saw_hz(consts::G4);
    // let synth = hammond_hz(consts::C4) + hammond_hz(consts::E4) + hammond_hz(consts::G4);

    Box::new(synth)
}

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
