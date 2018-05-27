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

    words.join(&options.separator)
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
        vec![
            DicewareInfo {
                language: s!("pl"),
                num_dices: 1,
                words: vec![s!("pl-1"), s!("pl-2"), s!("pl-3"), s!("pl-4"), s!("pl-5"), s!("pl-6")]
            },
            DicewareInfo {
                language: s!("en"),
                num_dices: 1,
                words: vec![s!("en-1"), s!("en-2"), s!("en-3"), s!("en-4"), s!("en-5"), s!("en-6")]
            }
        ]
    }

    #[test]
    fn proper_diceware_info_should_be_returned() {

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
    fn generate_single_word_should_return_one_word_in_specified_language() {
        let diceware_repository = build_fake_diceware_repository();

        let word = get_random_word("pl", &diceware_repository);
        assert!(word.len() > 0);
        assert!(word.starts_with("pl"));
    }

    #[test]
    fn generate_single_password_should_return_password_consisting_of_specified_number_of_words() {
        let diceware_repository = build_fake_diceware_repository();

        let options = Options {
            language : s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length: 2,
            separator: s!(" ")
        };
        let password = generate_single_password(&options, &diceware_repository);
        assert!(password.len() > 0);
//        let words = password.split_whitespace();
    }

    #[test]
    fn dash_is_used_as_default_separator() {
        let diceware_repository = build_fake_diceware_repository();

        let password_length: usize = 4;
        let options = Options {
            language : s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length: password_length,
            separator: s!("")
        };

        let password = generate_single_password(&options, &diceware_repository);
        assert!(password.contains("-"));
    }

    #[test]
    fn specified_separator_should_be_used() {
        let diceware_repository = build_fake_diceware_repository();

        let password_length: usize = 4;
        let separator = s!("*");
        let options = Options {
            language : s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length: password_length,
            separator: separator
        };

        let password = generate_single_password(&options, &diceware_repository);
        assert!(password.contains(&options.separator));
    }

    #[test]
    fn dpg_should_generate_specified_number_of_passwords_at_once() {
        let diceware_repository = build_fake_diceware_repository();
        let expected_passwords_count: usize = 5;
        let options = Options {
            language : s!("pl"),
            clipboard: false,
            help: false,
            password_count: expected_passwords_count,
            password_length: 1,
            separator: s!("")
        };

        let password = generate_combined_password(&options, diceware_repository);
        let passwords_count = password.lines().count();
        assert_eq!(passwords_count, expected_passwords_count);
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
