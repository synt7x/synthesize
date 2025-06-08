use crate::audio::prelude::*;

use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

pub const NOTES: [(f32, f32); 43] = [
    (4.0, 43.65),
    (1.0, 65.41),
    (3.0, 69.30),
    (4.0, 65.41),
    (4.0, 51.91),
    (1.0, 43.65),
    (3.0, 38.89),
    (2.0, 69.30),
    (2.0, 58.27),
    (4.0, 65.41),
    (4.0, 51.91),
    (4.0, 43.65),
    (1.0, 65.41),
    (3.0, 69.30),
    (4.0, 65.41),
    (4.0, 51.91),
    (1.0, 43.65),
    (3.0, 38.89),
    (2.0, 69.30),
    (2.0, 58.27),
    (4.0, 65.41),
    (4.0, 51.91),
    (4.0, 87.31),
    (1.0, 65.41),
    (3.0, 69.30),
    (4.0, 65.41),
    (4.0, 51.91),
    (1.0, 43.65),
    (3.0, 51.91),
    (2.0, 69.30),
    (2.0, 58.27),
    (4.0, 65.41),
    (4.0, 51.91),
    (4.0, 43.65),
    (1.0, 65.41),
    (3.0, 51.91),
    (4.0, 69.30),
    (4.0, 58.27),
    (1.0, 43.65),
    (3.0, 51.91),
    (4.0, 69.30),
    (4.0, 65.41),
    (4.0, 43.65),
];

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub note: f32,
    pub octave: f32,
    pub playing: bool,
    phase: f32,
    volume: f32,

    pub alpha: f32,
    pub mode: Mode,
    pub stream: Stream,
    pub filter: Filter,
}

impl Synth {
    pub fn new() -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(Self {
            note: NOTES[6].1,
            phase: 0.0,
            volume: 0.025,
            mode: Mode::Oscillator(Shape::Sawtooth),
            stream: Vec::new(),
            playing: true,
            octave: 3.0,
            alpha: 0.5,
            filter: Filter::new(),
        }));
    }

    pub fn get_stream(&mut self) -> Stream {
        return self.stream.clone();
    }

    fn mix(&self, stream: &mut Stream, streams: Vec<Stream>) {
        let length = streams.iter().map(|stream| stream.len()).min().unwrap_or(0);

        let mix_amount = streams.len() as f32 / 2.0;

        for child in streams {
            for i in 0..length {
                if i >= stream.len() {
                    stream.push(child[i]);
                } else {
                    stream[i] += child[i];
                }
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

    fn saw(&mut self, note: f32, stream: &mut Stream) {
        for _ in 0..stream.capacity() {
            let sample = (0.5 - self.phase) * self.volume * 2.0;
            stream.push(sample);
            stream.push(sample);

            self.phase = (self.phase + note / 44100.0) % 1.0;
        }
    }

    fn triangle(&mut self, note: f32, stream: &mut Stream) {
        for _ in 0..stream.capacity() {
            let sample = ((0.5 - self.phase).abs() * 2.0 - 0.5) * self.volume * 2.0;
            stream.push(sample);
            stream.push(sample);

            self.phase = (self.phase + note / 44100.0) % 1.0;
        }
    }

    fn sine(&mut self, note: f32, stream: &mut Stream) {
        for _ in 0..stream.capacity() {
            let sample = (PI * 2.0 * self.phase).sin() * self.volume;
            stream.push(sample);
            stream.push(sample);

            self.phase = (self.phase + note / 44100.0) % 1.0;
        }
    }
}

impl AudioCallback<f32> for Synth {
    fn callback(&mut self, stream: &mut AudioStream, requested: i32) {
        if !self.playing {
            self.note = 0.0;
        };
        let mut audio = Stream::with_capacity(requested as usize);

        match &self.mode {
            Mode::Oscillator(shape) => match shape {
                Shape::Square => self.square(self.note * self.octave, &mut audio),
                Shape::Sawtooth => self.saw(self.note * self.octave, &mut audio),
                Shape::Sine => self.sine(self.note * self.octave, &mut audio),
                Shape::Triangle => self.triangle(self.note * self.octave, &mut audio),
            },
            Mode::Multi(shape, voices, detune) => match shape {
                Shape::Sawtooth => {
                    let mut streams: Vec<Vec<f32>> = Vec::new();

                    for i in 0..*voices {
                        let tune = 0.0;
                        let mut stream = Vec::<f32>::with_capacity(audio.capacity());
                        self.saw(self.note + tune, &mut stream);

                        streams.push(stream);
                    }

                    self.mix(&mut audio, streams);
                }
                _ => {}
            },
        }

        self.filter.pass(&mut audio, self.alpha);

        stream
            .put_data_f32(&audio)
            .expect("Failed to generate audio");

        self.stream = audio;
    }
}
