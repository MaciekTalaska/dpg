extern crate rand;

pub mod mrandom;

pub fn dice_roll() -> u8 {
    let r = rand::random::<u8>();
    return r % 6 + 1;
}

pub fn dice_rolls(dices: u8) -> u8 {
    println!("dices to throw: {:?}", dices);
    for i in {0..dices} {
        println!("current dice throw: {:?} result: {:?}", i, dice_roll());
    }
    return dices;
}

pub fn dice_roll_test() {
    let index = dice_roll();
    println!("dice roll returned: {:?}", index);
}

pub fn dice_rolls_test() {
    dice_rolls(5);
}
