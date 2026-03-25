use lvgl::display::LvDisplay;

pub mod ffi;
pub mod gui;

pub const DISPLAY_WIDTH: i32 = 800;
pub const DISPLAY_HEIGHT: i32 = 480;

pub fn initialize() {
	lvgl::init();

	// Create display
	let disp = lvgl::display::SdlDisplay::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
		.expect("Failed to create display");
	disp.set_default();

	// Create input devices
	lvgl::input_device::SdlMouse::new().expect("Failed to create mouse input device");

	let _gui = gui::Gui::new();

	// let ret = unsafe { ffi::gui_init() };
	// if ret != 0 {
	// 	panic!("Failed to initialize GUI");
	// }
}

// pub fn apply_snapshot(snapshot: &ffi::UiSnapshot) {
// 	unsafe { ffi::gui_apply_snapshot(snapshot) };
// }

pub fn do_loop() {
	// lvgl::tick::inc(10);
	lvgl::timer_handler();
}
