# Typing practice

![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/Typing-Practice-tui)

This is a simple Terminal User Interface (tui) program to practice typing and try [tui-rs](https://docs.rs/crate/tui/latest).

![tui_typing_practice](https://github.com/CMIW/Typing-Practice-tui/blob/master/Screenshot_typing_practice.png)

## Prerequisites

Before you begin, follow the Rust installation guide from https://doc.rust-lang.org/book/ch01-01-installation.html.

## Installing the project

Clone the project repo:

HTTPS:
```
https://github.com/CMIW/Typing-Practice-tui.git
```

SSH:
```
git@github.com:CMIW/Typing-Practice-tui.git
```

GitHub CLI:
```
gh repo clone CMIW/Typing-Practice-tui
```

## Using the project

First you need to create a .txt and save the text you want to practice with on that file. Open the project location on a terminal.

Then on a terminal run
```
cargo run <filename/file path>
# Ex
cargo run /home/medaka/Downloads/typing.txt
```

or build the project with

```
cargo build --release
```

move to the location of the binary or executable file "./target/release/" and run command

```
tui_typing_practice <filename/file path>
# Ex
tui_typing_practice /home/medaka/Downloads/typing.txt
```
