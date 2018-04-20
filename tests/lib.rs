extern crate dpg;

#[test]
fn classic_dice_roll_should_always_return_value_from_1_to_6() {
    let result = dpg::mrandom::roll_dice();
    assert!(result >= 1);
    assert!(result <= 6);
}

#[test]
fn roll_dices_1_dice_should_return_value_from_0_to_5() {
    let result = dpg::mrandom::roll_dices(1);
    assert!(result <= 5);
}

#[test]
fn roll_dices_2_dices_should_not_exceed_35() {
    let result = dpg::mrandom::roll_dices(2);
    assert!(result <= 35);
}

#[test]
fn roll_dices_3_dices_should_not_excedeed_215() {
    let result = dpg::mrandom::roll_dices(3);
    assert!(result <= 215);
}

#[test]
fn roll_dices_4_dices_should_not_exceed_1295() {
    let result = dpg::mrandom::roll_dices(4);
    assert!(result <= 1295);
}

#[test]
fn roll_dices_5_dices_should_not_exceed_7775() {
    let result = dpg::mrandom::roll_dices(5);
    assert!(result <= 7775);
}
