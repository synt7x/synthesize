use crate::video::prelude::*;

pub struct Slider {
    pub rect: Rect,
    pub value: f32,
    on_change: Callback<Self>,
    text: Text,
    button: Button,
    max: f32,
    min: f32,
}

impl Slider {
    pub fn new(label: String, default: f32, max: f32, min: f32, renderer: RenderReference) -> Self {
        return Self {
            rect: Rect::new(0, 0, 0, 0),
            on_change: Box::new(|_| {}),
            button: Button::new(" ".to_owned(), renderer.clone()),
            text: Text::new(label, renderer),
            value: default,
            min,
            max,
        }
    }

    pub fn on_change(&mut self, callback: Callback<Self>) {
        self.on_change = callback;
    }
}

impl Element for Slider {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        let center_y = self.rect.center().y;
        let width = self.rect.width();
        
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(
            Point::new(self.rect.x as i32, center_y),
            Point::new(self.rect.x + width as i32, center_y)
        ).unwrap();

        let t = (1.0 - (self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        let button_x = self.rect.x as f32 + t * width as f32;

        self.button.position(button_x as i32 - 15, self.rect.center().y - 15);
        self.button.render(canvas);
        self.text.render(canvas);
    }

    fn update(&mut self, event: &Event) {
        self.button.update(event);

        match *event {
            Event::MouseMotion { x, y, .. } => {
                if self.button.active {
                    let width = self.rect.width() as f32;
                    let t = 1.0 - ((x as f32 - self.rect.x as f32) / width).clamp(0.0, 1.0);
                    self.value = self.min + t * (self.max - self.min);

                    let self_ptr: *mut Self = self;
                    (self.on_change)(unsafe { &mut *self_ptr });
                }
            }
            _ => {}
        }
    }

    fn size(&mut self, width: u32, height: u32) {
        self.button.size(30, 30);
        self.text.size(15 * self.text.label.len() as u32, 21);


        self.text.position(self.rect.x, self.rect.y - 15);
        self.rect.resize(width, height);
    }

    fn position(&mut self, x: i32, y: i32) {
        self.rect.reposition(Point::new(x, y));
        self.text.position(self.rect.x, self.rect.y - 15);
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}