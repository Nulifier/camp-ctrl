use crate::{
	board::{
		DisplayCtrlResources, DisplayDataResources, DisplayFillResources, DisplayTimingResources,
	},
	error::Result,
};
use embassy_rp::{
	bind_interrupts,
	gpio::{Level, Output},
	peripherals,
	pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, zerocopy_channel::Channel};
use embassy_time::{Duration, Timer};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

mod buffers;
mod fill;
mod pio_progs;
mod rgb;
mod timing;

const PCLK_FREQUENCY: u32 = 25_000_000; // 25 MHz

/// Number of lines of pixel data to buffer in SRAM (as opposed to PSRAM)
const SRAM_BUFFER_LINES: usize = 4;

const SRAM_BUFFER_COUNT: usize = 3;

static mut CHUNK_BUFFERS: [[u16; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES]; SRAM_BUFFER_COUNT] =
	[[0; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES]; SRAM_BUFFER_COUNT];

bind_interrupts!(struct Irqs {
	PIO0_IRQ_0 => InterruptHandler<peripherals::PIO0>;
	PIO1_IRQ_0 => InterruptHandler<peripherals::PIO1>;
});

pub struct DisplayDriver<'a> {
	reset_pin: Output<'static>,

	timing_engine: timing::TimingEngine,
	rgb_engine: rgb::RgbEngine,
	fill_engine: fill::FillEngine,

	chunk_channel:
		Channel<'a, ThreadModeRawMutex, [u16; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES]>,
	frame_buffers:
		buffers::DoubleBuffers<{ DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize }, u16>,
}

impl<'a> DisplayDriver<'a> {
	pub fn new(
		r_ctrl: DisplayCtrlResources,
		r_timing: DisplayTimingResources,
		r_data: DisplayDataResources,
		r_fill: DisplayFillResources,
	) -> Self {
		let reset_pin = Output::new(r_ctrl.lcd_rst, Level::Low);
		let timing_engine = timing::TimingEngine::new(r_timing);
		let rgb_engine = rgb::RgbEngine::new(r_data);
		let fill_engine = fill::FillEngine::new(r_fill);

		DisplayDriver {
			reset_pin,
			timing_engine,
			rgb_engine,
			fill_engine,
			chunk_channel: Channel::new(unsafe { &mut CHUNK_BUFFERS }),
			frame_buffers: buffers::DoubleBuffers::new(),
		}
	}

	fn chunk_buffers_test_pattern(&mut self) {
		for line in 0..SRAM_BUFFER_LINES {
			for x in 0..DISPLAY_WIDTH as usize {
				let color = match x {
					0..=99 => lvgl::misc::color::Color16::from_rgb8(255, 0, 0), // Red
					100..=199 => lvgl::misc::color::Color16::from_rgb8(255, 255, 0), // Yellow
					200..=299 => lvgl::misc::color::Color16::from_rgb8(0, 255, 0), // Green
					300..=399 => lvgl::misc::color::Color16::from_rgb8(0, 255, 255), // Cyan
					400..=499 => lvgl::misc::color::Color16::from_rgb8(0, 0, 255), // Blue
					500..=599 => lvgl::misc::color::Color16::from_rgb8(255, 0, 255), // White
					600..=699 => lvgl::misc::color::Color16::from_rgb8(255, 255, 255), // Magenta
					700..=799 => lvgl::misc::color::Color16::from_rgb8(128, 128, 128), // Gray
					_ => lvgl::misc::color::Color16::from_rgb8(0, 0, 0),        // Black
				};

				let color_u16: u16 = unsafe { core::mem::transmute(color) };

				if line >= SRAM_BUFFER_LINES / 2 {
					for i in 0..SRAM_BUFFER_COUNT {
						// self.chunk_buffers[i][line * DISPLAY_WIDTH as usize + x] =
						// 	color_u16.swap_bytes();
						self.chunk_buffers[i][line * DISPLAY_WIDTH as usize + x] = color_u16;
					}
				} else {
					for i in 0..SRAM_BUFFER_COUNT {
						self.chunk_buffers[i][line * DISPLAY_WIDTH as usize + x] = color_u16;
					}
				}
			}
		}
	}

	fn frame_buffers_test_pattern(&mut self) {
		for i in 0..(DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize) {
			unsafe {
				self.frame_buffers.active_slice_mut()[i] = 0xF800; // Red
				self.frame_buffers.back_slice_mut()[i] = 0x001F; // Blue
			}
		}
	}

	pub async fn initialize(&mut self) -> Result<()> {
		// Initialize test pattern in SRAM buffers
		self.chunk_buffers_test_pattern();
		// Initialize test pattern in PSRAM frame buffers
		self.frame_buffers_test_pattern();

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
		self.rgb_engine.start()?;
		self.timing_engine.start()?;

		Ok(())
	}

	pub async fn push_test(&mut self) -> Result<()> {
		self.rgb_engine.send_chunk(&self.chunk_buffers[0]).await?;

		Ok(())
	}
}

#[embassy_executor::task]
pub async fn display_fill_task() {}
