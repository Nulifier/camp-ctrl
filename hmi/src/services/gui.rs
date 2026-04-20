use crate::{
	board::{
		DisplayCtrlResources, DisplayDataResources, DisplayFillResources, DisplayTimingResources,
	},
	drivers::display::DisplayDriver,
};
use defmt::info;

#[embassy_executor::task]
pub async fn gui_task(
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
	r_fill: DisplayFillResources,
) -> ! {
	info!("Starting GUI task");

	// // Create a display
	// let display = crate::gui::display::create_rp_display();
	// display.set_default();

	let mut disp = DisplayDriver::new(r_ctrl, r_timing, r_data, r_fill);
	disp.initialize().await.unwrap();

	disp.start().unwrap();

	loop {
		// Wait for 1 second
		// Timer::after(Duration::from_secs(10)).await;

		disp.push_test().await.unwrap();
	}
}
