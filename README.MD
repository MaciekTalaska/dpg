
[![Build Status](https://secure.travis-ci.org/MaciekTalaska/xmastree.png?branch=master)](https://travis-ci.org/MaciekTalaska/dpg)

dpg (diceware password generator)
=================================

dpg is a simple executable that helps in generating strong and easy to remember password using diceware method.

What is diceware?
-----------------

Common problem with strong, secure passwords is that these are hard to remember. Diceware method () aims to solve the problem. This method is based on generating password containing couple of words (4 or more). It is easier to remember couple of words, especially those that could be somehow visualized, than do the same with random string of letters, special characters, digits etc. Good explanation of diceware has been depicted in one of the xkcd comic strips: https://xkcd.com/936/

Why dpg?
--------

This app has been created as an opportunity to get familiar with Rust programming language. It was my first project written in Rust. I just wanted to check the language on something that is a bit more than just a kata. This project allowed me to research on Rust project stucture (splitting code into modules), testing.

Diceware lists
--------------

Dpg requires list of words to be used (diceware word list). At the moment the application has two lists built in: one containg words in English (default) and other list containing words in Polish (it has to be explicity specified). 

List of english words has been taken from FFI website: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
List of Polish words has been created by me (https://github.com/MaciekTalaska/diceware-pl) - this work is based on the list created originally by Piotr (DrFugazi) Tarnowski.

Security
--------

Dpg uses OsRng from rand crate (https://crates.io/crates/rand). Rand crate documentation states that: ["An application that requires an entropy source for cryptographic purposes must use OsRng, which reads randomness from the source that the operating system provides (e.g. /dev/urandom on Unixes or CryptGenRandom() on Windows). The other random number generators provided by this module are not suitable for such purposes."](https://docs.rs/rand/0.4.2/rand/#cryptographic-security) 

Options
-------
`-l:<language>`     language list to use - currently only 'en' (English) or 'pl' (Polish) are supported. 'en' is used by default - i.e. for generating passwords consisting of English words this option does not have to be explicitly specified

`-w:<number>`       the number of words (password length in words) to be generated.<br>
                    &emsp;Minumum: 1<br>
                    &emsp;Maximum: 255<br>

`-c`                copy generated password to clipboard

`-p:<number>`       number of passwords to generate at once

`-s:<character>`    a character to be used to separate words, by default '-' is used

Todo:
Additional options to be added:

`-i:<file path>`    path to external wordlist file
