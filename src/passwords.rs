extern crate clipboard;

use self::clipboard::ClipboardContext;
use self::clipboard::ClipboardProvider;
use std::{thread, time};

use option_parser::Options;
use diceware_info::DicewareInfo;

pub fn generate_diceware_passwords(options: &Options,
                                   diceware_repository: Vec<DicewareInfo>)
 -> String {

    let password = generate_combined_password(options, diceware_repository);
    if options.clipboard {
        copy_to_clipboard(password.clone());
    }
    password
}


fn get_diceware_info_by_language(language: &str, diceware_repository: &Vec<DicewareInfo>) -> DicewareInfo {
    match language.to_lowercase().as_str() {
        "pl" => diceware_repository.iter().find(|di| di.language == "pl".to_string()).unwrap().clone(),
        _ => diceware_repository.iter().find(|di| di.language == "en".to_string()).unwrap().clone()
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


#[cfg(test)]
mod passwords_tests {
    use super::*;
    use super::DicewareInfo;

    fn build_fake_diceware_repository() -> Vec<DicewareInfo> {
        let mut fake_diceware_repository : Vec<DicewareInfo> = Vec::new();
        let polish_diceware_info = DicewareInfo {
            language: "pl".to_string(),
            num_dices: 1,
            words: vec!["polish_word1".to_string(), "polish_word2".to_string()]
        };
        let english_diceware_info = DicewareInfo {
            language: "en".to_string(),
            num_dices: 1,
            words: vec!["english1".to_string(), "english2".to_string()]
        };
        fake_diceware_repository.push(polish_diceware_info);
        fake_diceware_repository.push(english_diceware_info);

        fake_diceware_repository
    }

    #[test]
    fn proper_dicewareinfo_should_be_returned() {

        let diceware_repository = build_fake_diceware_repository();
        let expected_polish = "pl";
        let expected_english = "en";

        let di_en = get_diceware_info_by_language(expected_english, &diceware_repository);
        let di_pl = get_diceware_info_by_language(expected_polish, &diceware_repository);
        assert_eq!(di_en.language, expected_english.to_string());
        assert_eq!(di_pl.language, expected_polish);

    }

    #[test]
    fn return_english_diceware_info_by_default() {
        let diceware_repository = build_fake_diceware_repository();
        let invalid_language = "xk";

        let di = get_diceware_info_by_language(invalid_language, &diceware_repository);
        assert_eq!(di.language, "en".to_string());
    }

    #[test]
    fn passwords_copy_to_clipboard() {
        let initial = "initial".to_string();
        let expected = "newstring".to_string();
        copy_to_clipboard(initial);
        copy_to_clipboard(expected.clone());

        let mut ctx: clipboard::ClipboardContext = ClipboardProvider::new().unwrap();
        let retrieved = ctx.get_contents().unwrap();
        assert_eq!(retrieved, expected);
    }
}
