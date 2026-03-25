#![no_std]

use widgets::obj::ObjRef;

pub mod display;
pub mod error;
pub mod event;
pub mod input_device;
pub mod layouts;
pub mod misc;
pub mod style;
pub mod text;
pub mod tick;
pub mod widgets;

/// Initializes the LVGL library. This should be called once at the start of the program.
pub fn init() {
	// We can call this multiple times, but only the first call will actually do anything.
	unsafe { lvgl_sys::lv_init() };
}

pub fn active_screen() -> Option<ObjRef<'static>> {
	let screen = unsafe { lvgl_sys::lv_screen_active() };
	widgets::obj::ObjRef::from_raw(screen)
}

/// Should be called periodically to allow LVGL to handle timers and other background tasks.
/// Returns the number of milliseconds until the next timer is due, or None if there are no timers.
pub fn timer_handler() -> Option<u32> {
	let time_until_next = unsafe { lvgl_sys::lv_timer_handler() };
	if time_until_next == u32::MAX {
		None
	} else {
		Some(time_until_next)
	}
}
