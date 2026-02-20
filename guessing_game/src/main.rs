use rand::prelude::*;
use std::io;

#[derive(Debug)]
struct State {
    name: String,
    number: u8,
}

enum User {
    Pc(State),
    Npc(State),
    HiddenNumber(State),
}

enum Role {
    Npc,
    HiddenNumber,
}

impl User {
    fn process_winner(&self) {
        match self {
            User::Npc(state) => println!("{} won with the number {}", state.name, state.number),
            User::Pc(state) => println!("{}, you won with the number {}", state.name, state.number),
            User::HiddenNumber(state) => println!("{} is the hidden number!", state.name),
        }
    }

    fn get_number(&self) -> u8 {
        match self {
            User::Npc(state) => state.number,
            User::Pc(state) => state.number,
            User::HiddenNumber(state) => state.number,
        }
    }

    fn get_name(&self) -> String {
        match self {
            User::Npc(state) => state.name.to_string(),
            User::Pc(state) => state.name.to_string(),
            User::HiddenNumber(state) => state.name.to_string(),
        }
    }
}

fn get_pc_name() -> String {
    let mut pc_name: String = String::new();
    loop {
        println!("Please enter your name: ");
        let mut pc_name_input = String::new();

        io::stdin().read_line(&mut pc_name_input);

        match pc_name_input.parse::<String>() {
            Ok(s) => {
                pc_name = s;
                break;
            }
            Err(_) => {
                println!("Please enter a valid name");
            }
        }
    }

    pc_name
}

fn get_pc_number() -> u8 {
    let mut pc_number: u8 = 0;
    loop {
        println!("Please input your guess:");
        let mut pc_number_input = String::new();
        io::stdin().read_line(&mut pc_number_input).unwrap();

        if pc_number_input.trim().to_lowercase() == "exit" {
            break;
        }

        match pc_number_input.trim().parse::<u8>() {
            Ok(num) => {
                if num > 10 {
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

fn set_pc_data() -> User {
    let pc_name_input: String = get_pc_name();
    let pc_number_input: u8 = get_pc_number();

    User::Pc(State {
        name: pc_name_input,
        number: pc_number_input,
    })
}

fn set_data(role: Role, max_number: u8, mut rng: ThreadRng) -> User {
    match role {
        Role::Npc => User::Npc(State {
            name: "Computer".to_string(),
            number: rng.random_range(1..=max_number),
        }),
        Role::HiddenNumber => User::Npc(State {
            name: "Computer".to_string(),
            number: rng.random_range(1..=max_number),
        }),
    }
}

fn main() {
    let max_number: u8 = 10;
    let rng = rand::rng();

    let pc = set_pc_data();
    let npc = set_data(Role::Npc, max_number, rng);
    let guess_number = set_data(Role::HiddenNumber, max_number, rng);
}
