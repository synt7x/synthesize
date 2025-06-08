use crate::video::prelude::*;

pub struct Button {
    pub rect: Rect,
    pub active: bool,
    pub hover: bool,
    pub toggled: bool,
    on_click: Callback<Self>,
    text: Text,
}

impl Button {
    pub fn new(label: String, renderer: RenderReference) -> Self {
        return Self {
            rect: Rect::new(0, 0, 0, 0),
            active: false,
            hover: false,
            toggled: false,
            on_click: Box::new(|_| {}),
            text: Text::new(label, renderer),
        };
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        return self.rect.contains_point(Point::new(x as i32, y as i32));
    }

    pub fn color_text(&mut self) {
        self.text.color(if self.active {
            Color::WHITE
        } else if self.hover {
            Color::GREY
        } else {
            Color::RGB(80, 80, 80)
        });
    }

    pub fn on_click(&mut self, callback: Callback<Self>) {
        self.on_click = callback;
    }

    pub fn click(&mut self) {
        let self_ptr: *mut Self = self;
        (self.on_click)(unsafe { &mut *self_ptr });
    }
}

impl Element for Button {
    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        let border_color = if self.active {
            Color::WHITE
        } else if self.hover {
            Color::GREY
        } else {
            Color::RGB(80, 80, 80)
        };

        canvas.set_draw_color(border_color);
        canvas.fill_rect(self.rect).unwrap();

        // Render background
        canvas.set_draw_color(Color::BLACK);
        canvas
            .fill_rect(Rect::new(
                self.rect.x + 4,
                self.rect.y + 4,
                self.rect.width() - 8,
                self.rect.height() - 8,
            ))
            .unwrap();

        self.text.render(canvas);
    }

    fn size(&mut self, width: u32, height: u32) {
        self.rect.resize(width, height);
        self.text.size(width - 16, height - 16);
        self.text.rect.center_on(self.rect.center());
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect.reposition(Point::new(x, y));
        self.text.rect.center_on(self.rect.center());
    }

    fn update(&mut self, event: &Event) {
        match *event {
            Event::MouseMotion { x, y, .. } => {
                self.hover = self.contains(x, y);
                self.color_text();
            }
            Event::MouseButtonDown { x, y, .. } => {
                if self.contains(x, y) {
                    self.active = true;
                    self.color_text();
                }
            }
            Event::MouseButtonUp { x, y, .. } => {
                if self.contains(x, y) {
                    self.click();
                }
                
                if !self.toggled {
                    self.active = false;
                }

                self.color_text();
            }
            _ => {}
        }
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}
