use std::sync::atomic::{AtomicBool, Ordering};

use colored::{ColoredString, Colorize};

static VERBOSE: AtomicBool = AtomicBool::new(false);

/// Set the verbose flag for logging
pub fn set_debugging(verbose: bool) {
    VERBOSE.store(verbose, Ordering::Relaxed);
}

/// Get the verbose flag
pub fn debugging() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// Utility logging function
fn log(prefix: ColoredString, msg: String, is_error: bool) {
    let output: String = format!("{}\n    {}\n", prefix, msg);

    if is_error {
        eprintln!("{}", output);
    } else {
        println!("{}", output);
    }
}

/// Log a message to stdout
pub fn log_message(msg: String) {
    log("[Parser Info]".green().bold(), msg, false);
}

/// Log a debugging message to stdout
pub fn log_debug(msg: String) {
    if !debugging() {
        return;
    }

    log("[Parser Debug]".blue().bold(), msg, false);
}

/// Log a message to stderr
pub fn log_error(msg: String) {
    log("[Parser Error]".red().bold(), msg, true);
}
