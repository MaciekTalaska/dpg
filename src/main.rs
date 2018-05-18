extern crate dpg;
extern crate clipboard;

use dpg::DicewareInfo;
use std::env; 
//use std::collections::HashMap;
//use std::process;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::{thread, time};

use option_parser::Options;
pub mod option_parser;

fn get_diceware_info_by_language(language: &str, diceware_data: &Vec<DicewareInfo>) -> DicewareInfo {
    match language.to_lowercase().as_str() {
        "pl" => diceware_data[1].clone(),
        _ => diceware_data[0].clone()
    }
}

fn get_random_word(language: &str, diceware_data: &Vec<DicewareInfo>) -> String {
    let info: DicewareInfo = get_diceware_info_by_language(language, diceware_data.as_ref());

    #[cfg(debug_assertions)]
    println!("number of dice rolls: {:?}", info.num_dices);

    let result = dpg::mrandom::roll_dices(info.num_dices);

    #[cfg(debug_assertions)]
    println!("index: {:?}", result);
    println!("selected word: {}", info.words[result as usize % info.words.len()]);

    info.words[result as usize % info.words.len()].clone()
}



fn generate_single_password(options: &Options, diceware_data: &Vec<DicewareInfo>) -> String {
    let mut words: Vec<String> = Vec::new();
    for _i in {0..options.password_length} {
        let mut w = get_random_word(&options.language[..], diceware_data);
        words.push(w);
    }
    let password = words.join(&options.separator);
    password
}

fn generate_passwords(options: &Options, diceware_data: Vec<DicewareInfo>) -> String {
    let mut p: Vec<String> = Vec::<String>::new();
    for _i in { 0..options.password_count } {
        let new_password = generate_single_password(&options, &diceware_data);
        p.push(new_password);
    }
    p.join("\n")
}

fn copy_to_clipboard(password: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let _clipboard_result = ctx.set_contents(password.to_owned());
    // wait for 100ms, so clipboard holds the content after process ends
    thread::sleep(time::Duration::from_millis(100));
}

pub fn generate(args: Vec<String>) {

    let options = option_parser::parse_command_line(args);
    #[cfg(debug_assertions)]
        println!("Options: {:?}", options);
    //option_parser::validate_options(&options);

    let diceware_repository = dpg::read_all_diceware_lists();
    let password = generate_passwords(&options, diceware_repository);
    if options.clipboard {
        copy_to_clipboard(password.clone());
    }
    println!("generated password(s):\n{}", password);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    generate(args);
}
