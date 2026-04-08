// #[cfg(not(target_os = "linux"))]
// compile_error!("The hmi-sim crate is only supported on Linux");

use lvgl::display::LvDisplay;
use std::sync::LazyLock;
use std::thread::sleep;
use std::time::{Duration, Instant};

use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH, do_loop, gui};

static PROGRAM_START: LazyLock<Instant> = LazyLock::new(Instant::now);

extern "C" fn get_millis() -> u32 {
	PROGRAM_START.elapsed().as_millis() as u32
}

fn main() {
	hmi_gui::initialize();

	let disp = lvgl::display::SdlDisplay::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
		.expect("Failed to create display");
	disp.set_default();

	// Create input devices
	lvgl::input_device::SdlMouse::new().expect("Failed to create mouse input device");

	let _gui = gui::Gui::new();

	lvgl::tick::set_get_fn(get_millis);

	loop {
		// let snap = UiSnapshot {
		// 	tank_level_pct: 42,
		// 	solar_watts: 123,
		// 	charging: true,
		// };

		// apply_snapshot(&snap);
		do_loop();

		sleep(Duration::from_millis(10));
	}
}
