use crate::user::*;

use std::io::{self};

enum TypeSelector<'a> {
    U8Select(String),
    U16Select(String),
    StringSelect(&'a mut String),
    U8Set(u8),
    U16Set(u16),
    StringSet(String),
    ErrMessage(&'a str),
}

impl TypeSelector<'_> {
    fn u8_validate<'a>(self, message: &'a str) -> TypeSelector<'_> {
        if let TypeSelector::U8Select(input_data) = self {
                match input_data.trim().parse::<u8>() {
                    Ok(num) => TypeSelector::U8Set(num),
                    Err(_) => TypeSelector::ErrMessage(message),
                }
        } else {
            TypeSelector::ErrMessage("<Err> Unable to parse value to u8. Programm will close.")
        }
    }
    fn u16_validat<'a>(self, message: &'a str) -> TypeSelector<'_> {
        if let TypeSelector::U16Select(input_data) = self {
                match input_data.trim().parse::<u16>() {
                    Ok(num) => TypeSelector::U16Set(num),
                    Err(_) => TypeSelector::ErrMessage(message),
                }
        } else {
            TypeSelector::ErrMessage("<Err> Unable to parse value to u16. Programm will close.")
        }
    }
    fn string_validate<'a>(self, message: &'a str) -> TypeSelector<'_> {
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


}

fn input_data_string(message: String, input_data:  &mut String) -> &String {
    loop {
         let input = io::stdin();

        println!("{}", message);
        input.read_line(input_data).unwrap();

        if input_data.trim().to_lowercase() == "exit" {
            break;
        }
        
        let mut input_data_string = TypeSelector::StringSelect(input_data);
        input_data_string = input_data_string.string_validate("Your Input seems to be empty.");






    }
    input_data
}