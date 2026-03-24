mod contact;
mod db;
mod fomular;
mod input;
mod selector;
mod user;

use crate::contact::*;
use crate::fomular::*;
use crate::input::*;
use crate::selector::*;
use crate::user::*;

fn main() {
    welcome_message();
    select_instruction();
    bye_message();
}
