pub mod ffi;

pub fn initialize() {
	let ret = unsafe { ffi::gui_init() };
	if ret != 0 {
		panic!("Failed to initialize GUI");
	}
}

pub fn apply_snapshot(snapshot: &ffi::UiSnapshot) {
	unsafe { ffi::gui_apply_snapshot(snapshot) };
}

pub fn do_loop(ms: u32) {
	unsafe {
		ffi::gui_tick_inc(ms);
		ffi::gui_task_handler();
	}
}
