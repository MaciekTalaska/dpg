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
static ERR_TOO_MANY_OPTIONS: i32 = 4;

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

impl PartialEq for Options {
    fn eq(&self, other: &Options) -> bool {
        self.language == other.language
        && self.separator == other.separator
        && self.password_length == other.password_length
        && self.password_count == other.password_count
        && self.clipboard == other.clipboard
        && self.help == other.help
        && self.simulate_dices == other.simulate_dices
    }
}

pub fn parse_command_line(args: Vec<String>) -> Options {
    validate_parameters_count(&args);

    let mut opts: HashMap<String, String> = HashMap::new();

    match args.len() {
        1 => info(),
        2...7 => {
            for i in { 1..args.len() } {
                let (k, v) = get_option_key_value(&args[i]);
                opts.insert(k, v);
            }
        }
        _ => too_many_arguments_error(),
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
            .expect("error parsing options: '-w' is required!")
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

    options
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


fn too_many_arguments_error() {
    let info_message = " \
error: too many options provided! \r\
\r\
use -h for help";
    println!("{:}", info_message);
    exit(ERR_TOO_MANY_OPTIONS);
}

fn info() {
    let info_message = "dpg - diceware password generator \r
author: Maciek Talaska <maciek.talaska@gmail.com> \r
source: github.com/MaciekTalaska/dpg \r
\r
options:
-l:<language>   language (en or pl)                         [default: en]\r
-w:<number>     password length (in words)                  [range: 1-255]\r
-p:<number>     number of passwords to generate (up to 255) [default: 1]\r
-s:<char>       separator to be used to separate words      [default: '-']\r
-c              copy generated password to clipboard\r\
-d              simulate throwing dices (slower)\r\
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

#[cfg(test)]
mod option_parser_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "1")]
    fn should_not_work_with_empty_argument_list() {
        let args = vec![s!("./dpg")];
        let _options = parse_command_line(args);
    }

    #[test]
    fn only_words_is_required_parameter() {
        let args = vec![s!("./dpg"),
                        s!("-w:5")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "en".to_string(),
            password_length: 5,
            password_count: 1,
            separator: "-".to_string(),
            simulate_dices: false,
            help: false,
            clipboard: false,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    fn only_language_and_passwordlength() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:6")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 6,
            password_count: 1,
            separator: "-".to_string(),
            simulate_dices: false,
            help: false,
            clipboard: false,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    fn language_password_length_and_count() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:3"),
                        s!("-p:5")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 3,
            password_count: 5,
            separator: "-".to_string(),
            simulate_dices: false,
            help: false,
            clipboard: false,
        };
        assert_eq!(options, expected_options);
    }


    #[test]
    fn language_password_length_count_separator() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:7"),
                        s!("-p:4"),
                        s!("-s:.")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 7,
            password_count: 4,
            separator: ".".to_string(),
            simulate_dices: false,
            help: false,
            clipboard: false,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    fn language_password_length_count_separator_clipboard() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:8"),
                        s!("-p:5"),
                        s!("-s:."),
                        s!("-c")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 8,
            password_count: 5,
            separator: ".".to_string(),
            simulate_dices: false,
            help: false,
            clipboard: true,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    fn language_password_length_count_separator_clipboard_simulate_dices() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:9"),
                        s!("-p:6"),
                        s!("-s:."),
                        s!("-c"),
                        s!("-d")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 9,
            password_count: 6,
            separator: ".".to_string(),
            simulate_dices: true,
            help: false,
            clipboard: true,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    #[should_panic(expected = "0")]
    fn language_password_length_count_separator_clipboard_simulate_dices_help() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:9"),
                        s!("-p:6"),
                        s!("-s:."),
                        s!("-h")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 9,
            password_count: 6,
            separator: ".".to_string(),
            simulate_dices: true,
            help: true,
            clipboard: true,
        };
        assert_eq!(options, expected_options);
    }

    #[test]
    #[should_panic(expected = "4")]
    fn too_many_options() {
        let args = vec![s!("./dpg"),
                        s!("-l:pl"),
                        s!("-w:9"),
                        s!("-p:6"),
                        s!("-s:."),
                        s!("-c"),
                        s!("-d"),
                        s!("-h")];
        let options = parse_command_line(args);
        let expected_options = Options {
            language: "pl".to_string(),
            password_length: 9,
            password_count: 6,
            separator: ".".to_string(),
            simulate_dices: true,
            help: true,
            clipboard: true,
        };
        assert_eq!(options, expected_options);
    }
}
