extern crate rand;

use std::vec::Vec;

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
//    println!("dices to throw: {:?}", dices);
//    for i in {0..dices} {
//        println!("current dice throw: {:?} result: {:?}", i, dice_roll());
//    }
//    return dices as usize;

//    let mut all_dices= vec![0; dices as usize];
//    for i in {0..dices} {
//        all_dices[i as usize] = dice_roll() as u8;
//    }
//    let mut all_dices = Vec::<u8>::new();
//    for i in {0..dices} {
//        all_dices.push(roll_dice());
//    }
//    return all_dices as Vec<u8>;
    let mut index: u32 = 0;
    for i in {0..dices} {
        if (index == 0) {
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
