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
        let col: &mut Col = insert!(root, Col::new(0.5));
        let row: &mut Row = insert!(col, Row::new(0.5));
        let vis_padding: &mut Padding = insert!(row, Padding::new(16));
        let vis_border: &mut Border = insert!(vis_padding, Border::new(2));
        let visualizer: &mut Visualizer = insert!(vis_border, Visualizer::new(synth.0));

        return Self { renderer, root };
    }

    pub fn update(&mut self, event: &Event) {
        self.root.update(event);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
