use crate::user::*;

use std::io::{self};
use std::sync::Mutex;

static DB: Mutex<Vec<UserData>> = Mutex::new(Vec::new());

pub enum Db {
    User(UserData),
}

impl Db {
    pub fn save(self) {
        let mut db = DB.lock().unwrap();
        match self {
            Db::User(user) => {
                db.push(user);
            }
        }
    }

    pub fn delete(self) {
        let mut db = DB.lock().unwrap();
        match self {
            Db::User(user) => {
                is_db_empty();
                db.retain(|user_db| user_db.get_data_id() != user.get_data_id());
            }
        }
    }
}

fn is_db_empty() {
    let db = DB.lock().unwrap();
    if db.len() == 0 {
        return;
    }
}

fn error_message(message: &str, mut input_user: String) {
    println!("{}", message);
    input_user.clear();
}

fn select_user() -> u32 {
    is_db_empty();
    let db = DB.lock().unwrap();
    let input = io::stdin();

    loop {
        let mut input_user = String::new();

        println!("Total amount of user: {}", db.len());
        println!("Pls select the user");
        for (index, user) in db.iter().enumerate() {
            println!("[{}]-({})", user.get_data_name(), index + 1);
        }
        input.read_line(&mut input_user).unwrap();

        match input_user.trim().parse::<u32>() {
            Ok(user_index) => {
                if user_index <= db.len().try_into().unwrap() && user_index != 0 {
                    return user_index;
                } else {
                    error_message(
                        "<Err> Please enter a valid number to select the user",
                        input_user,
                    );
                }
            }
            Err(_) => {
                error_message(
                    "<Err> Please enter a valid number to select the user",
                    input_user,
                );
            }
        }
    }
}

pub fn get_user() -> UserData {
    let user_index = select_user();
    let mut db = DB.lock().unwrap();

    db.remove((user_index - 1) as usize)
}
