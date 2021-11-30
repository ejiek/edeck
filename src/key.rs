use crate::action::Action;
use crate::state::State;

pub struct Key {
    pub position: u8,
    pub on_press: Box<dyn Action + Send>,
    pub initial_state: State,
    //pub status_check: Option<Box <dyn Action>>,
    //on_hold: Option<Action>,
    //on_release: Option<Action>,
}
