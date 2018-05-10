extern crate dpg;
extern crate clipboard;

use dpg::DicewareInfo;
use std::env; 
use std::collections::HashMap;
use std::process;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::{thread, time};

static ERR_NO_ARGUMENTS: i32 = 1;
static ERR_ARGUMENT_PARSING: i32 = 2;
static ERR_UNKNOWN_OPTION: i32 = 3;

static DEFAULT_SEPARATOR:&'static str = "-";
static DEFAULT_PASSWORD_COUNT: usize = 1;

static MIN_WORDS_COUNT: usize = 1;
static MAX_WORDS_COUNT: usize = 255;

const OPTION_PREFIXES :&'static str = "lwspch";

#[derive(Debug)]
pub struct Options {
    language:           String,
    separator:          String,
    password_length:    usize,
    password_count:     usize,
    clipboard:          bool,
    help:               bool,
}

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
-l:<language>       - language (en or pl)\r\
                      Default: en\r
-w:<number>         - the number of words to be generated (range: 1-255)\r
-s:<char>           - separator to be used to separate words.\r
                      Default: '-'\r
-p:<number>         - number of passwords to generate (up to 255)\r\
                      Default: 1\r
-c                  - copy generated password to clipboard\r
\r
-h                  - this help\r
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
fn validate_arguments(opts: &HashMap<String,String>) {
    println!("validating arguments...");
    for k in opts.keys() {
        if !OPTION_PREFIXES.contains(k) {
            println!("error: unknown option: -'{}'",k);
            process::exit(ERR_UNKNOWN_OPTION);
        }
    }
}

fn parse_command_line(args: Vec<String>) -> Options {
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

    validate_arguments(&opts);
    Options {
        language : opts.get("l").unwrap_or(&"en".to_string()).to_string(),
        password_length : opts.get("w").unwrap_or(&"4".to_string()).parse::<usize>().unwrap_or(0),
        clipboard : opts.contains_key("c"),
        password_count: opts.get("p").unwrap_or(&"1".to_string()).parse::<usize>().unwrap_or(DEFAULT_PASSWORD_COUNT),
        separator : opts.get("s").unwrap_or(&DEFAULT_SEPARATOR.to_string()).to_string(),
        help: opts.contains_key("h"),
    }
}

fn validate_options(options: &Options) {
    let language = options.language.clone();
    let password_length = options.password_length.clone();
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
    if options.help {
        info();
        process::exit(0);
    }
}

fn generate_single_password(options: &Options, all_diceware: &Vec<DicewareInfo>) -> String {
    let mut words: Vec<String> = Vec::new();
    for _i in {0..options.password_length} {
        let mut w = get_random_word(&options.language[..], all_diceware);
        words.push(w);
    }
    let password = words.join(&options.separator);
    password
}

fn generate_passwords(options: &Options, all_diceware: Vec<DicewareInfo>) -> String {
    let mut p: Vec<String> = Vec::<String>::new();
    for _i in { 0..options.password_count } {
        let new_password = generate_single_password(&options, &all_diceware);
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

fn validate_parameters_count(args: &Vec<String>) {
    if args.len() < 2 {
        println!("error: insufficient parameters. Type 'dpg -h' for help.");
        process::exit(ERR_NO_ARGUMENTS);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    validate_parameters_count(&args);
    let options = parse_command_line(args);

    #[cfg(debug_assertions)]
    println!("Options: {:?}", options);

    validate_options(&options);
    let all_diceware = dpg::read_all_diceware_lists();

    let password = generate_passwords(&options, all_diceware);

    if options.clipboard {
        copy_to_clipboard(password.clone());
    }
    println!("generated password(s):\n{}", password);
}
