use crate::{
	board::{DisplayCtrlResources, DisplayDataResources, DisplayTimingResources},
	error::Result,
};
use embassy_rp::{
	bind_interrupts,
	gpio::{Level, Output},
	peripherals,
	pio::InterruptHandler,
};
use embassy_time::{Duration, Timer};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

mod buffers;
mod pio_progs;
mod rgb;
mod timing;

const PCLK_FREQUENCY: u32 = 25_000_000; // 25 MHz

/// Number of lines of pixel data to buffer in SRAM (as opposed to PSRAM)
const SRAM_BUFFER_LINES: usize = 4;

bind_interrupts!(struct Irqs {
	PIO0_IRQ_0 => InterruptHandler<peripherals::PIO0>;
	PIO1_IRQ_0 => InterruptHandler<peripherals::PIO1>;
});

pub struct DisplayDriver {
	reset_pin: Output<'static>,

	timing_engine: timing::TimingEngine,
	rgb_engine: rgb::RgbEngine,

	sram_buffer_0: [u16; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES],
	sram_buffer_1: [u16; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES],
}

impl DisplayDriver {
	pub fn new(
		r_ctrl: DisplayCtrlResources,
		r_timing: DisplayTimingResources,
		r_data: DisplayDataResources,
	) -> Self {
		let reset_pin = Output::new(r_ctrl.lcd_rst, Level::Low);
		let timing_engine = timing::TimingEngine::new(r_timing);
		let rgb_engine = rgb::RgbEngine::new(r_data);

		DisplayDriver {
			reset_pin,
			timing_engine,
			rgb_engine,
			sram_buffer_0: [0; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES],
			sram_buffer_1: [0; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES],
		}
	}

	pub async fn initialize(&mut self) -> Result<()> {
		// Initialize test pattern in SRAM buffers
		for line in 0..SRAM_BUFFER_LINES {
			for x in 0..DISPLAY_WIDTH as usize {
				let color = match x {
					0..=199 => lvgl::misc::color::Color16::from_rgb8(255, 0, 0), // Red
					200..=399 => lvgl::misc::color::Color16::from_rgb8(0, 255, 0), // Green
					400..=599 => lvgl::misc::color::Color16::from_rgb8(0, 0, 255), // Blue
					600..=799 => lvgl::misc::color::Color16::from_rgb8(255, 0, 255), // Magenta
					_ => lvgl::misc::color::Color16::from_rgb8(0, 0, 0),         // Black
				};

				let color_u16 = unsafe { core::mem::transmute(color) };

				if line == 1 {
					self.sram_buffer_0[line * DISPLAY_WIDTH as usize + x] = color_u16;
					self.sram_buffer_1[line * DISPLAY_WIDTH as usize + x] = color_u16;
				} else {
					self.sram_buffer_0[line * DISPLAY_WIDTH as usize + x] = color_u16;
					self.sram_buffer_1[line * DISPLAY_WIDTH as usize + x] = color_u16;
				}
			}
		}

		self.reset().await;
		self.timing_engine.init()?;
		self.rgb_engine.init()?;

		Ok(())
	}

	pub async fn reset(&mut self) {
		// Timings from Waveshare demo code
		self.reset_pin.set_low();
		Timer::after(Duration::from_millis(20)).await;
		self.reset_pin.set_high();
		Timer::after(Duration::from_millis(200)).await;
	}

	pub fn start(&mut self) -> Result<()> {
		self.timing_engine.start()?;
		self.rgb_engine.start()?;

		Ok(())
	}

	pub async fn push_test(&mut self) -> Result<()> {
		self.rgb_engine.push_test(&self.sram_buffer_0).await?;

		Ok(())
	}
}
