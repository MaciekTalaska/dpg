extern crate dpg;


fn main() {
    //dpg::mrandom::dice_roll_test();
    //dpg::mrandom::dice_rolls_test();
    let all_diceware = dpg::read_all_diceware_lists();
    for info in all_diceware {
        dpg::print_diceware_info(info);
    }
}
