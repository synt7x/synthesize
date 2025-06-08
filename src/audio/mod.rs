pub mod generator;
pub mod synth;
pub mod player;
pub mod filter;

pub mod prelude {
    pub use sdl3::audio::{
        AudioSpec, AudioFormat, AudioCallback,
        AudioStream, AudioStreamWithCallback
    };

    pub use sdl3::AudioSubsystem;

    pub use crate::audio::{
        generator::*,
        synth::*,
        player::*,
        filter::*,
    };

    pub type Stream = Vec<f32>;
}