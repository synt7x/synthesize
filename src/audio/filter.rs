use crate::audio::prelude::*;

pub enum Filtering {
    HighPass,
    LowPass,
    None,
}

pub struct Filter {
    y: f32,
    prev_x:  f32,
    alpha: f32,
    mode: Filtering
}

impl Filter {
    pub fn new() -> Self {
        return Self {
            y: 0.0,
            prev_x: 0.0,
            alpha: 0.0,
            mode: Filtering::None,
        }
    }

    pub fn set_filter(&mut self, filter: Filtering) {
        self.mode = filter;
    }

    pub fn pass(&mut self, stream: &mut Stream, alpha: f32) {
        match self.mode {
            Filtering::HighPass => self.high_pass(stream, alpha),
            Filtering::LowPass => self.low_pass(stream, alpha),
            _ => {}
        }
    }

    fn low_pass(&mut self, stream: &mut Stream, alpha: f32) {
        if stream.len() == 0 {
            return;
        };

        for i in 1..stream.len() {
            self.y = self.y + alpha * (stream[i] - self.y);
            stream[i] = self.y;
        }
    }

    fn high_pass(&mut self, stream: &mut Stream, alpha: f32) {
        if stream.len() < 2 {
            return;
        }

        for i in 1..stream.len() {
            let x = stream[i];
            self.y = alpha * (self.y + x - self.prev_x);
            stream[i] = self.y;
            self.prev_x = x;
        }
    }
}