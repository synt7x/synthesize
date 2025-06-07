use crate::video::prelude::*;
use crate::audio::prelude::Stream;
use crate::video::ui::visualizer;

pub struct App {
    renderer: RenderReference,
    root: Root,
    visualizer: usize
}

impl App {
    pub fn new(creator: TextureTarget, width: u32, height: u32) -> Self {
        let renderer: RenderReference = Renderer::new(creator);
        let mut root: Root = Root::new(width, height);
        let visualizer_idx = root.add(Box::new(Visualizer::new()));
        
        // Component logic
        
        return Self { renderer, root, visualizer: visualizer_idx };
    }

    pub fn stream(&mut self, stream: Stream) {
        let mut visualizer: &mut Visualizer = self.root.get(self.visualizer).unwrap();
        visualizer.stream(stream);
    }

    pub fn update(&mut self, event: &Event) {
        self.root.update(event);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
