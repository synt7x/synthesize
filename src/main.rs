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

    // Holds rendering and updating logic for the UI
    let size: (u32, u32) = canvas.output_size().unwrap();
    let mut synth = Synth::new();
    let generator = Generator::new(audio, Player(synth.clone()));
    let mut app = App::new(creator, size.0, size.1, Player(synth.clone()));

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
