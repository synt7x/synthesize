use crate::video::prelude::*;

pub struct Text {
    pub label: String,
    pub texture: u32,
    pub renderer: RenderReference,
    pub rect: Rect,
    pub color: Color,
}

impl Text {
    pub fn new(label: String, renderer: RenderReference) -> Self {
        let texture = renderer.borrow_mut().texture(Self::width_of(&label), Self::height_of(&label));
        return Self {
            label,
            renderer,
            texture,
            rect: Rect::new(0, 0, 0, 0),
            color: Color::WHITE,
        }
    }

    pub fn render_texture(&mut self) {
        let data = self.pixels();
        let width = self.rect.width() as usize * 4;
        let rect = Rect::new(0, 0, self.rect.width(), self.rect.height());

        self.renderer
            .borrow_mut()
            .render(self.texture, rect, &data);
    }

    pub fn width(&self) -> u32 {
        return Self::width_of(&self.label);
    }

    fn width_of(label: &String) -> u32 {
        let mut width: u32 = 0;
        let mut max_width: u32 = 0;
        for (i, c) in label.chars().enumerate() {
            if c == 'i' || c == 'j' || c == 'l' {
                width += 7
            } else if c == 'f' {
                width += 10
            } else if c == '\n' {
                max_width = max_width.max(width);
                width = 0;
            } else {
                width += 11
            }
        }

        max_width = max_width.max(width);
        return max_width;
    }

    pub fn height_of(label: &String) -> u32 {
        let mut height: u32 = 21;

        for (i, c) in label.chars().enumerate() {
            if c == '\n' {
                height += 21;
            }
        }

        return height;
    }

    pub fn pixels(&self) -> Vec<u8> {
        let width = self.rect.width() as usize;
        let height = self.rect.height() as usize;
        let mut buffer = vec![0u8; width * height * 4];
        let mut dx = 0;
        let mut line = 0;

        // For now, render all characters as font[1]
        for (i, c) in self.label.chars().enumerate() {
            let glyph = font::get_glyph(c);

            if dx != 0 && (c == 'i' || c == 'l' || c == 'j') {
                dx -= 2
            }

            if c == '\n' {
                line += 1;
                dx = 0;
                continue;
            }

            for x in 0..10 {
                let col = glyph[x];
                for y in 0..15 {
                    // Highest bit is at the bottom (y=14), so bit 14-y
                    let pixel_on = (col >> (y)) & 1 != 0;
                    let px = dx + x;
                    let py = if c == 'g' || c == 'p' || c == 'q' || c == 'y' {
                        y + 6
                    } else if c == 'j' { y + 3 } else { y } + line * 21;

                    if px < width && py < height {
                        let offset = (py * width + px) * 4;

                        if pixel_on {
                            let color = self.color;
                            buffer[offset] = color.r;     // R
                            buffer[offset + 1] = color.g; // G
                            buffer[offset + 2] = color.b; // B
                            buffer[offset + 3] = 255; // A
                        }
                    }
                }
            }

            if c == 'i' || c == 'j' || c == 'l' {
                dx += 9
            } else if c == 'f' {
                dx += 10
            } else {
                dx += 11
            }
        }

        buffer
    }
}

impl Element for Text {
    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(texture) = self.renderer.borrow().get(self.texture) {
            canvas.copy(texture, None, self.rect).unwrap();
        }
    }

    fn size(&mut self, width: u32, height: u32) {
        let text_width = self.width();
        let ratio = height as f32 / Self::height_of(&self.label) as f32;
        let scaled = (text_width as f32 * ratio) as u32;

        if scaled > width {
            let ratio = width as f32 / text_width as f32;
            let scaled = (Self::height_of(&self.label) as f32 * ratio) as u32;
            self.rect.resize(width, scaled);
        } else {
            self.rect.resize(scaled, height);
        }

        self.render_texture();
    }

    fn color(&mut self, color: Color) {
        self.color = color;
        self.render_texture();
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        self
    }
}
