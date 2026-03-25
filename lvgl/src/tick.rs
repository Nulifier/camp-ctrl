/// Increment the LVGL tick count by the specified number of milliseconds.
/// Either this should be called periodically (recommeded every 5ms) or a function should be
/// provided to set_tick_fn that returns the current tick count in milliseconds.
pub fn inc(period_ms: u32) {
	unsafe { lvgl_sys::lv_tick_inc(period_ms) };
}

/// Get the elapsed milliseconds since startup.
pub fn get() -> u32 {
	unsafe { lvgl_sys::lv_tick_get() }
}

/// Set a custom tick function that returns the current tick count in milliseconds. This is an
/// alternative to calling `inc` periodically, and allows LVGL to get the current tick count
/// directly from the function.
pub fn set_get_fn(tick_fn: extern "C" fn() -> u32) {
	unsafe { lvgl_sys::lv_tick_set_cb(Some(tick_fn)) };
}

/// Delay the current thread for the specified number of milliseconds.
/// This is either a blocking delay or a non-blocking function provided to set_delay_fn.
pub fn delay(ms: u32) {
	unsafe { lvgl_sys::lv_delay_ms(ms) };
}

/// Set a custom delay function that takes the number of milliseconds to delay. This can be used
/// to provide a non-blocking delay function that allows other tasks to run while waiting.
pub fn set_delay_fn<F>(delay_fn: extern "C" fn(u32)) {
	unsafe { lvgl_sys::lv_delay_set_cb(Some(delay_fn)) };
}
