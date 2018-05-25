pub mod dices;
pub mod option_parser;
pub mod diceware_info;
pub mod passwords;


// this function was extracted only to make it possible to test
// the whole solution
pub fn generate_diceware_passwords(args: Vec<String>) -> String {
    let options = self::option_parser::parse_command_line(args);

    #[cfg(debug_assertions)]
        println!("Options: {:?}", options);

    let diceware_repository = self::diceware_info::build_diceware_repository();

    passwords::generate_diceware_passwords(&options, diceware_repository)
}