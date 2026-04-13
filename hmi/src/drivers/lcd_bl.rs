use crate::board::BacklightResources;
use defmt::{assert, unwrap};
use embassy_rp::{
	clocks,
	gpio::Output,
	pwm::{self, SetDutyCycle},
};
use fixed::traits::ToFixed;

const PWM_FREQUENCY_HZ: u32 = 5000; // 50 kHz PWM frequency
const PWM_WRAP: u16 = 1000; // 16-bit PWM resolution

pub struct BacklightDriver {
	brightness: u8,
	en: Output<'static>,
	pwm: pwm::Pwm<'static>,
}

impl BacklightDriver {
	pub fn new(r: BacklightResources) -> Self {
		let clock_freq_hz = clocks::clk_sys_freq();
		let divider = (clock_freq_hz / (PWM_FREQUENCY_HZ * (PWM_WRAP as u32))) as u16;

		let mut config = pwm::Config::default();
		config.divider = divider.to_fixed();
		config.top = PWM_WRAP;
		config.enable = true;

		let pwm_out = pwm::Pwm::new_output_a(r.lcd_pwm, r.lcd_bl, config);

		Self {
			pwm: pwm_out,
			en: Output::new(r.lcd_en, embassy_rp::gpio::Level::Low),
			brightness: 0,
		}
	}

	pub fn enabled(&self) -> bool {
		self.en.is_set_high()
	}

	pub fn brightness(&self) -> u8 {
		self.brightness
	}

	pub fn enable(&mut self, enable: bool) {
		if enable {
			self.en.set_high();
			self.set_brightness(self.brightness);
		} else {
			self.en.set_low();
			unwrap!(
				self.pwm.set_duty_cycle_fully_on(),
				"Failed to set PWM duty cycle to fully on"
			);
		}
	}

	/// Set the PWM duty cycle based on the brightness value (0-100)
	pub fn set_brightness(&mut self, brightness: u8) {
		assert!(brightness <= 100, "Brightness must be between 0 and 100");

		// The lcd_bl pin is active low, so we need to invert the brightness value

		if brightness == 0 {
			unwrap!(
				self.pwm.set_duty_cycle_fully_on(),
				"Failed to set PWM duty cycle to fully on"
			);
		} else if brightness == 100 {
			unwrap!(
				self.pwm.set_duty_cycle_fully_off(),
				"Failed to set PWM duty cycle to fully off"
			);
		} else {
			unwrap!(
				self.pwm.set_duty_cycle_percent(100 - brightness),
				"Failed to set PWM duty cycle"
			);
		}
		self.brightness = brightness;
	}
}
