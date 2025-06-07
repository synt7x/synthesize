use crate::audio::prelude::*;
use crate::video::prelude::*;

pub struct Visualizer {
    pub rect: Rect,
    pub stream: Stream,
}

impl Visualizer {
    pub fn new() -> Self {
        return Self {
            rect: Rect::new(0, 0, 0, 0),
            stream: Vec::new(),
        };
    }

    pub fn stream(&mut self, stream: Stream) {
        self.stream = stream;
    }
}

impl Element for Visualizer {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::WHITE);

        let height = self.rect.height();
        let width = self.rect.width();

        let center= height as i32 / 2;
        let step = self.stream.len() / width as usize;

        if self.stream.len() == 0 {
            canvas.draw_line(
                Point::new(self.rect.x + width as i32, center),
                Point::new(self.rect.x, center)
            ).unwrap();

            return;
        }

        for (x, i) in (0..self.stream.len()).step_by(step).enumerate() {
            let y = (self.stream[i] * (height as f32 / 2.0)) as i32;
            canvas
                .draw_line(
                    Point::new(x as i32, center - y),
                    Point::new(x as i32, center + y),
                )
                .unwrap();
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
