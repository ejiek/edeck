use crate::dummy::Dummy;
use crate::key::Key;
use crate::mic::Mic;
use crate::shell::Shell;
use crate::state::State;

pub struct Config {
    pub keys: Vec<Key>,
}

impl Config {
    pub fn new() -> Config {
        let mut keys = Vec::new();
        keys.push(Key {
            position: 0,
            on_press: Box::new(Mic::default().get_toggle_action()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 1,
            on_press: Box::new(Shell::new(vec![
                String::from("/home/ejiek/.xmonad/scripts/volume.sh"),
                String::from("down"),
            ])),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 2,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 3,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 4,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 5,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 6,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 7,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 8,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 9,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 10,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 11,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 12,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 13,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 14,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });
        keys.push(Key {
            position: 15,
            on_press: Box::new(Dummy::new()),
            initial_state: State::default(),
        });

        Config { keys }
    }
}
