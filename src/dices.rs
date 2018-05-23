extern crate rand;

use self::rand::Rng;

fn get_rnd() -> rand::OsRng {
    let rng = match rand::OsRng::new() {
        Ok(result) => result,
        Err(error) => panic!("Unable to obtain crypto secure random number generator! {}", error)
    };
    return rng;
}

pub fn roll_dice() -> u8 {
    let mut rng = get_rnd();
    return roll_dice_internal(&mut rng);
}

fn roll_dice_internal(rng: &mut rand::OsRng) -> u8 {
    let value = rng.gen::<u8>();
    return value % 6 + 1;
}

pub fn roll_dices(dices: u8) -> u32 {
    let mut index: u32 = 0;
    let mut rng = get_rnd();
    for _i in {0..dices} {
        index *= 6;
        index += (roll_dice_internal(&mut rng) as u32) -1;
    }
    return index;
}
