use std::env;

pub mod option_parser;
pub mod passwords;


fn main() {
    let args: Vec<String> = env::args().collect();

    passwords::generate_diceware_passwords(args);
}
