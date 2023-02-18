// TODO: Maybe this module should not be named like this
use crate::{print, println, PrintColor};

// TODO:
// init_fn should be FnOnce() -> Result<...>
// handle error cases!
pub fn init_with_message(what: &str, init_fn: impl FnOnce()) {
    print!("Initializing {what}...");
    init_fn();
    println!([PrintColor::Green], " OK");
}
