use crate::input::*;
use crate::user::*;

pub fn fill_fomular() {
    let mut user = UserData::new();
    let mut input_name = String::new();
    let message_name = "Please enter your name: ";

    let name = input_data_string(message_name, &mut input_name);

    user.set_data(DataSelector::Name(name.as_deref().unwrap()));
}
