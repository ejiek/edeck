use crate::action::Action;
use crate::bg::*;
use crate::mic;
use crate::shell;
use crate::state::State;
use std::convert::TryInto;
use streamdeck::{Colour, Error as DeckError, ImageOptions, StreamDeck};

pub fn handle(mut deck: StreamDeck) {
    let mut previous_states: Vec<u8> = vec![0; 15];
    loop {
        match deck.read_buttons(None) {
            Ok(states) => {
                println!("{:?}", &states);
                states
                    .iter()
                    .enumerate()
                    .zip(previous_states.iter_mut())
                    .for_each(|((key, current_state), mut previous_state)| {
                        match (&previous_state, current_state) {
                            (0, 1) => match key {
                                0 => {
                                    match shell::new(vec![
                                        String::from("/home/ejiek/.xmonad/scripts/volume.sh"),
                                        String::from("down"),
                                    ])
                                    .exec()
                                    {
                                        Ok(state_update) => {
                                            update_state(&mut deck, key, state_update)
                                        }
                                        Err(e) => println!("Error during action: {:?}", e),
                                    }
                                }
                                14 => match mic::default().exec() {
                                    Ok(state_update) => update_state(&mut deck, key, state_update),
                                    Err(e) => println!("Error during action: {:?}", e),
                                },
                                _ => match green().exec() {
                                    Ok(state_update) => update_state(&mut deck, key, state_update),
                                    Err(e) => println!("Error during action: {:?}", e),
                                },
                            },
                            (1, 0) => match key {
                                14 => {}
                                _ => match black().exec() {
                                    Ok(state_update) => update_state(&mut deck, key, state_update),
                                    Err(e) => println!("Error during action: {:?}", e),
                                },
                            },
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

fn update_state(deck: &mut StreamDeck, key: usize, state: State) {
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
