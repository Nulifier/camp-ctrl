use cortex_m::asm::nop;
use defmt::{info, unwrap};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel, signal::Signal};
use embassy_time::{Duration, Timer};
use hmi_gui::DISPLAY_WIDTH;
use rp_pac::{self as pac};

use crate::{
	board::{
		DisplayCtrlResources, DisplayDataResources, DisplayFillResources, DisplayTimingResources,
	},
	drivers::display::{buffers::DoubleBuffers, chunks::init_chunk_pool},
};

mod buffers;
mod chunks;
mod ctrl;
mod fill;
mod pio_progs;
mod rgb;
mod test_patterns;
mod timing;

const PCLK_FREQUENCY: u32 = 25_000_000; // 25 MHz

const FRAME_PIXELS: usize = (DISPLAY_WIDTH as usize) * (hmi_gui::DISPLAY_HEIGHT as usize);

static SWAP_REQUESTED: Signal<CriticalSectionRawMutex, bool> = Signal::new();

/// Request that the display driver swaps the active frame buffer at the next frame start.
pub fn request_swap() {
	SWAP_REQUESTED.signal(true);
}

static FREE_CHUNKS: channel::Channel<
	CriticalSectionRawMutex,
	usize,
	{ chunks::CHUNK_BUFFER_COUNT },
> = channel::Channel::new();

// #[embassy_executor::task]
pub fn display_task(
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
	_r_fill: DisplayFillResources,
) -> ! {
	info!("Starting display driver");
	rgb::enable_dma_irq_cycle_counter();
	rgb::clear_dma_irq_stats();

	// Create the frame buffers
	let mut frame_buffers = DoubleBuffers::<{ FRAME_PIXELS }, u16>::new();

	// Fill the frame buffers with a test pattern
	test_patterns::fill_frame_buffers_with_test_pattern(unsafe {
		frame_buffers.active_slice_mut()
	});
	test_patterns::fill_frame_buffers_with_test_pattern(frame_buffers.back_slice_mut());

	rgb::set_frame_buffer_ptr(frame_buffers.active_ptr());

	// Create the chunk buffers
	let chunks = init_chunk_pool();

	// Fill the chunk buffers from the active frame buffer
	for i in 0..chunks::CHUNK_BUFFER_COUNT {
		let pixel_start = i * chunks::CHUNK_PIXELS;
		let pixel_end = pixel_start + chunks::CHUNK_PIXELS;
		chunks
			.get_mut(i)
			.copy_from_slice(&frame_buffers.active_slice()[pixel_start..pixel_end]);
	}

	rgb::set_chunk_ptr(0, chunks.as_mut_ptr(0));
	rgb::set_chunk_ptr(1, chunks.as_mut_ptr(1));
	// rgb::set_chunk_ptr(2, chunks.as_mut_ptr(2));
	// rgb::set_chunk_ptr(3, chunks.as_mut_ptr(3));

	pac::PIO1.fdebug().modify(|w| {
		// Clear the TXSTALL flag if it was set
		w.set_txstall(1);
	});

	let mut ctrl_engine = ctrl::CtrlEngine::new(r_ctrl);
	let mut timing_engine = timing::TimingEngine::new(r_timing);
	let mut rgb_engine = rgb::RgbEngine::new(r_data);

	// ctrl_engine.reset().await;
	unwrap!(timing_engine.init());
	unwrap!(rgb_engine.init([chunks.ptr(0), chunks.ptr(1)]));

	// Start the engines
	unwrap!(rgb_engine.start());
	unwrap!(timing_engine.start());

	info!("Display driver started");

	loop {
		static mut FRAME_COUNT: usize = 0;
		unsafe {
			FRAME_COUNT += 1;

			info!("PIO TXSTALL: {}", pac::PIO1.fdebug().read().txstall());
			info!(
				"FRAME {}, DMA IRQ3 stats: max_cycles={}, overruns={}, count={}",
				unsafe { FRAME_COUNT },
				rgb::dma_irq_max_cycles(),
				rgb::dma_irq_overruns(),
				rgb::dma_irq_count()
			);
		};

		// Do nothing, just burn CPU cycles
		for _ in 0..400000 {
			nop();
		}

		// Timer::after(Duration::from_secs(2)).await;
	}
}
