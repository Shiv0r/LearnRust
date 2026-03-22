use crate::input::*;
use crate::user::*;

pub fn fill_fomular() {
    let mut user = UserData::new();

    fill_name(&mut user);
    fill_age(&mut user);

    fill_street(&mut user);
    fill_postal_code(&mut user);
    fill_city(&mut user);
    fill_country(&mut user);
}

fn fill_name(mut user: &mut UserData) {
    let mut input_name = String::new();
    let message_name = "Please enter your name(for- and lastname): ";
    let name = input_data_string(message_name, &mut input_name);
    user.set_data(DataSelector::Name(name.as_deref().unwrap()));
}

fn fill_age(mut user: &mut UserData) {
    let mut input_age: String = String::new();
    let message_age = "Please enter your age: ";
    let age = input_data_u8(message_age, &mut input_age);
    user.set_data(DataSelector::Age(age.unwrap()));
}

fn fill_street(mut user: &mut UserData) {
    let mut input_street = String::new();
    let message_street = "Please enter your street and street number: ";
    let street = input_data_string(message_street, &mut input_street);
    user.set_data(DataSelector::Street(street.as_deref().unwrap()));
}

fn fill_city(mut user: &mut UserData) {
    let mut input_city = String::new();
    let message_city = "Please enter your city: ";
    let city = input_data_string(message_city, &mut input_city);
    user.set_data(DataSelector::City(city.as_deref().unwrap()));
}

fn fill_postal_code(mut user: &mut UserData) {
    let mut input_postal_code = String::new();
    let message_postal_code = "Please enter your postal_code: ";
    let postal_code = input_data_u16(message_postal_code, &mut input_postal_code);
    user.set_data(DataSelector::PostalCode(postal_code.unwrap()));
}

fn fill_country(mut user: &mut UserData) {
    let mut input_country = String::new();
    let message_country = "Please enter your country: ";
    let country = input_data_string(message_country, &mut input_country);
    user.set_data(DataSelector::Country(country.as_deref().unwrap()));
}
