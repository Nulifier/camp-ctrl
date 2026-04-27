use crate::board::DisplayDataResources;
use crate::drivers::display::FREE_CHUNKS;
use crate::error::Result;
use defmt::info;
use embassy_rp::{Peri, interrupt};
use embassy_rp::{
	gpio, peripherals,
	pio::{self, PioBatch},
};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use rp_pac as pac;
use rp_pac::dma::regs::CtrlTrig;

const DMA_CH_A: usize = 12;
const DMA_CH_B: usize = 13;
const DMA_CH_C: usize = 14;
const DMA_CH_D: usize = 15;

const DMA_CH_A_MASK: u32 = 1 << DMA_CH_A;
const DMA_CH_B_MASK: u32 = 1 << DMA_CH_B;
const DMA_CH_C_MASK: u32 = 1 << DMA_CH_C;
const DMA_CH_D_MASK: u32 = 1 << DMA_CH_D;

const DMA_CH_ALL_MASK: u32 = (1 << DMA_CH_A) | (1 << DMA_CH_B) | (1 << DMA_CH_C) | (1 << DMA_CH_D);

static mut CHUNK_A_PTR: *const u16 = core::ptr::null();
static mut CHUNK_B_PTR: *const u16 = core::ptr::null();
static mut CHUNK_C_PTR: *const u16 = core::ptr::null();
static mut CHUNK_D_PTR: *const u16 = core::ptr::null();

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

	// We just hold references to the DMA channels here as we're doing this using the PAC
	#[allow(dead_code)]
	pub dma_channel_a: Peri<'static, peripherals::DMA_CH12>,
	#[allow(dead_code)]
	pub dma_channel_b: Peri<'static, peripherals::DMA_CH13>,
	#[allow(dead_code)]
	pub dma_channel_c: Peri<'static, peripherals::DMA_CH14>,
	#[allow(dead_code)]
	pub dma_channel_d: Peri<'static, peripherals::DMA_CH15>,
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
			dma_channel_a: r.dma_a,
			dma_channel_b: r.dma_b,
			dma_channel_c: r.dma_c,
			dma_channel_d: r.dma_d,
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

		// Enable the DMA IRQ 3 in the NVIC
		unsafe {
			cortex_m::peripheral::NVIC::unmask(pac::Interrupt::DMA_IRQ_3);
		}

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

	pub fn start_dma_ring(&mut self, bufs: [*const u16; super::chunks::SRAM_BUFFER_COUNT]) {
		let pio = pac::PIO1;

		// Get PIO1 SM1 TX FIFO address
		let tx_fifo_addr = pio.txf(1).as_ptr() as u32;

		// DREQ:
		// PIO0 TX0..TX3 = DREQ 0..3
		// PIO1 TX0..TX3 = DREQ 8..11
		// PIO2 TX0..TX3 = DREQ 16..19
		const DREQ: pac::dma::vals::TreqSel = pac::dma::vals::TreqSel::PIO1_TX1; // PIO1 TX1

		// REMOVE AFTER TESTING
		info!(
			"Setting up DMA ring with buffers at: {:#010x}, {:#010x}, {:#010x}, {:#010x}",
			bufs[0] as u32, bufs[1] as u32, bufs[2] as u32, bufs[3] as u32
		);
		unsafe {
			CHUNK_A_PTR = bufs[0];
			CHUNK_B_PTR = bufs[1];
			CHUNK_C_PTR = bufs[2];
			CHUNK_D_PTR = bufs[3];
		};

		info!("Configuring DMA channels for RGB output");
		unsafe {
			Self::configure_dma_channel(
				DMA_CH_A,
				bufs[0],
				tx_fifo_addr,
				super::chunks::CHUNK_PIXELS as u32,
				DREQ,
				DMA_CH_B,
			);
			Self::debug_one("DMA_CH_A", DMA_CH_A);
			Self::configure_dma_channel(
				DMA_CH_B,
				bufs[1],
				tx_fifo_addr,
				super::chunks::CHUNK_PIXELS as u32,
				DREQ,
				DMA_CH_C,
			);
			Self::debug_one("DMA_CH_B", DMA_CH_B);
			Self::configure_dma_channel(
				DMA_CH_C,
				bufs[2],
				tx_fifo_addr,
				super::chunks::CHUNK_PIXELS as u32,
				DREQ,
				DMA_CH_D,
			);
			Self::debug_one("DMA_CH_C", DMA_CH_C);
			Self::configure_dma_channel(
				DMA_CH_D,
				bufs[3],
				tx_fifo_addr,
				super::chunks::CHUNK_PIXELS as u32,
				DREQ,
				DMA_CH_A,
			);
			Self::debug_one("DMA_CH_D", DMA_CH_D);
		}

		// Clear stale interrupt flags
		pac::DMA.ints(3).write_value(DMA_CH_ALL_MASK);

		// Enable interrupts for the DMA channels
		pac::DMA.inte(3).write_value(DMA_CH_ALL_MASK);

		// Start only the first channel, the rest will be started by chaining
		pac::DMA.ch(DMA_CH_A).ctrl_trig().modify(|w| {
			w.set_en(true);
		});
	}

	unsafe fn configure_dma_channel(
		ch_num: usize,
		read_addr: *const u16,
		write_addr: u32,
		transfer_count: u32,
		dreq: pac::dma::vals::TreqSel,
		chain_to: usize,
	) {
		let ch = pac::DMA.ch(ch_num);

		// Disable the channel before configuring
		ch.ctrl_trig().modify(|w| {
			w.set_en(false);
		});

		ch.read_addr().write_value(read_addr as u32);
		ch.write_addr().write_value(write_addr);
		ch.trans_count().write(|w| {
			w.set_count(transfer_count);
		});

		let ctrl = {
			let mut ctrl = CtrlTrig::default();

			// Enable the channel, but this won't trigger it as we're using a non-triggering alias
			ctrl.set_en(true);

			// Schedule over low priority channels
			ctrl.set_high_priority(true);

			// 16 bits per pixel (RGB565)
			ctrl.set_data_size(pac::dma::vals::DataSize::SIZE_HALFWORD);

			// Walk through chunk buffer
			ctrl.set_incr_read(true);

			// Always write to the same PIO FIFO address
			ctrl.set_incr_write(false);

			// Start next DMA channel when this one finishes
			ctrl.set_chain_to(chain_to as u8);

			// Pace from PIO TX FIFO DREQ
			ctrl.set_treq_sel(dreq);

			// Generate interrupt at end of each chunk
			ctrl.set_irq_quiet(false);

			ctrl
		};

		// Use non-triggering alias
		ch.al1_ctrl().write_value(ctrl.0);
	}

	pub fn print_debug() {
		info!("Debug -----------------");
		Self::debug_one("DMA_CH_A", DMA_CH_A);
		Self::debug_one("DMA_CH_B", DMA_CH_B);
		Self::debug_one("DMA_CH_C", DMA_CH_C);
		Self::debug_one("DMA_CH_D", DMA_CH_D);
		Self::debug_pio();
	}

	fn debug_one(name: &'static str, ch_num: usize) {
		let ch = pac::DMA.ch(ch_num);
		let ctrl = ch.ctrl_trig().read();

		info!(
			"{}({}) en={} busy={} count={} next_count={} dreq_count={} write_err={} read_err={}  chain_to={} treq_sel={} read_addr={:#010x} write_addr={:#010x}",
			name,
			ch_num,
			ctrl.en(),
			ctrl.busy(),
			ch.trans_count().read().count(),
			ch.dbg_tcr().read(),
			ch.dbg_ctdreq().read().0,
			ctrl.write_error(),
			ctrl.read_error(),
			ctrl.chain_to(),
			ctrl.treq_sel(),
			ch.read_addr().read(),
			ch.write_addr().read(),
		);
	}

	fn debug_pio() {
		let fstat = pac::PIO1.fstat().read();
		let fdebug = pac::PIO1.fdebug().read();
		let sm1_addr = pac::PIO1.sm(1).addr().read();

		info!(
			"PIO1 fstat={=u32:#010x} fdebug={=u32:#010x} sm1_pc={}",
			fstat.0, fdebug.0, sm1_addr
		);
	}
}

#[interrupt]
fn DMA_IRQ_3() {
	let pending = pac::DMA.ints(3).read();

	// Check Channel 12 (DMA_CH_A)
	if pending & DMA_CH_A_MASK != 0 {
		// Clear the interrupt flag
		pac::DMA.ints(3).write_value(DMA_CH_A_MASK);

		// Signal free chunk
		FREE_CHUNKS.try_send(0).ok();

		// Reset the read address
		pac::DMA
			.ch(DMA_CH_A)
			.read_addr()
			.write_value(unsafe { CHUNK_A_PTR } as u32);
	}

	// Check Channel 13 (DMA_CH_B)
	if pending & DMA_CH_B_MASK != 0 {
		// Clear the interrupt flag
		pac::DMA.ints(3).write_value(DMA_CH_B_MASK);

		// Signal free chunk
		FREE_CHUNKS.try_send(1).ok();

		// Reset the read address
		pac::DMA
			.ch(DMA_CH_B)
			.read_addr()
			.write_value(unsafe { CHUNK_B_PTR } as u32);
	}

	// Check Channel 14 (DMA_CH_C)
	if pending & DMA_CH_C_MASK != 0 {
		// Clear the interrupt flag
		pac::DMA.ints(3).write_value(DMA_CH_C_MASK);

		// Signal free chunk
		FREE_CHUNKS.try_send(2).ok();

		// Reset the read address
		pac::DMA
			.ch(DMA_CH_C)
			.read_addr()
			.write_value(unsafe { CHUNK_C_PTR } as u32);
	}

	// Check Channel 15 (DMA_CH_D)
	if pending & DMA_CH_D_MASK != 0 {
		// Clear the interrupt flag
		pac::DMA.ints(3).write_value(DMA_CH_D_MASK);

		// Signal free chunk
		FREE_CHUNKS.try_send(3).ok();

		// Reset the read address
		pac::DMA
			.ch(DMA_CH_D)
			.read_addr()
			.write_value(unsafe { CHUNK_D_PTR } as u32);
	}
}
