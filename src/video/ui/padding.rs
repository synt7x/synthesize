use crate::video::prelude::*;

pub struct Padding {
    pub weight: u32,
    pub children: Elements,
    pub rect: Rect,
}

impl Padding {
    pub fn new(weight: u32) -> Self {
        return Self {
            weight,
            children: Vec::new(),
            rect: Rect::new(0, 0, 0, 0),
        };
    }

    pub fn add(&mut self, mut child: Box<dyn Element>) -> usize {
        let width = self.rect.width();
        let height = self.rect.height();

        let x = self.rect.x;
        let y = self.rect.y;

        child.size(width - self.weight * 2, height - self.weight * 2);
        child.position(x + self.weight as i32, y + self.weight as i32);

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

impl Element for Padding {
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
        self.rect.resize(width, height);

        for child in self.children.iter_mut() {
            let width = self.rect.width();
            let height = self.rect.height();

            let x = self.rect.x;
            let y = self.rect.y;

            child.size(width - self.weight * 2, height - self.weight * 2);
            child.position(x + self.weight as i32, y + self.weight as i32);
        }
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect.reposition(Point::new(x, y));
        for child in self.children.iter_mut() {
            let x = self.rect.x;
            let y = self.rect.y;

            child.position(x + self.weight as i32, y + self.weight as i32);
        }
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
