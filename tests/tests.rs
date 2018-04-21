extern crate dpg;

#[test]
fn classic_dice_roll_should_always_return_value_from_1_to_6() {
    let result = dpg::mrandom::roll_dice();
    assert!(result >= 1);
    assert!(result <= 6);
}

fn max_by_dice_num(dice_num: u8) -> u32 {
    (6 as u32).pow(dice_num as u32) -1
}

#[test]
fn roll_dices_1_dice_should_return_value_from_0_to_5() {
    let result = dpg::mrandom::roll_dices(1);
    assert!(result <= max_by_dice_num(1));
}

#[test]
fn roll_dices_2_dices_should_not_exceed_35() {
    for _i in {0..10} {
        let result = dpg::mrandom::roll_dices(2);
        assert!(result <= max_by_dice_num(2));
    }
}

#[test]
fn roll_dices_3_dices_should_not_exceed_215() {
    for _i in {0..40} {
        let result = dpg::mrandom::roll_dices(3);
        assert!(result <= max_by_dice_num(3));
    }
}

#[test]
fn roll_dices_4_dices_should_not_exceed_1295() {
    for _i in {0..100} {
        let result = dpg::mrandom::roll_dices(4);
        assert!(result <= max_by_dice_num(4));
    }
}

#[test]
fn roll_dices_5_dices_should_not_exceed_7775() {
    for _i in {0..1000} {
        let result = dpg::mrandom::roll_dices(5);
        assert!(result <= max_by_dice_num(5));
    }
}
