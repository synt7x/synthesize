use crate::video::prelude::*;

pub struct Renderer {
    creator: TextureTarget,

    pub textures: Textures,
    pub id: ID,
}

impl Renderer {
    pub fn new(creator: TextureTarget) -> RenderReference {
        return Rc::new(RefCell::new(Self {
            creator,

            textures: HashMap::new(),
            id: 0,
        }));
    }

    pub fn texture(&mut self, width: u32, height: u32) -> ID {
        self.id += 1;

        let format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::ARGB8888) };
        let texture = self.creator.create_texture_static(format, width, height);
        let mut texture = unsafe { transmute::<Texture<'_>, RenderTexture>(texture.unwrap()) };

        texture.set_blend_mode(BlendMode::Blend);
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        self.textures.insert(self.id, texture);

        return self.id;
    }

    pub fn render(&mut self, id: ID, rect: Rect, data: &[u8]) {
        let pitch = rect.width() * 4;

        if let Some(texture) = self.textures.get_mut(&id) {
            texture.update(rect, data, pitch as usize).unwrap();
        }
    }

    pub fn get(&self, id: ID) -> Option<&Texture> {
        return self.textures.get(&id);
    }
}
