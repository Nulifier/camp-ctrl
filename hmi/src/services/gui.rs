use defmt::info;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn gui_task() -> ! {
	info!("Starting GUI task");

	loop {
		// Wait for 1 second
		Timer::after(Duration::from_secs(10)).await;
	}
}
