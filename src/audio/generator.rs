use crate::audio::prelude::*;

pub struct Generator {
    device: AudioStreamWithCallback<Player>,
}

impl Generator {
    pub fn new(audio: AudioSubsystem, synth: Player) -> Self {
        let spec = AudioSpec {
            freq: Some(44100),
            channels: Some(2),
            format: Some(AudioFormat::f32_sys()),
        };

        let device = audio.open_playback_stream(&spec, synth).unwrap();
        device.resume().expect("Failed to start audio");

        return Self { device };
    }

    pub fn stop(&mut self) {
        self.device.pause().expect("Failed to stop audio");
    }

    pub fn play(&mut self) {
        self.device.resume().expect("Failed to play audio")
    }
}
