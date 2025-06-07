use std::collections::VecDeque;

use crate::audio::prelude::*;
use crate::video::prelude::*;

pub struct Visualizer {
    pub rect: Rect,
    pub stream: VecDeque<f32>,
    pub zoom: usize,
}

impl Visualizer {
    pub fn new() -> Self {
        let zoom = 2000;
        return Self {
            rect: Rect::new(0, 0, 0, 0),
            stream: VecDeque::with_capacity(zoom),
            zoom,
        };
    }

    pub fn stream(&mut self, stream: Stream) {
        for sample in stream {
            if self.stream.len() == self.stream.capacity() {
                self.stream.pop_front();
            }

            self.stream.push_back(sample);
        }
    }
}

impl Element for Visualizer {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::WHITE);

        let height = self.rect.height();
        let width = self.rect.width();

        let center= height as i32 / 2;
        let step = self.zoom as f32 / width as f32;

        let mut last_x = self.rect.x;
        let mut last_y = center;

        for px in 1..width {
            let sample = (px as f32 * step) as usize;
            if sample >= self.stream.len() {
                break
            }

            let x = self.rect.x + px as i32;
            let y = center - (self.stream[sample] * 4.0 * (height as f32 / 2.0)) as i32;

            canvas.draw_line(
                Point::new(last_x, last_y),
                Point::new(x, y)
            ).unwrap();

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
