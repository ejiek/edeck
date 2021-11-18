use streamdeck::{Colour, ImageOptions};

pub struct State {
    pub image: Option<(String, ImageOptions)>,
    pub colour: Option<Colour>,
    pub brightness: Option<u8>,
}
