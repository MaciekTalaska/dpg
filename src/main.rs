use std::env;
extern crate dpg;

fn main() {
    let args: Vec<String> = env::args().collect();

    let passwords = dpg::generate_diceware_passwords(args);

    // this should be printed probably only with "-verbose"
    println!("generated password(s):\n{}", passwords);
}
