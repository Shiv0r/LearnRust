use rand::prelude::*;
use std::io::{self, Write};

const MAX_NUMBER: u8 = 10;

#[derive(Debug)]
struct State {
    name: String,
    number: u8,
}

enum UserData {
    Pc(State),
    Npc(State),
    HiddenNumber(State),
}

enum Role {
    Pc,
    Npc,
    HiddenNumber,
}

impl UserData {
    fn process_winner(&self) {
        match self {
            UserData::Npc(state) => println!("{} won with the number {}", state.name, state.number),
            UserData::Pc(state) => {
                println!("{}, you won with the number {}", state.name, state.number)
            }
            UserData::HiddenNumber(state) => println!("{} is the hidden number!", state.number),
        }
    }
    fn process_loser(&self) {
        match self {
            UserData::Pc(state) => {
                println!("{}, you lost. with the number {}", state.name, state.number)
            }
            UserData::Npc(state) => {
                println!("{} lost with the number {}", state.name, state.number)
            }
            UserData::HiddenNumber(_) => print!(""),
        }
    }

    fn get_number(&self) -> u8 {
        match self {
            UserData::Npc(state) => state.number,
            UserData::Pc(state) => state.number,
            UserData::HiddenNumber(state) => state.number,
        }
    }
}

fn get_pc_name() -> String {
    loop {
        print!("Please enter your name: ");
        io::stdout().flush().unwrap();

        let mut pc_name_input = String::new();

        io::stdin().read_line(&mut pc_name_input).unwrap();

        match pc_name_input.parse::<String>() {
            Ok(input) => {
                if !input.trim().is_empty() {
                    return input.trim().to_string();
                } else {
                    println!("Please enter a valid name");
                }
            }
            Err(_) => {
                println!("Please enter a valid name");
            }
        }
    }
}

fn get_pc_number() -> u8 {
    let mut pc_number: u8 = 0;
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut pc_number_input = String::new();
        io::stdin().read_line(&mut pc_number_input).unwrap();

        if pc_number_input.trim().to_lowercase() == "exit" {
            break;
        }

        match pc_number_input.trim().parse::<u8>() {
            Ok(num) => {
                if num <= MAX_NUMBER {
                    pc_number = num;
                    break;
                } else {
                    println!("Please enter a valid number");
                }
            }
            Err(_) => {
                println!("Please enter a valid number");
            }
        }
    }
    pc_number
}

fn set_pc_data() -> UserData {
    let pc_name_input: String = get_pc_name();
    let pc_number_input: u8 = get_pc_number();

    UserData::Pc(State {
        name: pc_name_input,
        number: pc_number_input,
    })
}

fn set_data(role: Role, mut rng: ThreadRng) -> UserData {
    match role {
        Role::Pc => set_pc_data(),
        Role::Npc => UserData::Npc(State {
            name: "Computer".to_string(),
            number: rng.random_range(1..=MAX_NUMBER),
        }),
        Role::HiddenNumber => UserData::HiddenNumber(State {
            name: "hidden number".to_string(),
            number: rng.random_range(1..=MAX_NUMBER),
        }),
    }
}

fn select_winner(pc: UserData, npc: UserData, hidden_number: UserData) {
    let dist_pc = u8::abs_diff(hidden_number.get_number(), pc.get_number());
    let dist_npc = u8::abs_diff(hidden_number.get_number(), npc.get_number());

    if dist_npc < dist_pc {
        hidden_number.process_winner();
        npc.process_winner();
        pc.process_loser();
    } else {
        hidden_number.process_winner();
        pc.process_winner();
        npc.process_loser();
    }
}

fn main() {
    let rng = rand::rng();

    println!("Guess Game!\nTry to guess the hidden number against the computer!");
    let pc = set_data(Role::Pc, rng.clone());
    let npc = set_data(Role::Npc, rng.clone());
    let hidden_number = set_data(Role::HiddenNumber, rng.clone());

    select_winner(pc, npc, hidden_number);
}
