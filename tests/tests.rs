#[macro_use]
extern crate dpg;

#[test]
#[should_panic]
fn calling_dpg_without_any_argument_should_fail() {
    let args = vec![s!("./dpg")];
    let _passwords = dpg::generate_diceware_passwords_old(args);
}

#[test]
fn generate_passwords_without_specifying_language() {
    let args = vec![s!("./dpg"), s!("-w:4"), s!("-p:1")];
    let passwords = dpg::generate_diceware_passwords_old(args);
    assert!(passwords.len() > 0);
    assert_eq!(passwords.lines().count(), 1);
}

#[test]
fn generate_password_using_polish_wordlist() {
    let args = vec![s!("./dpg"), s!("-l:pl"), s!("-w:4"), s!("-p:1")];
    let passwords = dpg::generate_diceware_passwords_old(args);
    assert!(passwords.len() > 0);
    assert_eq!(passwords.lines().count(), 1);
}

#[test]
#[should_panic(expected = "2")]
fn generate_password_specifying_unsupported_language_should_fail() {
    let args = vec![s!("./dpg"), s!("-l:xy"), s!("-w:4"), s!("-p:1")];
    dpg::generate_diceware_passwords_old(args);
}

#[test]
#[should_panic(expected = "1")]
fn should_exit_early_if_no_parameters_are_given() {
    let args = vec![s!("dpg")];
    dpg::generate_diceware_passwords_old(args);
}

#[test]
#[should_panic(expected = "2")]
fn should_fail_if_asked_to_generate_password_consisting_of_less_than_1_word() {
    let args = vec![s!("dpg"), s!("-w:0")];
    dpg::generate_diceware_passwords_old(args);
}

#[test]
#[should_panic(expected = "2")]
fn should_fail_if_asked_to_generate_password_longer_than_255_words() {
    let args = vec![s!("dpg"), s!("-w:256")];
    dpg::generate_diceware_passwords_old(args);
}

#[test]
#[should_panic(expected = "3")]
fn should_fail_if_unknown_option_is_used() {
    let args = vec![s!("dpg"), s!("-q:yes")];
    dpg::generate_diceware_passwords_old(args);
}