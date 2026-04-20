use defmt::info;

#[embassy_executor::task]
pub async fn fill_task() {
	loop {
		info!("Filling screen with color...");
		embassy_time::Timer::after(embassy_time::Duration::from_secs(5)).await;
	}
}
