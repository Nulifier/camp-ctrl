use defmt::info;
use embassy_time::{Duration, Timer};
// use lvgl::display::LvDisplay;

use crate::{
	board::{DisplayCtrlResources, DisplayDataResources, DisplayTimingResources},
	drivers::display::DisplayDriver,
};

#[embassy_executor::task]
pub async fn gui_task(
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
) -> ! {
	info!("Starting GUI task");

	// // Create a display
	// let display = crate::gui::display::create_rp_display();
	// display.set_default();

	let mut disp = DisplayDriver::new(r_ctrl, r_timing, r_data);
	disp.initialize().await.unwrap();

	disp.start().unwrap();

	loop {
		// Wait for 1 second
		// Timer::after(Duration::from_secs(10)).await;

		disp.push_test().await.unwrap();
	}
}
