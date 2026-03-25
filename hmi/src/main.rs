use std::sync::LazyLock;
use std::thread::sleep;
use std::time::{Duration, Instant};

use hmi::do_loop;

#[unsafe(no_mangle)]
pub extern "C" fn rust_on_reset_button() {
	println!("Reset button pressed!");
}

static PROGRAM_START: LazyLock<Instant> = LazyLock::new(Instant::now);

extern "C" fn get_millis() -> u32 {
	PROGRAM_START.elapsed().as_millis() as u32
}

fn main() {
	hmi::initialize();

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
