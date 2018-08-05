use std::env;
extern crate dpg;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = dpg::option_parser::parse_command_line(args);

    #[cfg(debug_assertions)]
    println!("Options: {:?}", options);

    let passwords = dpg::generate_diceware_passwords(options);

    println!("generated password(s):\n{}", passwords);
}
