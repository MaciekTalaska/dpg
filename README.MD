
[![Appveyor Build Status](https://ci.appveyor.com/api/projects/status/github/MaciekTalaska/dpg)](https://ci.appveyor.com/project/MaciekTalaska/dpg)
[![Travis Build Status](https://secure.travis-ci.org/MaciekTalaska/dpg.png?branch=master)](https://travis-ci.org/MaciekTalaska/dpg)

dpg (diceware password generator)
=================================

dpg is a simple executable that helps in generating strong and easy to remember password using diceware method.

What is diceware?
-----------------

Common problem with strong, secure passwords is that these are hard to remember. Diceware method () aims to solve the problem. This method is based on generating password containing couple of words (4 or more). It is easier to remember couple of words, especially those that could be somehow visualized, than do the same with random string of letters, special characters, digits etc. Good explanation of diceware has been depicted in one of the xkcd comic strips: 

![alt text](https://imgs.xkcd.com/comics/password_strength.png "xkcd on Diceware")

Diceware method is described in details by its author - Arnold G. Reinhold: http://world.std.com/~reinhold/diceware.html

Why dpg?
--------

This app has been created as an opportunity to get familiar with Rust programming language. It was my first project written in Rust. I just wanted to check the language on something that is a bit more than just a kata. This project allowed me to research on Rust project stucture (splitting code into modules), testing.

Diceware lists
--------------

Dpg requires list of words to be used (diceware word list). At the moment the application has two lists built in: one containg words in English (default) and other list containing words in Polish (it has to be explicity specified). 

List of english words has been taken from FFI website: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
List of Polish words has been created by me (https://github.com/MaciekTalaska/diceware-pl) - this work is based on the list created originally by Piotr (DrFugazi) Tarnowski.


[Mathias Gumz's repository](https://github.com/mgumz/diceware/tree/master/lists) contains diceware lists for different languages.



Security
--------

Dpg uses OsRng from rand crate (https://crates.io/crates/rand). Rand crate documentation states that: ["An application that requires an entropy source for cryptographic purposes must use OsRng, which reads randomness from the source that the operating system provides (e.g. /dev/urandom on Unixes or CryptGenRandom() on Windows). The other random number generators provided by this module are not suitable for such purposes."](https://docs.rs/rand/0.4.2/rand/#cryptographic-security) 

Options
-------

`-l:<language>`     language list to use - currently only 'en' (English) or 'pl' (Polish) are supported. 'en' is used by default - i.e. for generating passwords consisting of English words this option does not have to be explicitly specified

`-w:<number>`       the number of words (password length in words) to be generated.<br>
                    &emsp;Minumum: 1<br>
                    &emsp;Maximum: 255<br>

`-p:<number>`       number of passwords to generate at once<br>
                    &emsp;Default: 1<br>
                    &emsp;Minumum: 1<br>
                    &emsp;Maximum: 255<br>
                    

`-s:<character>`    a character to be used to separate words<br>
                    &emsp;Default: `-` (dash)

Flags:
------

`-c`                copy generated password to clipboard

`-d`                simulate dices option. At the moment dpg may work in two different modes:<br>
                    &emsp;a) 'simplified mode' in which only one random number is generated to get a single words from a list<br>
                    &emsp;b) 'diceware explicit' mode, in which there are several random numbers generated, each is equivalent of rolling a dice, that are required to retrieve single word from list<br><br>
                    &emsp;These two modes offer the same safety (as the same, crypto-secure method of generating random numbers is used), but obviously the second one (which could be turned on by using `-d` switch) is a tiny bit slower, as there are more operations required. This should not be anything ntoicable unless there are very many very long passwords being generated.
                    </div>
Todo:
Additional options to be added:

`-i:<file path>`    path to external wordlist file

Installation/Usage
------------------

`dpg` is available on crates.io, so the easiest way to have it installed is via cargo: `cargo install dpg`. 

This will download, compile and store the binary inside the `~/.cargo/bin` directory, and this should make `dpg` available to the current user as a command-line utility.



Licenses
--------

Diceware is trademark (™) by Arnold G. Reinhold (http://world.std.com/~reinhold/diceware.html)

The English word list used is created and copyrighted by Electronic Frontier Foundation (https://www.eff.org/about).

Polish word list has been created by myself, but is based on the work of Dr Piotr 'Fugazi' Tarnowski.
