extern crate clipboard;

use self::clipboard::ClipboardContext;
use self::clipboard::ClipboardProvider;
use std::{thread, time};

use diceware_info::DicewareInfo;
use option_parser::Options;


/// Main function to be called to generate passwords. It requires properly filled Options structue and repository of diceware word lists (which is collection of DicewareInfo structues).
///
/// Example of usage:
/// ```rust
///    let options : dpg::option_parser::Options = dpg::option_parser::Options {
///        language: "en".to_string(),     // use English word list
///        password_length: 6,             // 6 words per password
///        password_count: 3,              // generate 3 passwords
///        separator: "-".to_string(),     // separate words by dash ('-')
///        simulate_dices: false,          // do not simulate dice roll
///        clipboard: false,               // do not copy passwords to clipboard
///        help: false                     // do not call for help/usage
///    };
///
///    let repository = dpg::diceware_info::build_diceware_repository();
///    let passwords = dpg::passwords::generate_diceware_passwords(&options, repository);
/// ```
pub fn generate_diceware_passwords(
    options: &Options,
    diceware_repository: Vec<DicewareInfo>,
) -> String {
    let passwords = generate_all_passwords(options, diceware_repository);
    // TODO: this should probably be moved from here,
    // it is not responsibility of the library itself
    // to copy generated passwords into clipboard
    if options.clipboard {
        copy_to_clipboard(passwords.clone());
    }
    passwords
}

/// Alternative for generate_diceware_passwords
/// - does not require passing parameters as Options struct
/// - does not require passing repository (this function takes care of creating repository)
///
/// This mehod should be easier to consume, as it creates all requirements needed itself.
/// Please note that this function does not allow to:
/// - passwords to be copied to clipboard
/// - set help flag to 'true' so that the additional help on usage is printed. This is due to the fact, that this method is only intended to be consumed as third party library and be called directly from your code. Printing anything to the standard output and using clipboard should be responsibility of your own code.
///
/// Example of usage:
/// ```rust
///
///    // generating 3 passwords, using English word list, each password 6 words long. Words separated by dash ("-"):
///
///    let passwords = dpg::passwords::generate_diceware_passwords_simple(
///        "en",       // use English words list
///        6,          // 6 words per password
///        3,          // generate 3 passwords
///        "-",        // separate words by dash ('-')
///        false);     // do not simulate dice rolls
///
///    // the code above is the same as:
///
///    let options : dpg::option_parser::Options = dpg::option_parser::Options {
///        language: "en".to_string(),     // use English word list
///        password_length: 6,             // 6 words per password
///        password_count: 3,              // generate 3 passwords
///        separator: "-".to_string(),     // separate words by dash ('-')
///        simulate_dices: false,          // do not simulate dice roll
///        clipboard: false,               // do not copy passwords to clipboard
///        help: false                     // do not call for help/usage
///    };
///
///    let repository = dpg::diceware_info::build_diceware_repository();
///    let passwords = dpg::passwords::generate_diceware_passwords(&options, repository);
/// ```
pub fn generate_diceware_passwords_simple(language: &str,
                               password_length: usize,
                               passwords_count: usize,
                               separator: &str,
                               simulate_dices: bool) -> String {
    let repository = ::diceware_info::build_diceware_repository();
    let options = ::option_parser::Options {
        language : language.to_string(),
        separator : separator.to_string(),
        password_length,
        password_count : passwords_count,
        simulate_dices,
        clipboard : false,
        help : false,
    };

    let passwords = generate_all_passwords(&options, repository);
    passwords
}


pub struct PasswordsIterator {
    repository: Vec<DicewareInfo>,
    options: ::option_parser::Options,
}

impl PasswordsIterator {
    pub fn new(language: &str,
               separator: &str,
               password_length: usize,
               simulate_dices: bool) -> PasswordsIterator {
        PasswordsIterator {
            repository: ::diceware_info::build_diceware_repository(),
            options: ::option_parser::Options {
                language: language.to_string(),
                separator: separator.to_string(),
                password_length,
                password_count: 1,
                simulate_dices,
                clipboard: false,
                help: false,
            }
        }
    }
}

impl Iterator for PasswordsIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        let password = generate_single_password(&self.options, &self.repository);
        Some(password.clone())
    }
}


fn get_diceware_info_by_language(
    language: &str,
    diceware_repository: &[DicewareInfo],
) -> DicewareInfo {
    match language.to_lowercase().as_str() {
        "pl" => diceware_repository
            .iter()
            .find(|di| di.language.as_str() == "pl")
            .expect("unable to find data for language [pl]")
            .clone(),
        _ => diceware_repository
            .iter()
            .find(|di| di.language.as_str() == "en")
            .expect("unable to find data for default language [en]")
            .clone(),
    }
}

fn get_random_word(language: &str, diceware_repository: &[DicewareInfo], simulate_dices: bool) -> String {
    let info: DicewareInfo = get_diceware_info_by_language(language, diceware_repository.as_ref());

    //#[cfg(debug_assertions)]
    //println!("number of dice rolls: {:?}", info.num_dices);

    let result = match simulate_dices {
        true => super::dices::roll_dices(info.num_dices),
        false => super::dices::get_random_number(info.words.len() as u32)
    };

    //#[cfg(debug_assertions)] {
    //println!("index: {:?}", result);
    //println!(
    //    "selected word: {}",
    //    info.words[result as usize % info.words.len()]
    //);
    //}

    info.words[result as usize % info.words.len()].clone()
}

fn generate_single_password(options: &Options, diceware_repository: &[DicewareInfo]) -> String {
    let language = &options.language[..];

    let mut words: Vec<String> = Vec::with_capacity(options.password_length);
    for _i in { 0..options.password_length } {
        let word = get_random_word(language, diceware_repository, options.simulate_dices);
        words.push(word);
    }

    words.join(&options.separator)
}

fn generate_all_passwords(options: &Options, diceware_repository: Vec<DicewareInfo>) -> String {
    let mut all_passwords: Vec<String> = Vec::<String>::with_capacity(options.password_count);
    for _i in { 0..options.password_count } {
        let password = generate_single_password(&options, &diceware_repository);
        all_passwords.push(password);
    }

    all_passwords.join("\n")
}

fn copy_to_clipboard(password: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .expect("error accessing clipboard");
    let _clipboard_result = ctx.set_contents(password.to_owned());
    // wait for 100ms, so clipboard holds the content after process ends
    thread::sleep(time::Duration::from_millis(100));
}

#[cfg(test)]
mod passwords_tests {
    use super::DicewareInfo;
    use super::*;

    fn build_fake_diceware_repository() -> Vec<DicewareInfo> {
        vec![
            DicewareInfo {
                language: s!("pl"),
                num_dices: 1,
                words: vec![
                    s!("pl-1"),
                    s!("pl-2"),
                    s!("pl-3"),
                    s!("pl-4"),
                    s!("pl-5"),
                    s!("pl-6"),
                ],
            },
            DicewareInfo {
                language: s!("en"),
                num_dices: 1,
                words: vec![
                    s!("en-1"),
                    s!("en-2"),
                    s!("en-3"),
                    s!("en-4"),
                    s!("en-5"),
                    s!("en-6"),
                ],
            },
        ]
    }

    #[test]
    fn proper_diceware_info_should_be_returned() {
        let diceware_repository = build_fake_diceware_repository();
        let expected_polish = "pl";
        let expected_english = "en";

        let di_en = get_diceware_info_by_language(expected_english, &diceware_repository);
        let di_pl = get_diceware_info_by_language(expected_polish, &diceware_repository);
        assert_eq!(di_en.language, expected_english);
        assert_eq!(di_pl.language, expected_polish);
    }

    #[test]
    fn return_english_diceware_info_by_default() {
        let diceware_repository = build_fake_diceware_repository();
        let invalid_language = "xk";

        let di = get_diceware_info_by_language(invalid_language, &diceware_repository);
        assert_eq!(di.language, "en");
    }

    #[test]
    fn generate_single_word_should_return_one_word_in_specified_language() {
        let diceware_repository = build_fake_diceware_repository();

        let word = get_random_word("pl", &diceware_repository, false);
        assert!(word.len() > 0);
        assert!(word.starts_with("pl"));
    }

    #[test]
    fn generate_single_password_should_return_password_consisting_of_specified_number_of_words() {
        let diceware_repository = build_fake_diceware_repository();

        let options = Options {
            language: s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length: 2,
            separator: s!(" "),
            simulate_dices: false,
        };
        let password = generate_single_password(&options, &diceware_repository);
        let words_count = password.split_whitespace().count();
        assert_eq!(words_count, options.password_length);
    }

    #[test]
    fn dash_is_used_as_default_separator() {
        let diceware_repository = build_fake_diceware_repository();

        let password_length: usize = 4;
        let options = Options {
            language: s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length,
            separator: s!(""),
            simulate_dices: false,
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
            language: s!("pl"),
            clipboard: false,
            help: false,
            password_count: 1,
            password_length,
            separator,
            simulate_dices: false,
        };

        let password = generate_single_password(&options, &diceware_repository);
        assert!(password.contains(&options.separator));
    }

    #[test]
    fn dpg_should_generate_specified_number_of_passwords_at_once() {
        let diceware_repository = build_fake_diceware_repository();
        let expected_passwords_count: usize = 5;
        let options = Options {
            language: s!("pl"),
            clipboard: false,
            help: false,
            password_count: expected_passwords_count,
            password_length: 1,
            separator: s!(""),
            simulate_dices: false,
        };

        let password = generate_all_passwords(&options, diceware_repository);
        let passwords_count = password.lines().count();
        assert_eq!(passwords_count, expected_passwords_count);
    }

    #[test]
    fn passwords_copy_to_clipboard() {
        let initial = s!("initial");
        let expected = s!("expected");
        copy_to_clipboard(initial);
        copy_to_clipboard(expected.clone());

        let mut ctx: clipboard::ClipboardContext = ClipboardProvider::new()
            .expect("error accessing clipboard [in test]");
        let retrieved = ctx.get_contents()
            .expect("error retrieving clipboard data [in test]");
        assert_eq!(retrieved, expected);
    }
}
