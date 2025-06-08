use crate::video::prelude::*;

pub const MESSAGES: [(&str, u8); 26] = [
    ("Welcome to synthesize\nThis is a tool to help\nyou learn the basics\nof audio synthesis!", 0),
    ("The first thing I\nwant you to notice is\nthe waveform renderer\nbelow", 0),
    ("It shows the\ncurrently playing\naudio as its\nwaveform", 0),
    ("This is the\ndirect output from\nthe synthesizer", 0),
    ("What you are\ncurrently seeing is\nknown as a\nSawtooth Wave", 0),
    ("It gets this name\nbecause of its\nsimilarity to the\nteeth on a saw", 0),
    ("There are however\nother basic waveforms", 0),
    ("I have opened a panel\nthat contains various\nother wave types", 1),
    ("Try picking different\nshapes and observe\ntheir effects on the\ntimbre of the sound", 0),
    ("You can also use the\noctave slider to\nchange the pitch", 0),
    ("Notice that as the\npitch gets higher\nthe waves get closer", 0),
    ("This is because a\nhigher pitch means\nmore oscillations per\nsecond", 0),
    ("Thus the cycles of\nthe wave must become\ncloser together to\noscillate more often", 0),
    ("Feel free to continue\nusing this screen\nYou can press Next\nto proceed", 0),
    ("I have now opened a\nsecond panel with the\ncontrols for the\nsynths filter", 2),
    ("The filter cuts out\ndifferent parts of\nthe waveform based\non their frequency", 0),
    ("Currently the filter\nis bypassed and thus\nhas no effect on the\nwaveform", 0),
    ("The high pass filter\ncuts out the low\nfrequencies in the\nwaveform", 0),
    ("The low pass filter\ncuts out the high\nfrequencies in the\nwaveform", 0),
    ("The filter cutoff\nspecifies how many\nfrequencies should be\ncut from the wave", 0),
    ("Dragging the slider\nto the left increases\nthe cutoff and removes\nmore frequencies", 0),
    ("Play around with\nthe filter combined\nwith different\nwaveforms", 0),
    ("This is one of the\nbasic ways to create\nnew and unique sounds", 0),
    ("Try varying the pitch\nto see how the\nfrequencies are\naffected", 0),
    ("Hopefully now you\nhave a basic\nunderstanding of\naudio synthesis", 0),
    ("Thanks for playing", 0),
];

pub struct Educator { 
    pub text: Text,
    pub button: Button,
    pub rect: Rect,
    pub index: usize,
}

impl Educator { 
    pub fn new(renderer: RenderReference) -> Self {
        let size = Rect::new(32, 32, 250, 250);
        let mut text = Text::new(MESSAGES[0].0.to_owned(), renderer.clone());
        let mut button = Button::new("Next".to_owned(), renderer);

        let mut text_size = size.clone();
        text_size.resize(size.width() - 8, size.height() - 8);
        text_size.center_on(size.center());

        text.size(text_size.width(), text_size.height());
        text.position(text_size.x, text_size.y);

        button.size(250, 40);

        return Self {
            text,
            button,
            rect: size,
            index: 0,
        }
    }

    pub fn refresh(&mut self) {
        if self.index >= MESSAGES.len() { return };
        self.text.label = MESSAGES[self.index].0.to_owned();
        self.text.render_texture();
    }
}

impl Element for Educator {
    fn size(&mut self, width: u32, height: u32) {}
    fn position(&mut self, x: i32, y: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas) {
        self.text.render(canvas);
        self.button.position(32, 48 + Text::height_of(&self.text.label) as i32);

        self.button.render(canvas);
    }

    fn update(&mut self, event: &Event) {
        self.button.update(event);
    }

    fn rect(&mut self) -> &mut Rect {
        return &mut self.rect;
    }

    fn dynamic(&mut self) -> &mut Dynamic {
        return self;
    }
}