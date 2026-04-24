use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};

use crate::board::DisplayCtrlResources;

pub(super) struct CtrlEngine {
	reset_pin: Output<'static>,
}

impl CtrlEngine {
	pub fn new(r_ctrl: DisplayCtrlResources) -> Self {
		let reset_pin = Output::new(r_ctrl.lcd_rst, embassy_rp::gpio::Level::Low);

		CtrlEngine { reset_pin }
	}

	pub async fn reset(&mut self) {
		// Timings from Waveshare demo code
		self.reset_pin.set_low();
		Timer::after(Duration::from_millis(20)).await;

		self.reset_pin.set_high();
		Timer::after(Duration::from_millis(200)).await;
	}
}
