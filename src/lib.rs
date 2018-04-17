
pub mod mrandom;
//use mrandom;
//use std::collections::HashMap;

//const diceware_files: HashMap<&str, &str> =
//[("en", "diceware-en.txt"),
// ("pl", "diceware-pl.txt")];
// //.iter().cloned().collect();

fn process_diceware_words(message: &str) {
    let s4 =  message
        .lines()
        .map(|l| l.split_whitespace().last().unwrap())
        .collect::<Vec<&str>>();

    println!("sp4: {:?}", s4);
}

fn read_diceware_list() {
    let bytes = include_bytes!("diceware-en.txt");
    let words = String::from_utf8_lossy(bytes);
    process_diceware_words(&words);
}


//fn show_files() {
//    for (k, v) in DICEWARE_FILES {
//        println!("key: {:?} value: {:?}", k, v);
//    }
//}

fn main() {
    mrandom::dice_roll_test();
    mrandom::dice_rolls_test();
    //read_diceware_list();
//    show_files();
}
