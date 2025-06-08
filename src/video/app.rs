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
        let left_col_ptr = Rc::new(left_col as *mut Col);
        let top_left: &mut Row = insert!(left_col, Row::new(0.9));
        let top_left_ptr = Rc::new(top_left as *mut Row);

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
        let playback_btns: &mut Col = insert!(playback_row, Col::new(1.0));
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

        let playback_slider: &mut Col = insert!(playback_row, Col::new(1.0));
        let slider_padding: &mut Padding = insert!(playback_slider, Padding::new(16));
        let octave_slider: &mut Slider = insert!(slider_padding, Slider::new("Octave".to_owned(), 3.0, 0.1, 14.0, renderer.clone()));

        let slider_synth = synth.0.clone();
        octave_slider.on_change(Box::new(move |slider| {
            let value = slider.value;
            let mut synth = slider_synth.lock().unwrap();
            synth.octave = value;
        }));

        let right_col: &mut Col = insert!(root, Col::new(0.5));
        let top_right: &mut Row = insert!(right_col, Row::new(0.33));
        let controls_padding: &mut Padding = insert!(top_right, Padding::new(16));
        let controls_border: &mut Border = insert!(controls_padding, Border::new(2));

        let filter_btns: &mut Col = insert!(controls_border, Col::new(1.0));
        let filter_btns_grid: &mut Row = insert!(filter_btns, Row::new(0.3));
        let off_btn_padding: &mut Padding = insert!(filter_btns_grid, Padding::new(16));
        let off_btn: &mut Button = insert!(
            off_btn_padding,
            Button::new("Bypass".to_owned(), renderer.clone())
        );

        let off_synth = synth.0.clone();
        off_btn.on_click(Box::new(move |_| {
            let mut synth = off_synth.lock().unwrap();
            synth.filter.set_filter(Filtering::None);
        }));

        let high_pass_btn_padding: &mut Padding = insert!(filter_btns_grid, Padding::new(16));
        let high_pass_btn: &mut Button = insert!(
            high_pass_btn_padding,
            Button::new("High Pass".to_owned(), renderer.clone())
        );

        let high_pass_synth = synth.0.clone();
        high_pass_btn.on_click(Box::new(move |_| {
            let mut synth = high_pass_synth.lock().unwrap();
            synth.filter.set_filter(Filtering::HighPass);
        }));

        let low_pass_btn_padding: &mut Padding = insert!(filter_btns_grid, Padding::new(16));
        let low_pass_btn: &mut Button = insert!(
            low_pass_btn_padding,
            Button::new("Low Pass".to_owned(), renderer.clone())
        );

        let low_pass_synth = synth.0.clone();
        low_pass_btn.on_click(Box::new(move |_| {
            let mut synth = low_pass_synth.lock().unwrap();
            synth.filter.set_filter(Filtering::LowPass);
        }));

        let cutoff_grid: &mut Row = insert!(filter_btns, Row::new(0.7));
        let cutoff_padding: &mut Padding = insert!(cutoff_grid, Padding::new(16));
        let cutoff_slider: &mut Slider = insert!(cutoff_padding, Slider::new("Filter Cutoff".to_owned(), 0.5, 1.0, 0.0, renderer.clone()));
        let cutoff_synth = synth.0.clone();
        cutoff_slider.on_change(Box::new(move |slider| {
            let value = slider.value;
            let mut synth = cutoff_synth.lock().unwrap();
            synth.alpha = value;
        }));

        let educator_ref: &mut Educator = insert!(root, Educator::new(renderer.clone()));
        let educator_ptr = Rc::new(educator_ref as *mut Educator);

        unsafe {
            if let Some(educator) = educator_ptr.as_mut() {
                let educator_clone = educator_ptr.clone();
                educator.button.on_click(Box::new(move |_| {
                    if let Some(educator) = educator_clone.as_mut() {
                        educator.index += 1;
                        educator.refresh();

                        if educator.index >= MESSAGES.len() {
                            return;
                        }

                        if MESSAGES[educator.index].1 == 1 {
                            let top_left = top_left_ptr.as_mut().unwrap();
                            top_left.adjust(0.8);
                        } else if MESSAGES[educator.index].1 == 2 {
                            let left_col: &mut Col = left_col_ptr.as_mut().unwrap();
                            left_col.adjust(0.5);
                        }
                    }
                }));
            }
        }

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
        self.root
            .size(self.root.rect.width(), self.root.rect.height());
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.root.render(canvas);
    }
}
