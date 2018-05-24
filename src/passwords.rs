extern crate clipboard;

use self::clipboard::ClipboardContext;
use self::clipboard::ClipboardProvider;
use std::{thread, time};

use option_parser::Options;
use diceware_info::DicewareInfo;

pub fn generate_diceware_passwords(args: Vec<String>) {
    let options = super::option_parser::parse_command_line(args);

    #[cfg(debug_assertions)]
        println!("Options: {:?}", options);

    let diceware_repository = super::diceware_info::read_all_diceware_lists();
    let password = generate_combined_password(&options, diceware_repository);
    if options.clipboard {
        copy_to_clipboard(password.clone());
    }
    println!("generated password(s):\n{}", password);
}

fn get_diceware_info_by_language(language: &str, diceware_repository: &Vec<DicewareInfo>) -> DicewareInfo {
    match language.to_lowercase().as_str() {
        "pl" => diceware_repository[1].clone(),
        _ => diceware_repository[0].clone()
    }
}

fn get_random_word(language: &str, diceware_repository: &Vec<DicewareInfo>) -> String {
    let info: DicewareInfo = get_diceware_info_by_language(language, diceware_repository.as_ref());

    #[cfg(debug_assertions)]
    println!("number of dice rolls: {:?}", info.num_dices);

    let result = super::dices::roll_dices(info.num_dices);

    #[cfg(debug_assertions)]
    println!("index: {:?}", result);
    println!("selected word: {}", info.words[result as usize % info.words.len()]);

    info.words[result as usize % info.words.len()].clone()
}


fn generate_single_password(options: &Options, diceware_repository: &Vec<DicewareInfo>) -> String {
    let mut words: Vec<String> = Vec::new();
    for _i in {0..options.password_length} {
        let mut w = get_random_word(&options.language[..], diceware_repository);
        words.push(w);
    }
    let password = words.join(&options.separator);
    password
}


fn generate_combined_password(options: &Options, diceware_repository: Vec<DicewareInfo>) -> String {
    let mut combined_password: Vec<String> = Vec::<String>::new();
    for _i in { 0..options.password_count } {
        let new_password = generate_single_password(&options, &diceware_repository);
        combined_password.push(new_password);
    }
    combined_password.join("\n")
}


fn copy_to_clipboard(password: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let _clipboard_result = ctx.set_contents(password.to_owned());
    // wait for 100ms, so clipboard holds the content after process ends
    thread::sleep(time::Duration::from_millis(100));
}

