use defmt::info;
use embassy_rp::adc;
use embassy_time::{Duration, Timer};

use crate::board::{self, SystemResources};

#[embassy_executor::task]
pub async fn gui_task(r: SystemResources) -> ! {
	info!("Starting GUI task");

	let mut adc = adc::Adc::new(r.adc, board::Irqs, adc::Config::default());
	let mut temp_sensor = adc::Channel::new_temp_sensor(r.temp_sensor);

	loop {
		// Read the temperature sensor
		let temp_sensor = adc.read(&mut temp_sensor).await.unwrap();
		let temp_celsius = convert_adc_to_celsius(temp_sensor);
		info!("Temperature: {}°C", temp_celsius);

		// Wait for 1 second
		Timer::after(Duration::from_secs(5)).await;
	}
}

fn convert_adc_to_celsius(adc_value: u16) -> f32 {
	let temp = 27.0 - (adc_value as f32 * 3.3 / 4096.0 - 0.706) / 0.001721;
	let sign = if temp < 0.0 { -1.0 } else { 1.0 };
	let rounded_temp_x10: i16 = ((temp * 10.0) + 0.5 * sign) as i16;
	(rounded_temp_x10 as f32) / 10.0
}
