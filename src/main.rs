use std::env;
extern crate dpg;
#[macro_use]

pub mod macros;
pub mod dices;
pub mod diceware_info;
pub mod option_parser;
pub mod passwords;

fn main() {
    let args: Vec<String> = env::args().collect();

    let passwords = dpg::generate_diceware_passwords(args);

    // this should be printed probably only with "-verbose"
    println!("generated password(s):\n{}", passwords);
}
