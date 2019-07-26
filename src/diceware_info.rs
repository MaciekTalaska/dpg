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

/// Builds repository of word lists for many languages.
/// Note: currently does NOT support external words lists!
pub fn build_diceware_repository() -> Vec<DicewareInfo> {
    let languages = [s!("en"), s!("pl")];

    languages.iter()
        .map(|language| read_diceware_list(language))
        .collect::<Vec<DicewareInfo>>()
}

fn read_diceware_list(language: &str) -> DicewareInfo {
    let words = get_diceware_words_by_language(language);

    process_diceware_words(&words, language)
}

fn get_diceware_words_by_language(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "en" => ENGLISH_DICEWARE,
        "pl" => POLISH_DICEWARE,
        _ => ENGLISH_DICEWARE,
    }
}

fn calculate_max_dice_count(size: usize) -> u8 {
    (size as f32)
        .log(6.0)
        .abs()
        .ceil() as u8
}

fn process_diceware_words(message: &str, language: &str) -> DicewareInfo {
    let words = message
        .lines()
        .map(|l| l.split_whitespace().last().expect("bad file format: second column is missing"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    DicewareInfo {
        language: language.to_string(),
        num_dices: calculate_max_dice_count(words.len()),
        words,
    }
}
