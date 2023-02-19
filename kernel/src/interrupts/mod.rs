//! This module contains architecture agnostic interruptor handlers
mod keyboard;
mod timer;

pub use keyboard::handler as keyboard_handler;
pub use timer::handler as timer_handler;
