#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Clone)]
pub struct DicewareInfo {
    pub language:   String,
    pub num_dices:  u8,
    pub words:      Vec<String>
}

static POLISH_DICEWARE: &str = include_str!("diceware-pl.txt");
static ENGLISH_DICEWARE: &str = include_str!("diceware-en.txt");

#[cfg(debug_assertions)]
pub fn print_diceware_info(info: DicewareInfo) {
    println!("---- print diceware info ----");
    println!("language: {:?}", info.language);
    println!("num_dices: {:?}", info.num_dices);
    println!("words[0]: {:?}", info.words[0]);
    println!("words.length: {:?}", info.words.len());
}

pub fn build_diceware_repository() -> Vec<DicewareInfo> {
    let languages = ["en", "pl"];
    let mut diceware_repository: Vec<DicewareInfo> = Vec::new();
    for lang in languages.iter() {
        let info = read_diceware_list(lang);
        diceware_repository.push(info);
    }
    return diceware_repository;
}

fn read_diceware_list(language: &str) -> DicewareInfo {
    let words = get_diceware_words_by_language(language);
    let mut info = process_diceware_words(&words);
    info.language = language.to_string();
    return info;
}

fn get_diceware_words_by_language(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "en" => ENGLISH_DICEWARE,
        "pl" => POLISH_DICEWARE,
        _ => ENGLISH_DICEWARE,
    }
}

fn calculate_max_dice_count(size: usize) -> u8 {
    let fsize: f32 = size as f32;
    let result = fsize.log(6.0).abs().ceil() as u8;
    return result;
}

fn process_diceware_words(message: &str) -> DicewareInfo {
    let words = message
        .lines()
        .map(|l| l.split_whitespace().last().unwrap())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    return DicewareInfo {
        language: s!(""),
        num_dices: calculate_max_dice_count(words.len()),
        words,
    };
}
