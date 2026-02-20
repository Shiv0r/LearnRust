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

fn get_pc_data() -> User {
    println!("Please enter your name: ");
    let mut pc_name_input = String::new();

    io::stdin()
        .read_line(&mut pc_name_input)
        .expect("Please enter a valid name");

    println!("Please input your guess:");
    let mut pc_number_input = String::new();

    io::stdin().read_line(&mut pc_number_input).unwrap();

    let pc_number: u8 = pc_number_input
        .trim()
        .parse()
        .expect("Please enter a valid name");

    User::Pc(State {
        name: pc_name_input.trim().to_string(),
        number: pc_number,
    })
}

fn main() {
    let pc = get_pc_data();
}
