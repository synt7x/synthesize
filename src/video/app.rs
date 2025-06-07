use crate::video::prelude::*;

pub struct App {
    renderer: RenderReference
}

impl App {
    pub fn new(creator: TextureTarget) -> Self {
        let renderer: RenderReference = Renderer::new(creator);
        
        // Component logic
        
        return Self { renderer };
    }

    pub fn update(&mut self, event: &Event) {
    
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
    
    }
}
