use std::sync::{Arc, Mutex};

use crate::audio::prelude::*;
use crate::video::prelude::*;
pub struct App {
    renderer: RenderReference,
    root: Root,
}

impl App {
    pub fn new(creator: TextureTarget, width: u32, height: u32, synth: Player) -> Self {
        let renderer: RenderReference = Renderer::new(creator);
        let mut root: Root = Root::new(width, height);
        let left_col: &mut Col = insert!(root, Col::new(0.5));
        let top_left: &mut Row = insert!(left_col, Row::new(0.5));

        let vis_padding: &mut Padding = insert!(top_left, Padding::new(16));
        let vis_border: &mut Border = insert!(vis_padding, Border::new(2));
        let visualizer: &mut Visualizer = insert!(vis_border, Visualizer::new(synth.0));

        let bottom_left: &mut Row = insert!(left_col, Row::new(0.5));
        let panel_padding: &mut Padding = insert!(bottom_left, Padding::new(16));
        let panel_border: &mut Border = insert!(panel_padding, Border::new(2));

        let right_col: &mut Col = insert!(root, Col::new(0.5));
        let controls_padding: &mut Padding = insert!(right_col, Padding::new(16));
        let controls_border: &mut Border = insert!(controls_padding, Border::new(2));

        return Self { renderer, root };
    }

    pub fn set_panel_height(&mut self, height: f32) {
        let left_col: &mut Col = self.root.get(0).unwrap();
        let top_left: &mut Row = left_col.get(0).unwrap();

        top_left.adjust(height);
        self.recalculate();
    }

    pub fn set_panel_width(&mut self, width: f32) {
        let left_col: &mut Col = self.root.get(0).unwrap();

        left_col.adjust(width);
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        self.root.size(self.root.rect.width(), self.root.rect.height());
    }

    pub fn update(&mut self, event: &Event) {
        self.root.update(event);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
