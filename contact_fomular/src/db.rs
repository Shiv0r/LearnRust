use crate::user::*;

use std::io::{self};
use std::sync::Mutex;
use uuid::Uuid;

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

    pub fn delete(self) -> UserData {
        let mut db = DB.lock().unwrap();
        match self {
            Db::User(user) => {
                is_db_empty();

                if let Some(index) = db
                    .iter()
                    .position(|db_user| user.get_data_id() == db_user.get_data_id())
                {
                    let removed_user = db.remove(index);
                    return removed_user;
                } else {
                    panic!("User not found");
                }
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

pub fn select_user() -> Uuid {
    is_db_empty();
    let db = DB.lock().unwrap();
    let input = io::stdin();
    let index;

    loop {
        let mut input_user = String::new();

        println!("Total amount of user: {}", db.len());
        println!("Pls select the user:");
        for (index, user) in db.iter().enumerate() {
            println!("[{}]-({})", user.get_data_name(), index + 1);
        }
        input.read_line(&mut input_user).unwrap();

        match input_user.trim().parse::<u32>() {
            Ok(user_index) => {
                if user_index <= db.len().try_into().unwrap() && user_index != 0 {
                    index = user_index - 1;
                    break;
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

    db[index as usize].get_data_id()
}
