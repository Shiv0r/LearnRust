use crate::fomular::*;
use std::io::*;

const INSTRUCTIONS: [&str; 3] = ["create", "edit", "delete"];

pub fn select_instruction() {
    let instruction_number = input_selector().unwrap();

    match instruction_number {
        1 => create_fomular(),
        2 => return, // edit
        3 => return, // delete
        _ => return,
    }
}

fn input_selector() -> Option<u8> {
    let mut input_raw = String::new();
    let input = stdin();
    let instructions_length = INSTRUCTIONS.len() as u8;

    loop {
        display_instructions();
        print!("Please select the instruction do you want to execute for the contact fomular: ");

        stdout().flush().unwrap();
        input.read_line(&mut input_raw).unwrap();

        if input_raw.trim().to_lowercase() == "exit" {
            return None;
        }

        if let Ok(converted_input) = input_raw.trim().parse::<u8>() {
            if converted_input > instructions_length || converted_input <= 0 {
                input_raw.clear();
                println!(
                    "Please enter a valid number in range of 1-{}.",
                    instructions_length
                );
                continue;
            }

            return Some(converted_input);
        } else {
            input_raw.clear();
            println!(
                "Please enter a valid number in range of 1-{}.",
                instructions_length
            );
        }
    }
}

fn display_instructions() {
    for (index, &instruction) in INSTRUCTIONS.iter().enumerate() {
        println!("[{}] - ({})", instruction, index + 1);
    }
}
