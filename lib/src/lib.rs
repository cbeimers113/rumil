mod ast;
mod expr;
mod lexer;
mod log;
mod parser;
mod token;

use std::{ffi::CStr, fs::read_to_string, os::raw::c_char, ptr::null_mut};

use crate::{
    ast::Ast,
    expr::Expr,
    lexer::scan,
    log::{log_error, log_message, set_debugging},
    parser::parse,
    token::Token,
};

/// Parses the source file passed in and returns a pointer to the resulting AST in memory.
/// If any errors arise, they are printed to stderr and a null pointer is returned.
#[unsafe(no_mangle)]
pub extern "C" fn parse_file(filepath: *const c_char, verbose: bool) -> *mut Ast {
    set_debugging(verbose);
    let r_filepath: String;

    // Ingest C string for filepath
    unsafe {
        r_filepath = CStr::from_ptr(filepath).to_string_lossy().into_owned();
        log_message(format!("Reading {}...", r_filepath));
    }

    // Read source code file
    let source_code: String = match read_to_string(&r_filepath) {
        Ok(contents) => contents,
        Err(msg) => {
            log_error(format!("Error reading {}: {}", r_filepath, msg));
            return null_mut();
        }
    };

    // Tokenize source code
    log_message("Scanning source code...".to_owned());
    let tokens: Vec<Token> = match scan(source_code, &r_filepath) {
        Ok(tk) => tk,
        Err(msg) => {
            log_error(msg);
            return null_mut();
        }
    };

    // Parse tokens into syntax tree
    log_message("Parsing tokens...".to_owned());
    let syntax_tree: Box<dyn Expr> = match parse(tokens, &r_filepath) {
        Ok(st) => st,
        Err(msg) => {
            log_error(msg);
            return null_mut();
        }
    };

    Box::into_raw(Ast::new())
}
