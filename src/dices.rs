extern crate rand;

use self::rand::Rng;

fn get_rnd() -> rand::OsRng {
    match rand::OsRng::new() {
        Ok(result) => result,
        Err(error) => panic!(
            "Unable to obtain crypto secure random number generator!\n{}",
            error
        ),
    }
}

pub fn get_random_number(max: u32) -> u32{
    let mut rng = get_rnd();
    let value = rng.gen::<u32>();

    (value % max)
}

pub fn roll_dice() -> u8 {
    let mut rng = get_rnd();

    roll_dice_internal(&mut rng)
}

fn roll_dice_internal(rng: &mut rand::OsRng) -> u8 {
    let value = rng.gen::<u8>();

    value % 6 + 1
}

pub fn roll_dices(dices: u8) -> u32 {
    let mut index: u32 = 0;
    let mut rng = get_rnd();
    for _i in { 0..dices } {
        index *= 6;
        index += (roll_dice_internal(&mut rng) as u32) - 1;
    }

    index
}

#[cfg(test)]
mod dices_tests {
    use dices;

    #[test]
    fn classic_dice_roll_should_always_return_value_from_1_to_6() {
        let result = dices::roll_dice();
        assert!(result >= 1);
        assert!(result <= 6);
    }

    fn max_by_dice_num(dice_num: u8) -> u32 {
        (6 as u32).pow(dice_num as u32) - 1
    }

    #[test]
    fn roll_dices_1_dice_should_return_value_from_0_to_5() {
        let result = dices::roll_dices(1);
        assert!(result <= max_by_dice_num(1));
    }

    #[test]
    fn roll_dices_2_dices_should_not_exceed_35() {
        for _i in { 0..10 } {
            let result = dices::roll_dices(2);
            assert!(result <= max_by_dice_num(2));
        }
    }

    #[test]
    fn roll_dices_3_dices_should_not_exceed_215() {
        for _i in { 0..40 } {
            let result = dices::roll_dices(3);
            assert!(result <= max_by_dice_num(3));
        }
    }

    #[test]
    fn roll_dices_4_dices_should_not_exceed_1295() {
        for _i in { 0..100 } {
            let result = dices::roll_dices(4);
            assert!(result <= max_by_dice_num(4));
        }
    }

    #[test]
    fn roll_dices_5_dices_should_not_exceed_7775() {
        for _i in { 0..1000 } {
            let result = dices::roll_dices(5);
            assert!(result <= max_by_dice_num(5));
        }
    }

}
