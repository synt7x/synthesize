use crate::video::prelude::*;

pub struct App {
    renderer: RenderReference,
    root: Root
}

impl App {
    pub fn new(creator: TextureTarget, width: u32, height: u32) -> Self {
        let renderer: RenderReference = Renderer::new(creator);
        let mut root = Root::new(width, height);
        
        // Component logic
        
        return Self { renderer, root };
    }

    pub fn update(&mut self, event: &Event) {
        self.root.update(event);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
