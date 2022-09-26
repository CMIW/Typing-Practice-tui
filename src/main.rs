use std::fs;
use std::env;
use std::process;
use std::path::Path;

use tui_typing_practice::program_tui::run_tui;
use typing_state::TypingState;

fn main() {
    // read any command line arguments passed to it and then collect the values into a vector
    let args: Vec<String> = env::args().collect();

    // You need at least one argument with the path of the file used to practice
    if args.len() < 2 {
        eprintln!("Please include the file you want to practice with.\nLike tui_typing_practice /home/medaka/Downloads/typing.txt");
        process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    if !file_path.is_file() {
        eprintln!("The file {:#?} does not exist.", file_path.file_name().unwrap());
        process::exit(1);
    }

    // Read the contents of the file
    let mut file_contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Replace new lines for whitespaces
    //file_contents = file_contents.replace('\n', " ");

    // Remove the whitespace at the end of the files.
    file_contents.pop().unwrap();

    // Init state
    let mut typing_state = TypingState::new(&file_contents);

    // Run the programs terminal user interface
    let _result = run_tui(&mut typing_state).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
