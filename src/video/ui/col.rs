use crate::video::prelude::*;

pub struct Col {
    pub width: f32,
    pub children: Elements,
    pub rect: Rect,
}

impl Col {
    pub fn new(width: f32) -> Self {
        return Self {
            width,
            children: Vec::new(),
            rect: Rect::new(0, 0, 0, 0),
        };
    }

    pub fn add(&mut self, child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        self.children.push(child);
        self.size((width as f32 / self.width) as u32, height);
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

    pub fn adjust(&mut self, width: f32) {
        let previous_width = self.rect.width();
        let previous_percentage = self.width;
        self.width = width;

        self.size((previous_width as f32 / previous_percentage) as u32, self.rect.height());
    }
}

impl Element for Col {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        for child in self.children.iter_mut() {
            child.render(canvas);
        }
    }

    fn update(&mut self, event: &Event) {
        for child in self.children.iter_mut() {
            child.update(event);
        }
    }

    fn size(&mut self, width: u32, height: u32) {
        let width = (width as f32 * self.width) as u32;
        self.rect.resize(width, height);

        let mut dy = 0;

        for child in self.children.iter_mut() {
            child.size(width, height);
            child.position(self.rect.x, self.rect.y + dy);
            dy += child.rect().height() as i32;
        }
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect.reposition(Point::new(x, y));
        let mut dy = 0;
        for child in self.children.iter_mut() {
            child.position(self.rect.x, self.rect.y + dy);
            dy += child.rect().height() as i32;
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
