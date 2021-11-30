use crate::action::Action;
use crate::state::State;
use std::error::Error;
use std::process::Command;
use streamdeck::Colour;

#[derive(Clone)]
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

    fn sys_toggle(&self) {
        Command::new("pactl")
            .arg("set-source-mute")
            .arg(&self.PA_device)
            .arg("toggle")
            .output()
            .expect("kek");
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

    pub fn get_toggle_action(&self) -> Toggle {
        Toggle { mic: self.clone() }
    }
}

pub struct Toggle {
    mic: Mic,
}

impl Action for Toggle {
    fn exec(&self) -> Result<State, Box<dyn Error>> {
        self.mic.sys_toggle();

        if self.mic.is_mute() {
            Ok(State {
                image: None,
                colour: Some(Colour {
                    r: 204,
                    g: 36,
                    b: 29,
                }),
                brightness: None,
            })
        } else {
            Ok(State {
                image: None,
                colour: Some(Colour {
                    r: 184,
                    g: 187,
                    b: 38,
                }),
                brightness: None,
            })
        }
    }

    fn name(&self) -> String {
        String::from("Toggle Microphone")
    }
}

pub struct Mute {
    mic: Mic,
}

pub struct Unmute {
    mic: Mic,
}
