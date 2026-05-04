#![no_std]

extern crate alloc;

pub mod future;
pub mod gui;
pub mod misc;

pub const DISPLAY_WIDTH: i32 = 800;
pub const DISPLAY_HEIGHT: i32 = 480;

pub fn initialize() {
	lvgl::init();
}

/// Runs one iteration of the GUI loop, processing events and updating the display as needed.
/// Returns `Some(call_again_after_ms)` for when to call this function again.
pub fn do_loop() -> u32 {
	lvgl::timer_handler()
}
