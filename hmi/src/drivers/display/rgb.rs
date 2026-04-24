use crate::board::DisplayDataResources;
use crate::error::Result;
use defmt::info;
use embassy_rp::{
	dma, gpio, peripherals,
	pio::{self, PioBatch},
};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub(super) struct RgbEngine {
	pub common: pio::Common<'static, peripherals::PIO1>,

	pub sm0: pio::StateMachine<'static, peripherals::PIO1, 0>,
	pub sm1: pio::StateMachine<'static, peripherals::PIO1, 1>,

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

	pub dma_channel_a: dma::Channel<'static>,
	// pub dma_channel_b: dma::Channel<'static>,
}

impl RgbEngine {
	pub fn new(r: DisplayDataResources) -> Self {
		let mut pio_rgb = pio::Pio::new(r.pio_rgb, crate::board::Irqs);

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
			common: pio_rgb.common,
			sm0: pio_rgb.sm0,
			sm1: pio_rgb.sm1,
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
			dma_channel_a: dma::Channel::new(r.dma_a, crate::board::Irqs),
			// dma_channel_b: dma::Channel::new(r.dma_b, crate::board::Irqs),
		}
	}

	pub fn init(&mut self) -> Result<()> {
		// Load the PIO program for ouputting the DE signal
		let de_prg = super::pio_progs::load_rgb_de_program(&mut self.common)?;
		let mut de_cfg = pio::Config::default();
		de_cfg.use_program(&de_prg, &[&self.de_pin]);

		unsafe {
			de_cfg.set_pins(pio::PinConfig {
				in_base: 16,
				out_base: 24,
				out_count: 16,
				set_base: 16,
				set_count: 0,
				sideset_base: 20,
				sideset_count: 1,
			})
		};

		// de_cfg.clock_divider = divider;
		de_cfg.fifo_join = pio::FifoJoin::TxOnly;

		self.sm0.set_config(&de_cfg);
		self.sm0.set_pin_dirs(pio::Direction::Out, &[&self.de_pin]);

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
		let rgb_prg = super::pio_progs::load_rgb_program(&mut self.common)?;
		let mut rgb_cfg = pio::Config::default();
		rgb_cfg.use_program(&rgb_prg, &[]);
		rgb_cfg.set_out_pins(&rgb_pins);

		unsafe {
			rgb_cfg.set_pins(pio::PinConfig {
				in_base: 16,
				out_base: 24,
				out_count: 16,
				set_base: 16,
				set_count: 0,
				sideset_base: 0,
				sideset_count: 0,
			})
		};
		// rgb_cfg.clock_divider = divider;
		rgb_cfg.fifo_join = pio::FifoJoin::TxOnly;

		self.sm1.set_config(&rgb_cfg);
		self.sm1.set_pins(gpio::Level::Low, &rgb_pins);
		self.sm1.set_pin_dirs(pio::Direction::Out, &rgb_pins);

		// Pass the parameters to the PIO programs

		// Pass the height to the RGB_DE program
		self.sm0.tx().push((DISPLAY_HEIGHT - 1) as u32);
		// Pass the width to the RGB program
		self.sm1.tx().push((DISPLAY_WIDTH - 1) as u32);

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

	pub async fn flush_chunk(&mut self, data: &[u16]) -> Result<()> {
		self.sm1
			.tx()
			.dma_push(&mut self.dma_channel_a, &data, false)
			.await;

		Ok(())
	}
}
