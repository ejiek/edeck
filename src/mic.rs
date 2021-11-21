use crate::action::Action;
use crate::state::State;
use std::error::Error;
use std::process::Command;
use streamdeck::Colour;
use streamdeck::StreamDeck;

pub struct Mic {
    PA_device: String,
}

impl Mic {
    fn is_mute(&self) -> bool {
        let mute = Command::new("sh")
            .arg("-c")
            .arg(format!{"pactl list | grep -A53 \"Name: {}\" | grep Mute | awk '{{ print $2 }}'", &self.PA_device})
            .output()
            .expect("kek");

        String::from_utf8(mute.stdout).unwrap().contains("yes")
    }

    fn toggle(&self) {
        Command::new("pactl")
            .arg("set-source-mute")
            .arg(&self.PA_device)
            .arg("toggle")
            .output()
            .expect("kek");
    }
}

impl Action for Mic {
    fn exec(&self) -> Result<State, Box<dyn Error>> {
        self.toggle();

        if self.is_mute() {
            Ok(State {
                image: None,
                colour: Some(Colour { r: 255, g: 0, b: 0 }),
                brightness: None,
            })
        } else {
            Ok(State {
                image: None,
                colour: Some(Colour {
                    r: 0,
                    g: 255,
                    b: 255,
                }),
                brightness: None,
            })
        }
    }
}

pub fn default() -> Mic {
    let info = Command::new("sh")
        .arg("-c")
        .arg("pactl info | grep \"Default Source\" | cut -d \" \" -f3")
        .output()
        .expect("kek");

    let mut default = String::from_utf8(info.stdout).unwrap();
    // getting rid of a new line
    default.truncate(default.len() - 1);

    Mic { PA_device: default }
}
