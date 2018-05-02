extern crate dpg;

use dpg::DicewareInfo;
use std::env; 
use std::collections::HashMap;

fn get_diceware_info_by_language(language: &str, diceware_data: &Vec<DicewareInfo>) -> DicewareInfo {
    match language.to_lowercase().as_str() {
        "pl" => diceware_data[1].clone(),
        _ => diceware_data[0].clone()
    }
}

fn get_random_word(language: &str, diceware_data: &Vec<DicewareInfo>) -> String {
    let info: DicewareInfo = get_diceware_info_by_language(language, diceware_data.as_ref());
    //println!("number of dice rolls: {:?}", info.num_dices);
    let result = dpg::mrandom::roll_dices(info.num_dices);

    //println!("index: {:?}", result);
    info.words[result as usize % info.words.len()].clone()
}

fn info() {
    let info = "dpg - diceware password generator \r
author: Maciek Talaska <maciek.talaska@gmail.com> \r
source: github.com/MaciekTalaska/dpg \r
\r
options:
-l:<language>       - language (en or pl) - en is the default \r
-s:<number>         - the number of words to be generated - default is 4 \r
-p:<number>         - (not implemented!) how many passwords to generate (up to 255)\r
-c                  - (not implemented!) copy generated password to clipboard\r 
-i:<path_to_file>   - (not implemented!) use external file with word list\r
-h                  - this help\r
-?                  - this help
\n";
    print!("{}",info);
}

fn parse_command_line() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    let mut opts: HashMap<String, String> = HashMap::new();

    match args.len() {
        1 => {
            info();
        },
        2...5 => {
            for i in {1..args.len()} {
                let t = &args[i]
                    .as_str()
                    .replace("-", "")
                    .to_ascii_lowercase().clone();
                let (k, v) = t.split_at(t
                                        .find(":")
                                        .unwrap_or(t.len()));
                opts.insert(k.to_string().clone(), v.replace(":","").to_string().clone());
            }
        },
        _ => {
            info();
        }
    }

    let language = opts.get("l").unwrap().to_string();
    let words_count = opts.get("s")
        .unwrap_or(&"4".to_string())
        .parse::<usize>()
        .unwrap_or(4);
    (language, words_count)
}


fn main() {
    let (language, password_length) = parse_command_line();
	
    let all_diceware = dpg::read_all_diceware_lists();
    let mut words: Vec<String> = Vec::new();
    for _i in {0..password_length} {
        let mut w = get_random_word(&language[..], &all_diceware);
        words.push(w);
    }
    let password = words.join("-");
    println!("generated password: {}", password); 
}
