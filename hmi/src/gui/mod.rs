pub mod display;

pub extern "C" fn get_millis() -> u32 {
	let now = embassy_time::Instant::now();
	now.as_millis() as u32
}
