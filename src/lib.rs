pub mod mrandom;

struct DicewareInfo {
    language:   String,
    num_dices:  u8,
    words:      Vec<String>
}

fn calculate_max_dice_count(size: usize) -> u8 {
    let fsize: f32 = size as f32;
    let result = fsize.log(6.0).abs().ceil() as u8;
    return result;
}

fn process_diceware_words(message: &str) -> DicewareInfo {
    let words =  message
        .lines()
        .map(|l| l.split_whitespace().last().unwrap())
        .map( |s| s.to_string())
        .collect::<Vec<String>>();

    println!("first word: {:?}", words[0]);
    println!("second word:  {:?}", words[1]);

    return DicewareInfo{language: "en".to_string(),
        num_dices: calculate_max_dice_count(words.len()),
        words};
}

pub fn read_diceware_list() {
    let bytes = include_bytes!("diceware-pl.txt");
    let words = String::from_utf8_lossy(bytes).to_string().to_owned();
    let info = process_diceware_words(&words);
    println!("language: {:?}", info.language);
    println!("num_dices: {:?}", info.num_dices);
    println!("words[0]: {:?}", info.words[0]);
    println!("words.length: {:?}", info.words.len());
}