use streamdeck::{Colour, ImageOptions};

pub struct State {
    pub image: Option<(String, ImageOptions)>,
    pub colour: Option<Colour>,
    pub brightness: Option<u8>,
}

impl State {
    pub fn default() -> State {
        State {
            image: None,
            colour: Some(Colour {
                r: 40,
                g: 40,
                b: 40,
            }),
            brightness: None,
        }
    }
}
