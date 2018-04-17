extern crate dpg;

//use ::{mrandom};

#[test]
fn dice_throw_should_always_return_value_from_1_to_6() {
    let result = dpg::mrandom::dice_roll();
    assert!(result > 0);
    assert!(result <= 6);
}

#[test]
fn dice_throws_should_return_array_of_values() {
    assert!(false);
}