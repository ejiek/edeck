use crate::action::Action;
use crate::config::Config;
use crate::state::State;
use std::convert::TryInto;
use streamdeck::{Colour, Error as DeckError, ImageOptions, StreamDeck};

pub fn handle(mut deck: StreamDeck) {
    let mut previous_states: Vec<u8> = vec![0; 15];
    let config = Config::new();
    loop {
        match deck.read_buttons(None) {
            Ok(states) => {
                println!("{:?}", &states);
                states
                    .iter()
                    .zip(previous_states.iter_mut())
                    .zip(config.keys.iter())
                    .for_each(|((current_state, previous_state), key)| {
                        match (&previous_state, current_state) {
                            (0, 1) => match key.on_press.exec() {
                                Ok(state_update) => {
                                    update_state(&mut deck, key.position, state_update)
                                }
                                Err(e) => println!("Error during action: {:?}", e),
                            },
                            (1, 0) => {
                                print!("r!")
                            }
                            (1, 1) => {
                                print!("h!")
                            }
                            (_, _) => {
                                print!("f!")
                            }
                        }
                        *previous_state = *current_state;
                    });
                println!("");
            }
            Err(e) => println!("Error while handling a key press: {:?}", e),
        }
    }
}

fn update_state(deck: &mut StreamDeck, key: u8, state: State) {
    match key.try_into() {
        Ok(key) => {
            if let Some((path, options)) = state.image {
                println!(
                    "Setting image is not supported yer! Hold on!\n\t img: {}, options {:?}",
                    path, options
                )
            };
            if let Some(colour) = state.colour {
                deck.set_button_rgb(key, &colour);
            };
        }
        Err(e) => println!("Unable to parse key value: {:?}", e),
    }
}
