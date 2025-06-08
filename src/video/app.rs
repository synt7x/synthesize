use std::cell::Cell;
use std::sync::{Arc, Mutex};

use crate::audio::prelude::*;
use crate::video::prelude::*;
pub struct App {
    renderer: RenderReference,
    root: Root,
}

impl App {
    pub fn new(creator: TextureTarget, width: u32, height: u32, synth: Player) -> Self {
        let renderer: RenderReference = Renderer::new(creator);
        let mut root: Root = Root::new(width, height);
        let left_col: &mut Col = insert!(root, Col::new(0.5));
        let top_left: &mut Row = insert!(left_col, Row::new(0.9));

        let vis_padding: &mut Padding = insert!(top_left, Padding::new(16));
        let vis_border: &mut Border = insert!(vis_padding, Border::new(2));
        let visualizer: &mut Visualizer = insert!(vis_border, Visualizer::new(synth.0.clone()));

        let bottom_left: &mut Row = insert!(left_col, Row::new(0.2));
        let panel_padding: &mut Padding = insert!(bottom_left, Padding::new(16));
        let panel_border: &mut Border = insert!(panel_padding, Border::new(2));
        let panel_grid: &mut Col = insert!(panel_border, Col::new(1.0));

        let shape_row: &mut Row = insert!(panel_grid, Row::new(0.5));
        let shape = Rc::new(Cell::new(Shape::Sawtooth));

        let saw_btn_padding: &mut Padding = insert!(shape_row, Padding::new(16));
        let saw_btn_ref: &mut Button = insert!(
            saw_btn_padding,
            Button::new("Sawtooth".to_owned(), renderer.clone())
        );
        let saw_btn = Rc::new(saw_btn_ref as *mut Button);

        saw_btn_ref.active = true;
        saw_btn_ref.toggled = true;

        let square_btn_padding: &mut Padding = insert!(shape_row, Padding::new(16));
        let square_btn_ref: &mut Button = insert!(
            square_btn_padding,
            Button::new("Square".to_owned(), renderer.clone())
        );
        let square_btn = Rc::new(square_btn_ref as *mut Button);

        let triangle_btn_padding: &mut Padding = insert!(shape_row, Padding::new(16));
        let triangle_btn_ref: &mut Button = insert!(
            triangle_btn_padding,
            Button::new("Triangle".to_owned(), renderer.clone())
        );
        let triangle_btn = Rc::new(triangle_btn_ref as *mut Button);

        let sine_btn_padding: &mut Padding = insert!(shape_row, Padding::new(16));
        let sine_btn_ref: &mut Button = insert!(
            sine_btn_padding,
            Button::new("Sine".to_owned(), renderer.clone())
        );
        let sine_btn = Rc::new(sine_btn_ref as *mut Button);

        let all_buttons = vec![
            (saw_btn.clone(), Shape::Sawtooth),
            (square_btn.clone(), Shape::Square),
            (triangle_btn.clone(), Shape::Triangle),
            (sine_btn.clone(), Shape::Sine),
        ];

        for (btn_ptr_rc, shape_type) in &all_buttons {
            let shape = shape.clone();
            let all_buttons = all_buttons.clone();
            let shape_type = *shape_type;

            let btn_ptr = *btn_ptr_rc.clone();
            let synth = synth.0.clone();

            unsafe {
                if let Some(btn) = btn_ptr.as_mut() {
                    btn.on_click(Box::new(move |_| {
                        shape.set(shape_type);
                        let mut synth = synth.as_ref().lock().unwrap();
                        synth.mode = Mode::Oscillator(shape_type);

                        for (other_btn_ptr_rc, other_shape) in &all_buttons {
                            let other_btn_ptr = **other_btn_ptr_rc;

                            if let Some(other_btn) = other_btn_ptr.as_mut() {
                                let selected = shape.get() == *other_shape;
                                other_btn.active = selected;
                                other_btn.toggled = selected;
                                other_btn.color_text();
                            }
                        }
                    }));
                }
            }
        }


        let playback_row: &mut Row = insert!(panel_grid, Row::new(0.5));
        let playback_btns: &mut Col = insert!(playback_row, Col::new(0.5));
        let playback_btns_grid: &mut Row = insert!(playback_btns, Row::new(1.0));
        let play_btn_padding: &mut Padding = insert!(playback_btns_grid, Padding::new(16));
        let play_btn: &mut Button = insert!(
            play_btn_padding,
            Button::new("Play".to_owned(), renderer.clone())
        );

        let play_synth = synth.0.clone();
        play_btn.on_click(Box::new(move |_| {
            let mut synth = play_synth.lock().unwrap();
            synth.playing = true;
        }));

        let stop_btn_padding: &mut Padding = insert!(playback_btns_grid, Padding::new(16));
        let stop_btn: &mut Button = insert!(
            stop_btn_padding,
            Button::new("Stop".to_owned(), renderer.clone())
        );

        let stop_synth = synth.0.clone();
        stop_btn.on_click(Box::new(move |_| {
            let mut synth = stop_synth.lock().unwrap();
            synth.playing = false;
        }));


        let right_col: &mut Col = insert!(root, Col::new(0.5));
        let controls_padding: &mut Padding = insert!(right_col, Padding::new(16));
        let controls_border: &mut Border = insert!(controls_padding, Border::new(2));

        return Self { renderer, root };
    }

    pub fn set_panel_height(&mut self, height: f32) {
        let left_col: &mut Col = self.root.get(0).unwrap();
        let top_left: &mut Row = left_col.get(0).unwrap();

        top_left.adjust(height);
        self.recalculate();
    }

    pub fn set_panel_width(&mut self, width: f32) {
        let left_col: &mut Col = self.root.get(0).unwrap();

        left_col.adjust(width);
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        self.root
            .size(self.root.rect.width(), self.root.rect.height());
    }

    pub fn update(&mut self, event: &Event) {
        self.root.update(event);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
