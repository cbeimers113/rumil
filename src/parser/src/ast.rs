use std::{ffi::CString, os::raw::c_char, ptr};

#[repr(C)]
pub struct Ast {
    pub name: *const c_char,
}

impl Ast {
    /// Create a new C++ compatible AST structure on the heap
    pub fn new(name: &str) -> Box<Self> {
        Box::new(Ast {
            name: CString::new(name).unwrap().into_raw(),
        })
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ast_name(ast: *mut Ast) -> *const c_char {
    if ast.is_null() {
        return ptr::null();
    }

    unsafe { (*ast).name }
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
        free_c_string(owned_ast.name);

        // Free AST
        drop(owned_ast);
    }
}
