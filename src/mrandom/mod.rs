extern crate rand;

pub mod mrandom;

pub fn roll_dice() -> u8 {
    let r = rand::random::<u8>();
    return r % 6 + 1;
}

pub fn roll_dice_test() {
    let index = roll_dice();
    println!("dice roll returned: {:?}", index);
}

pub fn roll_dices(dices: u8) -> u32 {
    let mut index: u32 = 0;
    for _i in {0..dices} {
        if index == 0 {
            index += roll_dice() as u32;
        } else {
            index += roll_dice() as u32;
            index *= 6;
        }
    }
    return index;
}

pub fn roll_dices_test() {
    roll_dices(5);
}
