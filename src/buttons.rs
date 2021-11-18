use crate::action::Action;
use crate::bg::*;
use crate::state::State;
use std::convert::TryInto;
use streamdeck::{Colour, Error as DeckError, ImageOptions, StreamDeck};

pub fn handle(mut deck: StreamDeck) {
    loop {
        match deck.read_buttons(None) {
            Ok(states) => {
                println!("{:?}", &states);

                states.iter().enumerate().for_each(|(key, state)| {
                    if *state == 1 {
                        match key.try_into() {
                            Ok(key) => match green().exec() {
                                Ok(state_update) => update_state(&mut deck, key, state_update),
                                Err(e) => println!("Error during action: {:?}", e),
                            },
                            Err(e) => println!("Unable to parse key value: {:?}", e),
                        }
                    } else {
                        match key.try_into() {
                            Ok(key) => match black().exec() {
                                Ok(state_update) => update_state(&mut deck, key, state_update),
                                Err(e) => println!("Error during action: {:?}", e),
                            },
                            Err(e) => println!("Unable to parse key value: {:?}", e),
                        }
                    }
                })
            }
            Err(e) => println!("Error while handling a key press: {:?}", e),
        }
    }
}

fn update_state(deck: &mut StreamDeck, key: u8, state: State) {
    match state.image {
        Some((path, options)) => {
            println!(
                "Setting image is not supported yer! Hold on!\n\t img: {}, options {:?}",
                path, options
            )
        }
        None => {}
    };
    match state.colour {
        Some(colour) => {
            deck.set_button_rgb(key, &colour);
        }
        None => {}
    };
}