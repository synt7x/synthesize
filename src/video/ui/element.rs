use crate::video::prelude::*;
use std::any::Any;

pub trait Element: Any {
    fn update(&mut self, _: &Event) {}
    fn render(&mut self, canvas: &mut WindowCanvas);
    fn rect(&mut self) -> &mut Rect;

    fn size(&mut self, width: u32, height: u32) {
        self.rect().resize(width, height);
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect().reposition(Point::new(x, y));
    }

    fn color(&mut self, _: Color) {}
    fn dynamic(&mut self) -> &mut Dynamic;
}
