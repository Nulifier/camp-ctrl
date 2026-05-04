use defmt::info;
use embassy_time::Timer;
use lvgl::display::LvDisplay;

use crate::drivers::display::FRAME_BUFFER_SWAPPED;

#[embassy_executor::task]
pub async fn gui_task() {
	info!("Starting GUI");

	hmi_gui::initialize();

	let mut display = crate::gui::display::create_rp_display().await;
	display.set_default();

	// display.refresh_now();

	lvgl::tick::set_get_fn(crate::gui::get_millis);

	info!("GUI initialized");

	let _gui = hmi_gui::gui::Gui::new();

	loop {
		let timeout_ms = hmi_gui::do_loop();

		// if let Some(_) = FRAME_BUFFER_SWAPPED.try_take() {
		// 	display.flush_ready();
		// }

		// info!("GUI loop completed, next timeout in {} ms", timeout_ms);

		Timer::after_millis(timeout_ms as u64).await;

		// display.refresh_now();
	}
}
