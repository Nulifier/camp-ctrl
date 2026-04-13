use defmt::info;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn gui_task() -> ! {
	info!("Starting GUI task");

	// Create a display
	let _display = crate::gui::display::RpDisplay::new();

	loop {
		// Wait for 1 second
		Timer::after(Duration::from_secs(1)).await;
	}
}
