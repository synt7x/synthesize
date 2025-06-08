use crate::video::prelude::*;

pub struct Root {
    pub children: Elements,
    pub rect: Rect,
}

impl Root {
    pub fn new(width: u32, height: u32) -> Self {
        return Self {
            children: Vec::new(),
            rect: Rect::new(0, 0, width, height),
        };
    }

    pub fn add(&mut self, child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        self.children.push(child);
        self.size(width, height);
        return self.children.len() - 1;
    }

    pub fn get<T: 'static>(&mut self, idx: usize) -> Option<&mut T> {
        return self
            .children
            .get_mut(idx)?
            .as_mut()
            .dynamic()
            .downcast_mut::<T>();
    }
}

impl Element for Root {
    fn size(&mut self, width: u32, height: u32) {
        self.rect.resize(width, height);

        let mut dx = 0;
        for child in self.children.iter_mut() {
            child.size(width, height);
            child.position(self.rect.x + dx, self.rect.y);
            dx += child.rect().width() as i32;
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        for child in self.children.iter_mut() {
            child.render(canvas);
        }
    }

    fn update(&mut self, event: &Event) {
        for child in self.children.iter_mut() {
            child.update(event);
        }

        match event {
            Event::Window { win_event, .. } => {
                if let WindowEvent::Resized(width, height) = *win_event {
                    self.size(width as u32, height as u32);
                }
            }
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
