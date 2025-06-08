use std::thread;
use std::time::Duration;

use crate::audio::prelude::*;
use crate::video::prelude::*;

mod audio;
mod video;

fn main() {
    // Initialize SDL3
    let context = sdl3::init().unwrap();
    let audio = context.audio().unwrap();
    let video = context.video().unwrap();

    // Create the SDL3 window
    let window = video
        .window("Synthesize", 1200, 900)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Rendering + Events
    let mut canvas = window.into_canvas();
    let mut pump = context.event_pump().unwrap();
    let creator = canvas.texture_creator();

    let size: (u32, u32) = canvas.output_size().unwrap();
    let synth = Synth::new();
    let _generator = Generator::new(audio, Player(synth.clone()));

    let mut app = App::new(creator, size.0, size.1, Player(synth.clone()));

    app.set_panel_height(0.5);
    app.set_panel_width(0.5);

    // Render loop
    loop {
        for event in pump.poll_iter() {
            // Match incoming OS events
            match event {
                Event::Quit { .. } => return,
                _ => app.update(&event),
            };
        }

        // Prepare canvas for rendering
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Render the app component
        app.render(&mut canvas);

        // Present rendered canvas
        canvas.present();
    }
}
