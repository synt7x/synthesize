use crate::video::prelude::*;

pub trait Element {
    fn update(&mut self, event: &Event) {}
    fn render(&self, canvas: &mut WindowCanvas);
    fn rect(&mut self) -> &mut Rect;
}