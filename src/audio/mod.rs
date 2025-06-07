pub mod generator;
pub mod synth;
pub mod player;

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
    };

    pub type Stream = Vec<f32>;
}