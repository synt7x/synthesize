use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::audio::prelude::*;
use crate::video::prelude::*;

pub struct Visualizer {
    pub rect: Rect,
    pub stream: VecDeque<f32>,
    pub zoom: usize,

    synth: Arc<Mutex<Synth>>,
}

impl Visualizer {
    pub fn new(synth: Arc<Mutex<Synth>>) -> Self {
        let zoom = 2000;
        return Self {
            rect: Rect::new(0, 0, 0, 0),
            stream: VecDeque::with_capacity(zoom),
            zoom,
            synth,
        };
    }
}

impl Element for Visualizer {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        for sample in self.synth.lock().unwrap().get_stream() {
            if self.stream.len() == self.stream.capacity() {
                self.stream.pop_front();
            }

            self.stream.push_back(sample);
        }

        canvas.set_draw_color(Color::WHITE);

        let height = self.rect.height();
        let width = self.rect.width();

        let center = height as i32 / 2;
        let step = self.zoom as f32 / width as f32;

        let mut last_x = self.rect.x;
        let mut last_y = center;

        for px in 1..width {
            let sample = (px as f32 * step) as usize;
            if sample >= self.stream.len() {
                break;
            }

            let x = self.rect.x + px as i32;
            let y = center - (self.stream[sample] * 4.0 * (height as f32 / 2.0)) as i32;

            for dx in -1..1 {
                for dy in -1..1 {
                    canvas
                        .draw_line(
                            Point::new(last_x + dx, last_y + dy),
                            Point::new(x + dx, y + dy),
                        )
                        .unwrap();
                }
            }

            last_x = x;
            last_y = y;
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
