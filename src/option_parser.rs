use std::collections::HashMap;

#[cfg(not(debug_assertions))]
use std::process::exit;
#[cfg(debug_assertions)]
fn exit(exit_code: i32) {
    eprintln!("exit_code: {0}", exit_code);
    panic!("{0}", exit_code);
}

static ERR_NO_ARGUMENTS: i32 = 1;
static ERR_ARGUMENT_PARSING: i32 = 2;
static ERR_UNKNOWN_OPTION: i32 = 3;

static DEFAULT_SEPARATOR: &'static str = "-";
static DEFAULT_PASSWORD_COUNT: usize = 1;

static MIN_WORDS_COUNT: usize = 1;
static MAX_WORDS_COUNT: usize = 255;

const OPTION_PREFIXES: &'static str = "lwspchd";

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct Options {
    pub language:           String,
    pub separator:          String,
    pub password_length:    usize,
    pub password_count:     usize,
    pub clipboard:          bool,
    pub help:               bool,
    pub simulate_dices:     bool,
}

pub fn parse_command_line(args: Vec<String>) -> Options {
    validate_parameters_count(&args);

    let mut opts: HashMap<String, String> = HashMap::new();

    match args.len() {
        1 => info(),
        2...5 => {
            for i in { 1..args.len() } {
                let (k, v) = get_option_key_value(&args[i]);
                opts.insert(k, v);
            }
        }
        _ => info(),
    }

    validate_arguments(&opts);
    create_options(&opts)
}

fn validate_parameters_count(args: &[String]) {
    if args.len() < 2 {
        eprintln!("error: insufficient parameters. Type 'dpg -h' for help.");
        exit(ERR_NO_ARGUMENTS);
    }
}

fn validate_arguments(opts: &HashMap<String, String>) {
    println!("validating arguments...");
    for k in opts.keys() {
        if !OPTION_PREFIXES.contains(k) {
            eprintln!("error: unknown option: -'{}'", k);
            exit(ERR_UNKNOWN_OPTION);
        }
    }
}

fn create_options(opts: &HashMap<String, String>) -> Options {
    let options = Options {
        language: opts.get("l").unwrap_or(&"en".to_string()).to_string(),
        password_length: opts.get("w")
            .unwrap_or(&"4".to_string())
            .parse::<usize>()
            .unwrap_or(0),
        clipboard: opts.contains_key("c"),
        password_count: opts.get("p")
            .unwrap_or(&"1".to_string())
            .parse::<usize>()
            .unwrap_or(DEFAULT_PASSWORD_COUNT),
        separator: opts.get("s")
            .unwrap_or(&DEFAULT_SEPARATOR.to_string())
            .to_string(),
        simulate_dices: opts.contains_key("d"),
        help: opts.contains_key("h"),
    };
    validate_options(&options);
    return options;
}

fn validate_options(options: &Options) {
    let language = options.language.clone();
    let password_length = options.password_length.clone();
    #[cfg(debug_assertions)]
    println!(
        "[passed parameter to check] language: {} password: {}",
        language, password_length
    );
    if password_length < MIN_WORDS_COUNT || password_length > MAX_WORDS_COUNT {
        eprintln!(
            "error: password should consist of at least {} and max {} words",
            MIN_WORDS_COUNT, MAX_WORDS_COUNT
        );
        exit(ERR_ARGUMENT_PARSING);
    }
    if language != "en" && language != "pl" {
        eprintln!("error: language: '{}' is not supported!", language);
        exit(ERR_ARGUMENT_PARSING);
    }
    if options.help {
        info();
        exit(0);
    }
}

fn get_option_key_value(option: &str) -> (String, String) {
    check_argument_format(option);

    let input = match option.starts_with("-") {
        true => option[1..].to_ascii_lowercase(),
        false => option[..].to_ascii_lowercase(),
    };

    let index = input.find(":").unwrap_or(input.len());
    let (k, v) = input.split_at(index);
    #[cfg(debug_assertions)]
    println!("k/v: {:?}", (k, v));

    (k.to_string(), v.replace(":", ""))
}

fn info() {
    let info_message = "dpg - diceware password generator \r
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
-c                  - copy generated password to clipboard\r\
-d                  - simulate throwing dices (slower)\r\
\r
-h                  - this help\r
\n";
    print!("{}", info_message);
}

fn check_argument_format(option: &str) {
    if !option.starts_with("-") {
        eprintln!("unrecognized option: {}", option);
        eprintln!("  are you missing a '-' prefix?");
        exit(ERR_ARGUMENT_PARSING);
    }
}
