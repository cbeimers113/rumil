mod ast;

use std::os::raw::c_char;

use crate::ast::Ast;

/// Parses the source file passed in and returns a pointer to the
/// resulting AST in memory, or panics if the source file doesn't exist.
#[unsafe(no_mangle)]
pub extern "C" fn parse_file(filepath: *const c_char, verbose: bool) -> *mut Ast {
    let ast: Box<Ast> = Ast::new();

    Box::into_raw(ast)
}
