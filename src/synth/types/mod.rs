pub mod fm;
pub mod hammond;
pub mod sawtooth;
pub mod square;
pub mod sine;

use crate::synth::traits::Synthesizer;

pub use self::fm::FMSynth;
pub use self::hammond::HammondSynth;
pub use self::sawtooth::SawtoothSynth;
pub use self::square::SquareSynth;
pub use self::sine::SineSynth;