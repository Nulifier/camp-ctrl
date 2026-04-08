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

// pub fn apply_snapshot(snapshot: &ffi::UiSnapshot) {
// 	unsafe { ffi::gui_apply_snapshot(snapshot) };
// }

pub fn do_loop() {
	// lvgl::tick::inc(10);
	lvgl::timer_handler();
}
