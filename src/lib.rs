#[macro_use]
pub mod macros;
pub mod dices;
pub mod diceware_info;
pub mod option_parser;
pub mod passwords;


/// This should be not used directly. It has been created only for the sake of running tests.
pub fn generate_diceware_passwords(options: option_parser::Options) -> String {
    let diceware_repository = self::diceware_info::build_diceware_repository();

    passwords::generate_diceware_passwords(&options, diceware_repository)
}
