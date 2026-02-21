mod user;
mod input;

use crate::input::*;
use crate::user::*;


fn main() {
    let location = create_new_location("Bielefeld Straße 20", 10178, "Bielefeld", "Narnia");
    let mut user = create_new_user("bla", "Hand Günther", 40, location);

    let mut name = user.get_data_name().to_string();

    println!("name: {}", name);

    user.set_data(DataSelector::Name("Joe Dane"));

    name = user.get_data_name().to_string();

    println!("new name: {}", name);

}