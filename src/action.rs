use crate::state::State;
use std::error::Error;

pub trait Action {
    fn exec(&self) -> Result<State, Box<dyn Error>>;
    fn name(&self) -> String;
}
