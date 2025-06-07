use crate::video::prelude::*;

pub struct Root {
    pub children: Elements,
    pub rect: Rect
}

impl Root {
    pub fn new(width: u32, height: u32) -> Self {
        return Self {
            children: Vec::new(),
            rect: Rect::new(0, 0, width, height),
        }
    }

    pub fn add(&mut self, mut child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        child.size(width, height);
        self.children.push(child);
        return self.children.len() - 1;
    }

    pub fn get<T: 'static>(&mut self, idx: usize) -> Option<&mut T> {
        return self.children
            .get_mut(idx)?.as_mut()
            .dynamic().downcast_mut::<T>();
    }
}

impl Element for Root {
    fn size(&mut self, width: u32, height: u32) {
        self.rect.resize(width, height);

        for child in self.children.iter_mut() {
            child.size(width, height);
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        for child in self.children.iter() {
            child.render(canvas);
        }
    }

    fn update(&mut self, event: &Event) {
        for child in self.children.iter_mut() {
            child.update(event);
        }

        match event {
            Event::Window { win_event, .. } => {
                if let WindowEvent::Resized(width,  height) = *win_event {
                    self.size(width as u32, height as u32);
                }
            },
            _ => {}
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}