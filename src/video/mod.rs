pub mod app;
pub mod font;
pub mod render;
pub mod ui;

pub mod prelude {
    use std::any::Any;
    pub use std::{cell::RefCell, collections::HashMap, mem::transmute, rc::Rc};

    pub use sdl3::{
        event::{Event, WindowEvent},
        pixels::{Color, PixelFormat},
        rect::{Point, Rect},
        render::{BlendMode, Canvas, Texture, TextureCreator},
        sys::pixels::SDL_PixelFormat,
        video::{Window, WindowContext},
    };

    pub use crate::video::{
        app::App,
        render::Renderer,
        ui::element::Element,
    };

    pub type RenderReference = Rc<RefCell<Renderer>>;
    pub type TextureTarget = TextureCreator<WindowContext>;
    pub type RenderTexture = Texture<'static>;
    pub type ID = u32;
    pub type Textures = HashMap<ID, RenderTexture>;
    pub type WindowCanvas = Canvas<Window>;
    pub type Elements = Vec<Box<dyn Element>>;
    pub type Dynamic = dyn Any;

    pub type GlyphMap = [u16; 10];
}
