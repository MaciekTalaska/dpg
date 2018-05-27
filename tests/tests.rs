extern crate dpg;

#[test]
fn generating_passwords() {
    let args = vec![ String::from("-l:en"), String::from("-w:4"), String::from("-p:1")];
    let passwords = dpg::generate_diceware_passwords(args);
    assert!(passwords.len() > 0);
    assert_eq!(passwords.lines().count(), 1);
}