use std::env;

pub fn is_debug() -> bool {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "-d" || arg == "--debug" {
            return true;
        }
    }
    return false;
}
