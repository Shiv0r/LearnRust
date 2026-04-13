use std::io::Write;
use std::io::stdout;

use crate::db::Db;
use crate::db::get_user;
use crate::db::select_user;
use crate::input::*;
use crate::user::*;
use uuid::Uuid;

enum Mode {
    Create(UserData),
    Edit(UserData),
}

pub fn create_fomular() {
    let mut user = UserData::new();

    user = fill_fomular(Mode::Create(user));
    let user_name = user.get_data_name().to_string();

    let db_user = Db::User(user);
    db_user.save();

    println!("The user {} was created.", user_name);
}

pub fn edit_fomular() {
    let user_id = select_user();
    let user = get_user(user_id);

    let db_user = Db::User(user);
    let target_user = db_user.delete();

    let edited_user = fill_fomular(Mode::Edit(target_user));
    let user_name = edited_user.get_data_name().to_string();
    let new_db_user = Db::User(edited_user);
    new_db_user.save();

    println!(
        "The user {} was edited and saved to the database.",
        user_name
    );
}

pub fn delete_entry() {
    let user_id = select_user();
    let user = get_user(user_id);

    let db_user = Db::User(user);
    let removed_user = db_user.delete();

    println!("The user {} was deleted.", removed_user.get_data_name());
}

pub fn show_entry() {
    let user_id = select_user();
    let user = get_user(user_id);

    print_user(user);
}

fn print_user(user: UserData) {
    let user_infos = [
        DataSelector::Name(user.get_data_name()),
        DataSelector::Age(*user.get_data_age()),
        DataSelector::Street(user.get_data_street()),
        DataSelector::PostalCode(*user.get_data_postal_code()),
        DataSelector::City(user.get_data_city()),
        DataSelector::Country(user.get_data_country()),
    ];

    println!("\n\n");
    for info in user_infos {
        println!("{:?}", info);
    }
    println!("\n\n");
}

fn fill_fomular(mode: Mode) -> UserData {
    match mode {
        Mode::Create(mut user) => {
            fill_name(&mut user);
            fill_age(&mut user);
            fill_street(&mut user);
            fill_postal_code(&mut user);
            fill_city(&mut user);
            fill_country(&mut user);

            return user;
        }
        Mode::Edit(mut user) => {
            println!("Current name: {} | Edit to:", user.get_data_name());
            fill_name(&mut user);

            println!("Current age: {} | Edit to:", user.get_data_age());
            fill_age(&mut user);

            println!("Current street: {} | Edit to:", user.get_data_street());
            fill_street(&mut user);

            println!(
                "Current postal code: {} | Edit to:",
                user.get_data_postal_code()
            );
            fill_postal_code(&mut user);

            println!("Current city: {} | Edit to:", user.get_data_city());
            fill_city(&mut user);

            println!("Current country: {} | Edit to:", user.get_data_country());
            fill_country(&mut user);

            return user;
        }
    }
}

fn generateId(user: &mut UserData) {
    user.set_data(DataSelector::Id(Uuid::new_v4()));
}

fn fill_name(user: &mut UserData) {
    let mut input_name = String::new();
    let message_name = "Please enter your name(for- and lastname): ";
    let name = input_data_string(message_name, &mut input_name);
    user.set_data(DataSelector::Name(name.as_deref().unwrap().trim()));
}

fn fill_age(user: &mut UserData) {
    let mut input_age: String = String::new();
    let message_age = "Please enter your age: ";
    let age = input_data_u8(message_age, &mut input_age);
    user.set_data(DataSelector::Age(age.unwrap()));
}

fn fill_street(user: &mut UserData) {
    let mut input_street = String::new();
    let message_street = "Please enter your street and street number: ";
    let street = input_data_string(message_street, &mut input_street);
    user.set_data(DataSelector::Street(street.as_deref().unwrap().trim()));
}

fn fill_city(user: &mut UserData) {
    let mut input_city = String::new();
    let message_city = "Please enter your city: ";
    let city = input_data_string(message_city, &mut input_city);
    user.set_data(DataSelector::City(city.as_deref().unwrap().trim()));
}

fn fill_postal_code(user: &mut UserData) {
    let mut input_postal_code = String::new();
    let message_postal_code = "Please enter your postal_code: ";
    let postal_code = input_data_u16(message_postal_code, &mut input_postal_code);
    user.set_data(DataSelector::PostalCode(postal_code.unwrap()));
}

fn fill_country(user: &mut UserData) {
    let mut input_country = String::new();
    let message_country = "Please enter your country: ";
    let country = input_data_string(message_country, &mut input_country);
    user.set_data(DataSelector::Country(country.as_deref().unwrap().trim()));
}
