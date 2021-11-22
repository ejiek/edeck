use crate::action::Action;
use crate::state::State;
use std::error::Error;
use streamdeck::Colour;

pub struct Bg {
    colour: Colour,
}

impl Bg {
    pub fn green() -> Bg {
        Bg {
            colour: Colour {
                r: 50,
                g: 255,
                b: 50,
            },
        }
    }

    pub fn black() -> Bg {
        Bg {
            colour: Colour { r: 0, g: 0, b: 0 },
        }
    }
}

impl Action for Bg {
    fn exec(&self) -> Result<State, Box<dyn Error>> {
        Ok(State {
            image: None,
            colour: Some(self.colour.clone()),
            brightness: None,
        })
    }

    fn name(&self) -> String {
        String::from("Set background")
    }
}
