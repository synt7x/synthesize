use sdl3::{event::Event, pixels::Color};

fn main() {
    let context = sdl3::init().unwrap();
    let video = context.video().unwrap();

    let window = video
        .window("Synthesize", 1200, 900)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut pump = context.event_pump().unwrap();

    // Render loop
    loop {
        for event in pump.poll_iter() {
            // Match incoming OS events
            match event {
                Event::Quit { .. } => return,
                _ => {},
            };
        }

        // Prepare canvas for rendering
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Present rendered canvas
        canvas.present();
    }
}
