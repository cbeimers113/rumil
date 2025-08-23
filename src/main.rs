mod tokenizer;

use std::{
    env::args,
    error::Error,
    path::PathBuf,
    vec::Vec,
};

use crate::tokenizer::{
    Token,
    tokenize
};

/// Check if we were given a valid filepath
/// ### Arguments:
/// * args: Vec<String> - command line arguments
/// ### Returns:
/// * Result<PathBuf, String> -
///   * PathBuf is valid PathBuf to Rumil source file
///   * String is error message if something goes wrong
fn check_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.get(0) {
        Some(arg) => {
            let path = PathBuf::from(arg);
            if path.exists() {
                Ok(path)
            } else {
                Err(format!("File '{}' does not exist", arg))
            }
        },
        None => Err("No arguments were supplied".to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line args minus the binary file
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    // Get the source file
    let src_file: PathBuf = check_args(args)?;
    println!("Running '{}'...", src_file.display());

    // Tokenize the source code
    let tokens: Vec<Token> = tokenize(src_file)?;

    tokens.iter().for_each(|token: &Token| println!("{}", token));

    Ok(())
}
