# Grep explorer

This is a command line interface program that helps you open a file
referenced in grep command result and locate matched line.

This program executes `grep` command with `-rn` arguments in order
to recursively search for a pattern and display line number.

## Example:

`
mygrep --include=*.rs let
`

Will offer all lines when word **let** has been matched and ask for user input
to select which match should be opened in Vim. Cursor position will be ona a
selected match line. So, this program accepts all grep arguments.

## Installation

This program still does not exists on crates.io so you have to build from source :)
Clone this repository and execute:

`
cargo build --release
`

Then copy `./target/release/pgrep` into your CARGO\_HOME bin directory (ex `/home/<user_name>/.cargo/bin`).
