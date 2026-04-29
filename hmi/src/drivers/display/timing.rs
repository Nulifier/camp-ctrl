use crate::board::DisplayTimingResources;
use crate::drivers::display::PCLK_FREQUENCY;
use crate::error::{Error, Result};
use defmt::info;
use embassy_rp::pio::{PinConfig, PioBatch};
use embassy_rp::{peripherals, pio};
use fixed::types::U24F8;
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub(super) struct TimingEngine {
	pub common: pio::Common<'static, peripherals::PIO0>,

	pub sm0: pio::StateMachine<'static, peripherals::PIO0, 0>,
	pub sm1: pio::StateMachine<'static, peripherals::PIO0, 1>,

	pub vsync_pin: pio::Pin<'static, peripherals::PIO0>,
	pub hsync_pin: pio::Pin<'static, peripherals::PIO0>,
	pub pclk_pin: pio::Pin<'static, peripherals::PIO0>,
}

impl TimingEngine {
	pub fn new(r: DisplayTimingResources) -> Self {
		let mut pio = pio::Pio::new(r.pio_timing, crate::board::Irqs);

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

	pub fn init(&mut self) -> Result<()> {
		// Computer a divider for the PIO state machines to achieve the desired pixel clock frequency
		let sys_clk_hz = embassy_rp::clocks::clk_sys_freq();
		let sys_clk_khz = U24F8::from_num(sys_clk_hz / 1000);
		let pclk_khz = U24F8::from_num(PCLK_FREQUENCY / 1000);

		// Each pclk pulse requires 2 state machine cycles (one for setting HSYNC low, one for setting it high)
		let divider = sys_clk_khz / (2 * pclk_khz);

		if pclk_khz == U24F8::ZERO {
			return Err(Error::InvalidDisplayTiming);
		}

		// Load the PIO program for generating the HSYNC and PCLK signals
		let hsync_prg = super::pio_progs::load_hsync_program(&mut self.common)?;
		let mut hsync_cfg = pio::Config::default();
		hsync_cfg.use_program(&hsync_prg, &[&self.hsync_pin, &self.pclk_pin]);
		hsync_cfg.clock_divider = divider;
		hsync_cfg.fifo_join = pio::FifoJoin::TxOnly;
		unsafe {
			hsync_cfg.set_pins(PinConfig {
				in_base: 0,
				out_base: 0,
				out_count: 0,
				set_base: 0,
				set_count: 0,
				sideset_base: 22, // HSYNC on bit0, PCLK on bit1
				sideset_count: 2,
			})
		};

		self.sm0.set_config(&hsync_cfg);
		self.sm0
			.set_pin_dirs(pio::Direction::Out, &[&self.hsync_pin, &self.pclk_pin]);

		// Load the PIO program for generating the VSYNC signal
		let vsync_prg = super::pio_progs::load_vsync_program(&mut self.common)?;
		let mut vsync_cfg = pio::Config::default();
		vsync_cfg.use_program(&vsync_prg, &[&self.vsync_pin]);
		vsync_cfg.fifo_join = pio::FifoJoin::TxOnly;
		// vsync_cfg.clock_divider = divider;
		unsafe {
			vsync_cfg.set_pins(PinConfig {
				in_base: 0,
				out_base: 0,
				out_count: 0,
				set_base: 0,
				set_count: 0,
				sideset_base: 21, // VSYNC on bit0
				sideset_count: 1,
			})
		};

		self.sm1.set_config(&vsync_cfg);
		self.sm1
			.set_pin_dirs(pio::Direction::Out, &[&self.vsync_pin]);

		// Pass the parameters to the PIO programs

		// Pass the width to the HSYNC program
		self.sm0.tx().push((DISPLAY_WIDTH - 1) as u32);
		// Pass the height to the VSYNC program
		self.sm1.tx().push((DISPLAY_HEIGHT - 1) as u32);

		Ok(())
	}

	pub fn start(&mut self) -> Result<()> {
		// Reset the state machine clock dividers
		self.sm0.clkdiv_restart();
		self.sm1.clkdiv_restart();

		// Start the state machines
		{
			let mut batch = PioBatch::new();
			batch.set_enable(&mut self.sm0, true);
			batch.set_enable(&mut self.sm1, true);

			batch.restart(&mut self.sm0);
			batch.restart(&mut self.sm1);

			batch.execute();
		}

		Ok(())
	}
}
