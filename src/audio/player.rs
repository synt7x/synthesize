use crate::audio::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Player(pub Arc<Mutex<Synth>>);

impl AudioCallback<f32> for Player {
    fn callback(&mut self, stream: &mut AudioStream, requested: i32) {
        let mut synth = self.0.lock().unwrap();
        synth.callback(stream, requested);
    }
}