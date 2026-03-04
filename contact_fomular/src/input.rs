use crate::user::*;

use std::io::{self};

enum TypeSelector<'a> {
    U8Select(&'a mut String),
    U16Select(&'a mut String),
    StringSelect(&'a mut String),
    U8Set(u8),
    U16Set(u16),
    StringSet(String),
    ErrMessage(&'a str),
    Exit,
}

impl TypeSelector<'_> {
    fn u8_validate<'a>(&self, message: &'a str) -> TypeSelector<'a> {
        if let TypeSelector::U8Select(input_data) = self {
            match input_data.trim().parse::<u8>() {
                Ok(num) => TypeSelector::U8Set(num),
                Err(_) => TypeSelector::ErrMessage(message),
            }
        } else {
            TypeSelector::ErrMessage("<Err> Unable to parse value to u8. Programm will close.")
        }
    }
    fn u16_validate<'a>(&self, message: &'a str) -> TypeSelector<'a> {
        if let TypeSelector::U16Select(input_data) = self {
            match input_data.trim().parse::<u16>() {
                Ok(num) => TypeSelector::U16Set(num),
                Err(_) => TypeSelector::ErrMessage(message),
            }
        } else {
            TypeSelector::ErrMessage("<Err> Unable to parse value to u16. Programm will close.")
        }
    }
    fn string_validate<'a>(&self, message: &'a str) -> TypeSelector<'a> {
        if let TypeSelector::StringSelect(input_data) = self {
            if input_data.trim().is_empty() {
                TypeSelector::ErrMessage(message)
            } else {
                TypeSelector::StringSet(input_data.to_string())
            }
        } else {
            TypeSelector::ErrMessage("<Err> Unable to proceed. Programm will close.")
        }
    }
    fn exit_loop(&self) -> bool {
        match self {
            TypeSelector::StringSelect(input_data) => input_data.trim().to_lowercase() == "exit",
            TypeSelector::U8Select(input_data) => input_data.trim().to_lowercase() == "exit",
            TypeSelector::U16Select(input_data) => input_data.trim().to_lowercase() == "exit",
            _ => false,
        }
    }
}

fn input_data_string(message: &str, input_data: &mut String) -> Option<String> {
    let input_result;
    loop {
        let input = io::stdin();

        println!("{}", message);
        input.read_line(input_data).unwrap();

        let input_data_string: TypeSelector<'_> = TypeSelector::StringSelect(input_data);

        if input_data_string.exit_loop() {
            return None;
        }

        match input_data_string.string_validate("Your Input seems to be empty.") {
            TypeSelector::StringSet(input) => {
                input_result = input;
                break;
            }
            TypeSelector::ErrMessage(message) => {
                println!("{}", message);
                input_data.clear();
                continue;
            }
            _ => {
                input_data.clear();
                continue;
            }
        }
    }
    Some(input_result)
}

fn input_data_u8(message: &str, input_data: &mut String) -> Option<u8> {
    let input_result: u8;
    loop {
        let input = io::stdin();

        println!("{}", message);
        input.read_line(input_data).unwrap();

        let input_data_u8: TypeSelector<'_> = TypeSelector::U8Select(input_data);

        if input_data_u8.exit_loop() {
            return None;
        }

        match input_data_u8.u8_validate("Your input is not a valid number in range of 0 - 255") {
            TypeSelector::U8Set(input) => {
                input_result = input;
                break;
            }
            TypeSelector::ErrMessage(message) => {
                println!("{}", message);
                input_data.clear();
                continue;
            }
            _ => {
                input_data.clear();
                continue;
            }
        }
    }
    Some(input_result)
}

fn input_data_u16(message: &str, input_data: &mut String) -> Option<u16> {
    let input_result: u16;
    loop {
        let input = io::stdin();

        println!("{}", message);
        input.read_line(input_data).unwrap();

        let input_data_u16 = TypeSelector::U16Select(input_data);

        if input_data_u16.exit_loop() {
            return None;
        }

        match input_data_u16.u16_validate("Your input is not a valid number in range of 0 - 65.536")
        {
            TypeSelector::U16Set(input) => {
                input_result = input;
                break;
            }
            TypeSelector::ErrMessage(message) => {
                println!("{}", message);
                input_data.clear();
                continue;
            }
            _ => {
                input_data.clear();
                continue;
            }
        }
    }
    Some(input_result)
}
