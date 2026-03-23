use std::thread::sleep;
use std::time::Duration;

use hmi::ffi::UiSnapshot;
use hmi::{apply_snapshot, do_loop};

#[unsafe(no_mangle)]
pub extern "C" fn rust_on_reset_button() {
	println!("Reset button pressed!");
}

fn main() {
	hmi::initialize();

	loop {
		let snap = UiSnapshot {
			tank_level_pct: 42,
			solar_watts: 123,
			charging: true,
		};

		apply_snapshot(&snap);
		do_loop(10);

		sleep(Duration::from_millis(10));
	}
}
