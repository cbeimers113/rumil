use std::{ffi::CString, os::raw::c_char, ptr};

#[repr(C)]
pub struct Ast {}

impl Ast {
    /// Create a new C++ compatible AST structure on the heap
    pub fn new() -> Box<Self> {
        Box::new(Ast {})
    }
}

/// Free a single C string
unsafe fn free_c_string(s: *const c_char) {
    if s.is_null() {
        return;
    }

    unsafe {
        drop(CString::from_raw(s as *mut c_char));
    }
}

/// Free the entire AST and all its heap-allocated resources
#[unsafe(no_mangle)]
pub extern "C" fn free_ast(ast: *mut Ast) {
    if ast.is_null() {
        return;
    }

    unsafe {
        let owned_ast: Box<Ast> = Box::from_raw(ast);

        // Free fields
        // free_c_string(owned_ast.some_str), etc...

        // Free AST
        drop(owned_ast);
    }
}
