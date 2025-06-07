use std::any::Any;
use crate::video::prelude::*;

pub trait Element: Any {
    fn update(&mut self, event: &Event) {}
    fn render(&self, canvas: &mut WindowCanvas);
    fn rect(&mut self) -> &mut Rect;

    fn size(&mut self, width: u32, height: u32) {
        self.rect().resize(width, height);
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect().reposition(Point::new(x, y));
    }

    fn color(&mut self, color: Color) {}
    fn dynamic(&mut self) -> &mut dyn Any;
}