extern crate streamdeck;
use streamdeck::{StreamDeck};

extern crate elgato_keylight;

use crate::config::Config;
use std::error::Error;
use std::thread;
use std::time;

mod action;
mod bg;
mod buttons;
mod config;
mod dummy;
mod key;
mod mic;
mod shell;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut deck = match StreamDeck::connect(0x0fd9, 0x6d, None) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to connect to a streamdeck {:?}", e);
            std::process::exit(1);
        }
    };

    match deck.version() {
        Ok(v) => println!("Version is: {}", v),
        Err(e) => {
            eprintln!("Unable to connect to a streamdeck {:?}", e);
            std::process::exit(1);
        }
    };

    let config = Config::new();

    thread::spawn(move || {
        buttons::handle(deck, config);
    });

    thread::sleep(time::Duration::from_secs(60));

    Ok(())
}
