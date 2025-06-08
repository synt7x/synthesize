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

    pub fn add(&mut self, child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        self.children.push(child);
        self.size(width, (height as f32 / self.height) as u32);
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

    pub fn adjust(&mut self, height: f32) {
        let previous_height = self.rect.height();
        let previous_percentage = self.height;
        self.height = height;

        self.size(self.rect.width(), (previous_height as f32 / previous_percentage) as u32);
    }
}

impl Element for Row {
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
        let height = (height as f32 * self.height) as u32;
        self.rect.resize(width, height);

        let mut dx = 0;
        let length = self.children.len();

        for child in self.children.iter_mut() {
            child.size((width as f32 / length as f32) as u32, height);
            child.position(self.rect.x + dx, self.rect.y);
            dx += child.rect().width() as i32;
        }
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect.reposition(Point::new(x, y));
        let mut dx = 0;
        for child in self.children.iter_mut() {
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
