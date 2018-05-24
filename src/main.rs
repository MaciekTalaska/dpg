use std::env;

pub mod dices;
pub mod option_parser;
pub mod diceware_info;
pub mod passwords;


fn main() {
    let args: Vec<String> = env::args().collect();
    let options = self::option_parser::parse_command_line(args);

    #[cfg(debug_assertions)]
        println!("Options: {:?}", options);

    let diceware_repository = self::diceware_info::read_all_diceware_lists();

    let passwords = passwords::generate_diceware_passwords(&options, diceware_repository);

    // this should be printed probably only with "-verbose"
    println!("generated password(s):\n{}", passwords);
}
