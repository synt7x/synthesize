pub mod app;
pub mod font;
pub mod render;
pub mod ui;

pub mod prelude {
    pub use std::{cell::RefCell, collections::HashMap, mem::transmute, rc::Rc};

    pub use sdl3::{
        event::Event,
        pixels::{Color, PixelFormat},
        rect::Rect,
        render::{BlendMode, Canvas, Texture, TextureCreator},
        sys::pixels::SDL_PixelFormat,
        video::{Window, WindowContext},
    };

    pub use crate::video::app::App;
    pub use crate::video::render::Renderer;

    pub type RenderReference = Rc<RefCell<Renderer>>;
    pub type TextureTarget = TextureCreator<WindowContext>;
    pub type RenderTexture = Texture<'static>;
    pub type ID = u32;
    pub type Textures = HashMap<ID, RenderTexture>;
    pub type WindowCanvas = Canvas<Window>;

    pub type GlyphMap = [u16; 10];
}
