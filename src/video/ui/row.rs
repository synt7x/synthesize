use crate::video::prelude::*;

pub struct Row {
    pub height: f32,
    pub children: Elements,
    pub rect: Rect,
}

impl Row {
    pub fn new(height: f32) -> Self {
        return Self {
            height,
            children: Vec::new(),
            rect: Rect::new(0, 0, 0, 0),
        };
    }

    pub fn add(&mut self, mut child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        child.size(width, height);
        self.children.push(child);
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

impl Element for Row {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        for child in self.children.iter_mut() {
            child.render(canvas);
        }
    }

    fn size(&mut self, width: u32, height: u32) {
        let height = (height as f32 * self.height) as u32;
        self.rect.resize(width, height);

        let mut dx = 0;

        for child in self.children.iter_mut() {
            child.size(width, height);
            child.position(self.rect.x + dx, self.rect.y);
            dx += child.rect().width() as i32;
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
