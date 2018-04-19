extern crate rand;

pub mod mrandom;

pub fn roll_dice() -> u8 {
    let r = rand::random::<u8>();
    return r % 6 + 1;
}

pub fn roll_dices(dices: u8) -> u32 {
    let mut index: u32 = 0;
    for _i in {0..dices} {
        index *= 6;
        index += roll_dice() as u32;
    }
    return index;
}
