//! The desktop entry point.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    rate_it_lib::run();
}
