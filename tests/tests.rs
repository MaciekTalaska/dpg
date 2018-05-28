#[macro_use]
extern crate dpg;

#[test]
fn generating_passwords() {
    let args = vec![s!("-l:en"), s!("-w:4"), s!("-p:1")];
    let passwords = dpg::generate_diceware_passwords(args);
    assert!(passwords.len() > 0);
    assert_eq!(passwords.lines().count(), 1);
}
