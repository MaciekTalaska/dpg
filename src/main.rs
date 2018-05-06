extern crate dpg;

use dpg::DicewareInfo;
use std::env; 
use std::collections::HashMap;
use std::process;

static ERR_ARGUMENT_PARSING: i32 = 1;

static MIN_WORDS_COUNT: usize = 1;
static MAX_WORDS_COUNT: usize = 255;

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

fn info() {
    let info = "dpg - diceware password generator \r
author: Maciek Talaska <maciek.talaska@gmail.com> \r
source: github.com/MaciekTalaska/dpg \r
\r
options:
-l:<language>       - language (en or pl) - en is the default \r
-w:<number>         - the number of words to be generated (range: 1-255)\r
-s:<char>           - (not implemented!) separator to be used to separate words. By default '-' is used as a separator
-p:<number>         - (not implemented!) how many passwords to generate (up to 255)\r
-c                  - (not implemented!) copy generated password to clipboard\r 
-i:<path_to_file>   - (not implemented!) use external file with word list\r
-
-h                  - this help\r
-?                  - this help
\n";
    print!("{}",info);
}

fn check_argument_format(option: &str) {
    if !option.starts_with("-") {
        println!("unrecognized option: {}", option);
        println!("  are you missing a '-' prefix?");
        process::exit(ERR_ARGUMENT_PARSING);
    }
}

fn get_option_key_value(option: &str) -> (String, String) {
    check_argument_format(option);

    let input = match option.starts_with("-") {
        true => option[1..].to_ascii_lowercase(),
        false => option[..].to_ascii_lowercase(),
    };

    let index = input.find(":").unwrap_or(input.len());
    let (k,v) = input.split_at(index);
    #[cfg(debug_assertions)]
    println!("k/v: {:?}", (k,v));

    (k.to_string(), v.replace(":",""))
}

fn parse_command_line(args: Vec<String>) -> (String, usize) {
    let mut opts: HashMap<String, String> = HashMap::new();

    match args.len() {
        1 => info(),
        2...5 => {
            for i in {1..args.len()} {
                let (k,v) =  get_option_key_value(&args[i]);
                opts.insert(k, v);
            }
        },
        _ => info()
    }

    let language = opts.get("l").unwrap().to_string();
    let words_count = opts.get("w")
        .unwrap_or(&"4".to_string())
        .parse::<usize>()
        .unwrap_or(0);
    (language, words_count)
}

fn check_parameters(language: &String, password_length: usize) {
    #[cfg(debug_assertions)]
    println!("[passed parameter to check] language: {} password: {}", language, password_length);
    if password_length < MIN_WORDS_COUNT || password_length  > MAX_WORDS_COUNT {
       println!("error: password should consist of at least {} and max {} words", MIN_WORDS_COUNT, MAX_WORDS_COUNT);
       process::exit(ERR_ARGUMENT_PARSING);
    }
    if language != "en" && language != "pl" {
        println!("error: language: '{}' is not supported!", language);
        process::exit(ERR_ARGUMENT_PARSING);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let (language, password_length) = parse_command_line(args);
	check_parameters(&language, password_length);

    let all_diceware = dpg::read_all_diceware_lists();
    let mut words: Vec<String> = Vec::new();
    for _i in {0..password_length} {
        let mut w = get_random_word(&language[..], &all_diceware);
        words.push(w);
    }
    let password = words.join("-");
    println!("generated password: {}", password); 
}
