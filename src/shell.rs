use crate::action::Action;
use crate::state::State;
use std::error::Error;
use std::process::Command;

pub struct Shell {
    command: Vec<String>,
}

impl Shell {
    pub fn new(command: Vec<String>) -> Shell {
        Shell { command }
    }
}

impl Action for Shell {
    fn exec(&self) -> Result<State, Box<dyn Error>> {
        Command::new(self.command.first().unwrap())
            .args(self.command.iter().skip(1))
            .output()
            .expect("kek");

        Ok(State {
            image: None,
            colour: None,
            brightness: None,
        })
    }

    fn name(&self) -> String {
        String::from("Shell command for")
    }
}
