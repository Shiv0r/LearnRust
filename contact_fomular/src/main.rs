mod contact;
mod db;
mod fomular;
mod input;
mod user;

use crate::contact::*;
use crate::fomular::*;
use crate::input::*;
use crate::user::*;

fn main() {
    welcome_message();
    fill_fomular();
    bye_message();
}
