use crate::action::Action;

pub struct Key {
    pub position: u8,
    pub on_press: Box<dyn Action>,
    //on_hold: Option<Action>,
    //on_release: Option<Action>,
}
