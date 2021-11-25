extern crate streamdeck;
use streamdeck::{Error as DeckError, ImageOptions, StreamDeck};

extern crate elgato_keylight;
use elgato_keylight::KeyLight;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
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
    println!("image size is: {:?}", deck.image_size());
    deck.set_button_file(
        2,
        "/home/ejiek/.streamdeck/icons/windows.png",
        &ImageOptions::new(None, false),
    );

    thread::spawn(move || {
        buttons::handle(deck);
    });

    thread::sleep(time::Duration::from_secs(60));

    Ok(())
}
