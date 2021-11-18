use crate::state::State;
use std::error::Error;
use streamdeck::StreamDeck;

pub trait Action {
    fn exec(&self) -> Result<State, Box<dyn Error>>;
}
