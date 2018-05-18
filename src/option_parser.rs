use std::collections::HashMap;
use std::process;

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
    pub language:           String,
    pub separator:          String,
    pub password_length:    usize,
    pub password_count:     usize,
    pub clipboard:          bool,
    pub help:               bool,
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

pub fn parse_command_line(args: Vec<String>) -> Options {
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

pub fn validate_options(options: &Options) {
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

pub fn validate_parameters_count(args: &Vec<String>) {
    if args.len() < 2 {
        println!("error: insufficient parameters. Type 'dpg -h' for help.");
        process::exit(ERR_NO_ARGUMENTS);
    }
}
