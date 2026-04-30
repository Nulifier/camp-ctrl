use crate::board::DisplayDataResources;
use crate::drivers::display::FREE_CHUNKS;
use crate::error::Result;
use defmt::assert_eq;
use embassy_rp::{Peri, interrupt};
use embassy_rp::{
	gpio, peripherals,
	pio::{self, PioBatch},
};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use rp_pac::dma::regs::CtrlTrig;
use rp_pac::{self as pac};

// This alignment makes sure that we can read the chunk ptrs in a ring buffer as it needs to be
// a round binary value for the wrapping to work correctly.
// It should be set to 4 * CHUNK_BUFFER_COUNT
#[repr(align(8))]
struct AlignedChunkPtrRing([*const u16; super::chunks::CHUNK_BUFFER_COUNT]);

const CHUNK_RING_ALIGNMENT: usize =
	core::mem::size_of::<*const u16>() * super::chunks::CHUNK_BUFFER_COUNT;

/// How many bits are needed to represent the chunk ring size, used for configuring the DMA ring buffer
const CHUNK_RING_BITS: u8 = CHUNK_RING_ALIGNMENT.ilog2() as u8;

static mut CHUNK_PTR_RING: AlignedChunkPtrRing =
	AlignedChunkPtrRing([core::ptr::null(); super::chunks::CHUNK_BUFFER_COUNT]);

const DMA_PIXELS: usize = 14;
const DMA_CTRL: usize = 15;

const DMA_PIXELS_MASK: u32 = 1 << DMA_PIXELS;

const DMA_IRQ_NUM: usize = 3;

pub(super) struct RgbEngine {
	common: pio::Common<'static, peripherals::PIO1>,

	sm0: pio::StateMachine<'static, peripherals::PIO1, 0>,
	sm1: pio::StateMachine<'static, peripherals::PIO1, 1>,

	de_pin: pio::Pin<'static, peripherals::PIO1>,

	b3_pin: pio::Pin<'static, peripherals::PIO1>,
	b4_pin: pio::Pin<'static, peripherals::PIO1>,
	b5_pin: pio::Pin<'static, peripherals::PIO1>,
	b6_pin: pio::Pin<'static, peripherals::PIO1>,
	b7_pin: pio::Pin<'static, peripherals::PIO1>,

	g2_pin: pio::Pin<'static, peripherals::PIO1>,
	g3_pin: pio::Pin<'static, peripherals::PIO1>,
	g4_pin: pio::Pin<'static, peripherals::PIO1>,
	g5_pin: pio::Pin<'static, peripherals::PIO1>,
	g6_pin: pio::Pin<'static, peripherals::PIO1>,
	g7_pin: pio::Pin<'static, peripherals::PIO1>,

	r3_pin: pio::Pin<'static, peripherals::PIO1>,
	r4_pin: pio::Pin<'static, peripherals::PIO1>,
	r5_pin: pio::Pin<'static, peripherals::PIO1>,
	r6_pin: pio::Pin<'static, peripherals::PIO1>,
	r7_pin: pio::Pin<'static, peripherals::PIO1>,

	// We just hold references to the DMA channels here as we're doing this using the PAC
	#[allow(dead_code)]
	dma_channel_pixels: Peri<'static, peripherals::DMA_CH14>,
	#[allow(dead_code)]
	dma_channel_ctrl: Peri<'static, peripherals::DMA_CH15>,
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
			dma_channel_pixels: r.dma_pixels,
			dma_channel_ctrl: r.dma_ctrl,
		}
	}

	pub fn init(&mut self, bufs: [*const u16; super::chunks::CHUNK_BUFFER_COUNT]) -> Result<()> {
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

		self.init_dma_ring(bufs);

		Ok(())
	}

	pub fn start(&mut self) -> Result<()> {
		// Start the DMA ring so pixel data is available when the PIO starts
		self.start_dma_ring();

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

	fn init_dma_ring(&mut self, bufs: [*const u16; super::chunks::CHUNK_BUFFER_COUNT]) {
		let pio = pac::PIO1;

		assert_eq!(
			CHUNK_RING_ALIGNMENT,
			core::mem::align_of::<AlignedChunkPtrRing>(),
			"Chunk pointer ring alignment does not match expected value"
		);

		// Update the global chunk pointer ring that the DMA channels will read from
		// We start at 1 as the first chunk is already being processed by the DMA when this function
		// is called, so the next chunk to process will be at index 1
		// unsafe { CHUNK_PTR_RING.0 = [bufs[1], bufs[2], bufs[3], bufs[0]] };
		unsafe { CHUNK_PTR_RING.0 = [bufs[1], bufs[0]] };

		// Get PIO1 SM1 TX FIFO address
		let tx_fifo_addr = pio.txf(1).as_ptr() as u32;
		let dma_al3_read_addr_ptr = pac::DMA.ch(DMA_PIXELS).al3_read_addr_trig().as_ptr() as u32;
		// This is safe because the DMA engine will only read from this address, and we won't modify it after this function
		let chunk_ptr_ring_ptr = unsafe { &raw mut CHUNK_PTR_RING.0 as *const *const u16 };

		// DREQ:
		// PIO0 TX0..TX3 = DREQ 0..3
		// PIO1 TX0..TX3 = DREQ 8..11
		// PIO2 TX0..TX3 = DREQ 16..19
		const DREQ: pac::dma::vals::TreqSel = pac::dma::vals::TreqSel::PIO1_TX1; // PIO1 TX1

		unsafe {
			Self::configure_pixel_dma(
				DMA_PIXELS,
				bufs[0],
				tx_fifo_addr,
				super::chunks::CHUNK_PIXELS as u32,
				DREQ,
				DMA_CTRL,
			);

			Self::configure_ctrl_dma(DMA_CTRL, chunk_ptr_ring_ptr, dma_al3_read_addr_ptr);
		}

		// Clear stale interrupt flags
		pac::DMA.ints(DMA_IRQ_NUM).write_value(DMA_PIXELS_MASK);

		// Only pixel DMA needs an IRQ to signal that a chunk has been completed and its buffer is free
		pac::DMA.inte(DMA_IRQ_NUM).write_value(DMA_PIXELS_MASK);
	}

	fn start_dma_ring(&mut self) {
		// Start first pixel transfer
		pac::DMA.ch(DMA_PIXELS).ctrl_trig().modify(|w| {
			w.set_en(true);
		});
	}

	unsafe fn configure_pixel_dma(
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

		let mut ctrl = CtrlTrig::default();

		// Enable the channel, but this won't trigger it as we're using a non-triggering alias
		// TODO: Can probably set this to false and just trigger the channel manually
		ctrl.set_en(true);
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

		// Use non-triggering alias
		ch.al1_ctrl().write_value(ctrl.0);
	}

	unsafe fn configure_ctrl_dma(ch_num: usize, read_addr: *const *const u16, write_addr: u32) {
		let ch = pac::DMA.ch(ch_num);

		// Disable the channel before configuring
		ch.ctrl_trig().modify(|w| {
			w.set_en(false);
		});

		ch.read_addr().write_value(read_addr as u32);
		ch.write_addr().write_value(write_addr);
		ch.trans_count().write(|w| {
			w.set_count(1);
		});

		let mut ctrl = CtrlTrig::default();

		// Enable the channel, but this won't trigger it as we're using a non-triggering alias
		ctrl.set_en(true);
		ctrl.set_high_priority(true);
		// Copy one 32-bit address per transfer
		ctrl.set_data_size(pac::dma::vals::DataSize::SIZE_WORD);
		// Walk through the 4-element pointer ring buffer
		ctrl.set_incr_read(true);
		// Always write to the same control register
		ctrl.set_incr_write(false);
		ctrl.set_ring_size(CHUNK_RING_BITS); // 2^4 = 16, so the read address will wrap around after 4 transfers
		ctrl.set_ring_sel(false); // Apply the ring buffer to the read address
		// Do not chain, writing this channel's value disables chaining
		ctrl.set_chain_to(ch_num as u8);
		// No pacing, this should run immediately after the pixel DMA channel triggers it
		ctrl.set_treq_sel(pac::dma::vals::TreqSel::PERMANENT);
		// No IRQ needed from control channel
		ctrl.set_irq_quiet(true);

		ch.al1_ctrl().write_value(ctrl.0);
	}
}

/// Tracks what which is the next chunk to be completed by the DMA
static DMA_COMPLETED_CHUNK: core::sync::atomic::AtomicUsize =
	core::sync::atomic::AtomicUsize::new(0);

#[interrupt]
fn DMA_IRQ_3() {
	const MAX_CHUNK_INDEX: usize = super::chunks::CHUNK_BUFFER_COUNT - 1;

	let pending = pac::DMA.ints(DMA_IRQ_NUM).read();

	if pending & DMA_PIXELS_MASK != 0 {
		// Clear the interrupt flag
		pac::DMA.ints(DMA_IRQ_NUM).write_value(DMA_PIXELS_MASK);

		// This is the index of the chunk we're going to copy the framebuffer data into
		let chunk_idx = DMA_COMPLETED_CHUNK.load(core::sync::atomic::Ordering::Relaxed);

		// Signal free chunk
		let _ = FREE_CHUNKS.try_send(chunk_idx).is_ok();

		DMA_COMPLETED_CHUNK.store(
			(chunk_idx + 1) & MAX_CHUNK_INDEX, // Wrap around the chunk index
			core::sync::atomic::Ordering::Relaxed,
		);
	}
}
