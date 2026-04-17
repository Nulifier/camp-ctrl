use crate::{
	board::{DisplayCtrlResources, DisplayDataResources, DisplayTimingResources},
	error::{Error, Result},
};
use defmt::info;
use embassy_rp::{
	bind_interrupts,
	gpio::{self, Level, Output},
	peripherals,
	pio::{self, InterruptHandler, PinConfig, Pio, program::pio_asm},
};
use embassy_time::{Duration, Timer};
use fixed::types::U24F8;
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

mod buffers;
mod pio_progs;

const PCLK_FREQUENCY: u32 = 25_000_000; // 25 MHz

bind_interrupts!(struct Irqs {
	PIO0_IRQ_0 => InterruptHandler<peripherals::PIO0>;
	PIO1_IRQ_0 => InterruptHandler<peripherals::PIO1>;
});

#[derive(Debug, Clone, Copy)]
struct Timing {
	pub width: usize,
	pub height: usize,

	// Units are in pixel clock cycles
	pub hsync_back_porch: u16,
	pub hsync_front_porch: u16,
	pub hsync_pulse: u16,

	// Units are in hsync cycles
	pub vsync_back_porch: u16,
	pub vsync_front_porch: u16,
	pub vsync_pulse: u16,

	pub pclk_hz: u32,
}

impl Timing {
	pub const fn default_st7262() -> Self {
		// Timings from ST7262 datasheet
		Self {
			width: DISPLAY_WIDTH as usize,
			height: DISPLAY_HEIGHT as usize,

			hsync_back_porch: 8,  // From 4 to 48
			hsync_front_porch: 8, // From 4 to 48
			hsync_pulse: 4,       // From 2 to 8

			vsync_back_porch: 8,  // From 4 to 12
			vsync_front_porch: 8, // From 4 to 12
			vsync_pulse: 4,       // From 2 to 8

			pclk_hz: PCLK_FREQUENCY,
		}
	}
}

struct TimingEngine {
	pub common: pio::Common<'static, peripherals::PIO0>,

	pub sm0: pio::StateMachine<'static, peripherals::PIO0, 0>,
	pub sm1: pio::StateMachine<'static, peripherals::PIO0, 1>,

	pub vsync_pin: pio::Pin<'static, peripherals::PIO0>,
	pub hsync_pin: pio::Pin<'static, peripherals::PIO0>,
	pub pclk_pin: pio::Pin<'static, peripherals::PIO0>,
}

impl TimingEngine {
	pub fn new(r: DisplayTimingResources) -> Self {
		let mut pio = Pio::new(r.pio_timing, Irqs);

		let vsync_pin = pio.common.make_pio_pin(r.lcd_vsync);
		let hsync_pin = pio.common.make_pio_pin(r.lcd_hsync);
		let pclk_pin = pio.common.make_pio_pin(r.lcd_pclk);

		TimingEngine {
			common: pio.common,
			sm0: pio.sm0,
			sm1: pio.sm1,
			vsync_pin,
			hsync_pin,
			pclk_pin,
		}
	}

	pub fn init(&mut self, timing: &Timing) -> Result<()> {
		// Computer a divider for the PIO state machines to achieve the desired pixel clock frequency
		let sys_clk_hz = embassy_rp::clocks::clk_sys_freq();
		let sys_clk_khz = U24F8::from_num(sys_clk_hz / 1000);
		let pclk_khz = U24F8::from_num(timing.pclk_hz / 1000);

		// Each pclk pulse requires 2 state machine cycles (one for setting HSYNC low, one for setting it high)
		let divider = sys_clk_khz / (2 * pclk_khz);

		if pclk_khz == U24F8::ZERO {
			return Err(Error::InvalidDisplayTiming);
		}

		// Load the PIO program for generating the VSYNC signal
		let vsync_prg = pio_progs::load_vsync_program(&mut self.common)?;
		let mut vsync_cfg = pio::Config::default();
		vsync_cfg.use_program(&vsync_prg, &[&self.vsync_pin]);
		vsync_cfg.fifo_join = pio::FifoJoin::TxOnly;
		// vsync_cfg.clock_divider = divider;

		self.sm0.set_config(&vsync_cfg);
		self.sm0
			.set_pin_dirs(pio::Direction::Out, &[&self.vsync_pin]);

		// Load the PIO program for generating the HSYNC and PCLK signals
		let hsync_prg = pio_progs::load_hsync_program(&mut self.common)?;
		let mut hsync_cfg = pio::Config::default();
		hsync_cfg.use_program(&hsync_prg, &[&self.hsync_pin, &self.pclk_pin]);
		hsync_cfg.clock_divider = divider;
		hsync_cfg.fifo_join = pio::FifoJoin::TxOnly;

		self.sm1.set_config(&hsync_cfg);
		self.sm1
			.set_pin_dirs(pio::Direction::Out, &[&self.hsync_pin, &self.pclk_pin]);

		// Pass the parameters to the PIO programs

		// Pass the height to the VSYNC program
		self.sm0.tx().push((DISPLAY_HEIGHT - 1) as u32);
		// Pass the width to the HSYNC program
		self.sm1.tx().push((DISPLAY_WIDTH - 1) as u32);

		Ok(())
	}

	pub fn start(&mut self) -> Result<()> {
		// Reset the state machine clock dividers
		self.sm0.clkdiv_restart();
		self.sm1.clkdiv_restart();

		// Start the state machines
		self.common.apply_sm_batch(|b| {
			// Enable
			b.set_enable(&mut self.sm0, true);
			b.set_enable(&mut self.sm1, true);
			// b.set_enable(&mut self.sm2, true);
			// b.set_enable(&mut self.sm3, true);

			// Restart
			b.restart(&mut self.sm0);
			b.restart(&mut self.sm1);
			// b.restart(&mut self.sm2);
			// b.restart(&mut self.sm3);
		});

		Ok(())
	}
}

struct RgbEngine {
	pub common2: pio::Common<'static, peripherals::PIO1>,

	pub sm0b: pio::StateMachine<'static, peripherals::PIO1, 0>,
	pub sm1b: pio::StateMachine<'static, peripherals::PIO1, 1>,

	pub de_pin: pio::Pin<'static, peripherals::PIO1>,

	pub b3_pin: pio::Pin<'static, peripherals::PIO1>,
	pub b4_pin: pio::Pin<'static, peripherals::PIO1>,
	pub b5_pin: pio::Pin<'static, peripherals::PIO1>,
	pub b6_pin: pio::Pin<'static, peripherals::PIO1>,
	pub b7_pin: pio::Pin<'static, peripherals::PIO1>,

	pub g2_pin: pio::Pin<'static, peripherals::PIO1>,
	pub g3_pin: pio::Pin<'static, peripherals::PIO1>,
	pub g4_pin: pio::Pin<'static, peripherals::PIO1>,
	pub g5_pin: pio::Pin<'static, peripherals::PIO1>,
	pub g6_pin: pio::Pin<'static, peripherals::PIO1>,
	pub g7_pin: pio::Pin<'static, peripherals::PIO1>,

	pub r3_pin: pio::Pin<'static, peripherals::PIO1>,
	pub r4_pin: pio::Pin<'static, peripherals::PIO1>,
	pub r5_pin: pio::Pin<'static, peripherals::PIO1>,
	pub r6_pin: pio::Pin<'static, peripherals::PIO1>,
	pub r7_pin: pio::Pin<'static, peripherals::PIO1>,
}

impl RgbEngine {
	pub fn new(r: DisplayDataResources) -> Self {
		let mut pio_rgb = Pio::new(r.pio_rgb, Irqs);

		let de_pin = pio_rgb.common.make_pio_pin(r.lcd_de);
		let b3_pin = pio_rgb.common.make_pio_pin(r.lcd_b3);
		let b4_pin = pio_rgb.common.make_pio_pin(r.lcd_b4);
		let b5_pin = pio_rgb.common.make_pio_pin(r.lcd_b5);
		let b6_pin = pio_rgb.common.make_pio_pin(r.lcd_b6);
		let b7_pin = pio_rgb.common.make_pio_pin(r.lcd_b7);

		let g2_pin = pio_rgb.common.make_pio_pin(r.lcd_g2);
		let g3_pin = pio_rgb.common.make_pio_pin(r.lcd_g3);
		let g4_pin = pio_rgb.common.make_pio_pin(r.lcd_g4);
		let g5_pin = pio_rgb.common.make_pio_pin(r.lcd_g5);
		let g6_pin = pio_rgb.common.make_pio_pin(r.lcd_g6);
		let g7_pin = pio_rgb.common.make_pio_pin(r.lcd_g7);

		let r3_pin = pio_rgb.common.make_pio_pin(r.lcd_r3);
		let r4_pin = pio_rgb.common.make_pio_pin(r.lcd_r4);
		let r5_pin = pio_rgb.common.make_pio_pin(r.lcd_r5);
		let r6_pin = pio_rgb.common.make_pio_pin(r.lcd_r6);
		let r7_pin = pio_rgb.common.make_pio_pin(r.lcd_r7);

		RgbEngine {
			common2: pio_rgb.common,
			sm0b: pio_rgb.sm0,
			sm1b: pio_rgb.sm1,
			de_pin,
			b3_pin,
			b4_pin,
			b5_pin,
			b6_pin,
			b7_pin,
			g2_pin,
			g3_pin,
			g4_pin,
			g5_pin,
			g6_pin,
			g7_pin,
			r3_pin,
			r4_pin,
			r5_pin,
			r6_pin,
			r7_pin,
		}
	}

	pub fn init(&mut self, _timing: &Timing) -> Result<()> {
		// Load the PIO program for ouputting the DE signal
		let de_prg = pio_progs::load_rgb_de_program(&mut self.common2)?;
		let mut de_cfg = pio::Config::default();
		de_cfg.use_program(&de_prg, &[&self.de_pin]);
		// de_cfg.clock_divider = divider;
		de_cfg.fifo_join = pio::FifoJoin::TxOnly;

		self.sm0b.set_config(&de_cfg);
		self.sm0b.set_pin_dirs(pio::Direction::Out, &[&self.de_pin]);

		// Load the PIO program for ouputting the RGB data
		let rgb_pins = [
			&self.b3_pin,
			&self.b4_pin,
			&self.b5_pin,
			&self.b6_pin,
			&self.b7_pin,
			&self.g2_pin,
			&self.g3_pin,
			&self.g4_pin,
			&self.g5_pin,
			&self.g6_pin,
			&self.g7_pin,
			&self.r3_pin,
			&self.r4_pin,
			&self.r5_pin,
			&self.r6_pin,
			&self.r7_pin,
		];
		let rgb_prg = pio_progs::load_rgb_program(&mut self.common2)?;
		let mut rgb_cfg = pio::Config::default();
		rgb_cfg.use_program(&rgb_prg, &[]);
		rgb_cfg.set_out_pins(&rgb_pins);
		// rgb_cfg.clock_divider = divider;
		rgb_cfg.fifo_join = pio::FifoJoin::TxOnly;

		self.sm1b.set_config(&rgb_cfg);
		self.sm1b.set_pins(gpio::Level::Low, &rgb_pins);
		self.sm1b.set_pin_dirs(pio::Direction::Out, &rgb_pins);

		// Pass the parameters to the PIO programs

		// Pass the height to the RGB_DE program
		self.sm0b.tx().push((DISPLAY_HEIGHT - 1) as u32);
		// Pass the width to the RGB program
		self.sm1b.tx().push((DISPLAY_WIDTH - 1) as u32);
		// Pass a dummy color to the RGB program, TODO: Remove this once we have a way to stream pixel data into the PIO
		self.sm1b.tx().push(0xF800);

		Ok(())
	}

	pub fn start(&mut self) -> Result<()> {
		// Reset the state machine clock dividers
		self.sm0b.clkdiv_restart();
		self.sm1b.clkdiv_restart();

		// Start the state machines
		self.common2.apply_sm_batch(|b| {
			// Enable
			b.set_enable(&mut self.sm0b, true);
			b.set_enable(&mut self.sm1b, true);

			// Restart
			b.restart(&mut self.sm0b);
			b.restart(&mut self.sm1b);
		});

		Ok(())
	}
}

pub struct DisplayDriver {
	reset_pin: Output<'static>,

	timing: Timing,
	timing_engine: TimingEngine,
	rgb_engine: RgbEngine,
}

impl DisplayDriver {
	pub fn new(
		r_ctrl: DisplayCtrlResources,
		r_timing: DisplayTimingResources,
		r_data: DisplayDataResources,
	) -> Self {
		let reset_pin = Output::new(r_ctrl.lcd_rst, Level::Low);
		let timing_engine = TimingEngine::new(r_timing);
		let rgb_engine = RgbEngine::new(r_data);

		DisplayDriver {
			reset_pin,
			timing: Timing::default_st7262(),
			timing_engine,
			rgb_engine,
		}
	}

	pub async fn initialize(&mut self) -> Result<()> {
		self.reset().await;
		self.timing_engine.init(&self.timing)?;
		self.rgb_engine.init(&self.timing)?;

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
}
