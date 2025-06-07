use crate::audio::prelude::*;
use std::sync::{Arc, Mutex};

pub enum Shape {
    Square,
    Sawtooth,
    Sine,
    Triangle,
}

pub enum Mode {
    Oscillator(Shape),
    Multi(Shape, u8, f32),
}

pub struct Synth {
    note: f32,
    phase: f32,
    volume: f32,

    mode: Mode,
    pub stream: Stream,
}

impl Synth {
    pub fn new() -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(Self {
            note: 440.0,
            phase: 0.0,
            volume: 0.025,
            mode: Mode::Oscillator(Shape::Square),
            stream: Vec::new(),
        }));
    }

    pub fn get_stream(&mut self) -> Stream {
        return self.stream.clone();
    }

    fn mix(stream: &mut Stream, streams: Vec<Stream>) {
        let length = streams.iter().map(|stream| stream.len()).min().unwrap_or(0);

        let mix_amount = streams.len() as f32;

        for child in streams {
            for i in 0..length {
                stream[i] += child[i]
            }
        }

        for sample in stream.iter_mut() {
            *sample /= mix_amount;
        }
    }

    fn square(&mut self, note: f32, stream: &mut Stream) {
        for _ in 0..stream.capacity() {
            stream.push(if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            });

            stream.push(if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            });


            self.phase = (self.phase + note / 44100.0) % 1.0;
        }
    }

    fn saw(&mut self, note: f32, stream: &mut Stream) {}
}

impl AudioCallback<f32> for Synth {
    fn callback(&mut self, stream: &mut AudioStream, requested: i32) {
        let mut audio = Stream::with_capacity(requested as usize);

        match &self.mode {
            Mode::Oscillator(shape) => match shape {
                Shape::Square => self.square(self.note, &mut audio),
                Shape::Sawtooth => self.saw(self.note, &mut audio),
                Shape::Sine => self.square(self.note, &mut audio),
                Shape::Triangle => self.square(self.note, &mut audio),
            },
            Mode::Multi(shape, voices, detune) => {}
        }

        stream
            .put_data_f32(&audio)
            .expect("Failed to generate audio");

        self.stream = audio;
    }
}
