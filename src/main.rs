extern crate dpg;

use dpg::DicewareInfo;

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

fn main() {
    let all_diceware = dpg::read_all_diceware_lists();
    println!("random word: {:?}", get_random_word("pl",&all_diceware));
}
