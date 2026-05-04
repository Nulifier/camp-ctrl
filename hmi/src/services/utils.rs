use core::sync::atomic::AtomicU32;
use defmt::info;
use embassy_rp::adc;
use embassy_time::{Duration, Instant, Ticker, Timer};

use crate::board::{self, SystemResources};

#[embassy_executor::task]
pub async fn cpu_temperature(r: SystemResources) -> ! {
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

static SLEEP_TICKS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
pub async fn cpu_usage() {
	let mut previous_tick = 0u32;
	let mut previous_sleep_tick = 0u32;
	let mut ticker = Ticker::every(Duration::from_secs(1));

	loop {
		ticker.next().await;

		let current_tick = Instant::now().as_ticks() as u32;
		let current_sleep_tick = SLEEP_TICKS.load(core::sync::atomic::Ordering::Relaxed);
		let sleep_tick_difference = (current_sleep_tick - previous_sleep_tick) as f32;
		let tick_difference = (current_tick - previous_tick) as f32;
		let usage = 1.0f32 - (sleep_tick_difference / tick_difference);

		previous_tick = current_tick;
		previous_sleep_tick = current_sleep_tick;

		info!("CPU Usage: {}%", usage * 100.0);
	}
}

#[allow(dead_code)]
pub fn track_cpu_usage() -> ! {
	loop {
		cortex_m::interrupt::free(|_cs| {
			let before = Instant::now().as_ticks() as u32;
			cortex_m::asm::wfi();
			let after = Instant::now().as_ticks() as u32;
			SLEEP_TICKS.fetch_add(after - before, core::sync::atomic::Ordering::Relaxed);
		});
	}
}
