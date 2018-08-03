use std::env;
extern crate dpg;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = dpg::option_parser::parse_command_line(args);

    //let passwords = dpg::generate_diceware_passwords(args);
    let passwords = dpg::generate_diceware_passwords_new(options);

    // this should be printed probably only with "-verbose"
    println!("generated password(s):\n{}", passwords);
}
