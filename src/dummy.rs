use crate::action::Action;
use crate::state::State;
use std::error::Error;

pub struct Dummy {}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {}
    }
}

impl Action for Dummy {
    fn exec(&self) -> Result<State, Box<dyn Error>> {
        Ok(State {
            image: None,
            colour: None,
            brightness: None,
        })
    }

    fn name(&self) -> String {
        String::from("Empty")
    }
}
