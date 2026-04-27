use defmt::{info, unwrap};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel, signal::Signal};
use hmi_gui::DISPLAY_WIDTH;

use crate::{
	board::{
		DisplayCtrlResources, DisplayDataResources, DisplayFillResources, DisplayTimingResources,
	},
	drivers::display::chunks::init_chunk_pool,
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
	{ chunks::SRAM_BUFFER_COUNT },
> = channel::Channel::new();

#[embassy_executor::task]
pub async fn display_task(
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
	_r_fill: DisplayFillResources,
) {
	info!("Starting display driver");

	// Create the chunk buffers
	let chunks = init_chunk_pool();

	// Fill the chunk buffers with a test pattern
	for i in 0..chunks::SRAM_BUFFER_COUNT {
		let chunk = chunks.get_mut(i);
		test_patterns::fill_chunk_with_test_pattern(chunk);
	}

	let mut ctrl_engine = ctrl::CtrlEngine::new(r_ctrl);
	let mut timing_engine = timing::TimingEngine::new(r_timing);
	let mut rgb_engine = rgb::RgbEngine::new(r_data);

	ctrl_engine.reset().await;
	unwrap!(timing_engine.init());
	unwrap!(rgb_engine.init([chunks.ptr(0), chunks.ptr(1), chunks.ptr(2), chunks.ptr(3)]));

	// Start the engines
	unwrap!(rgb_engine.start());
	unwrap!(timing_engine.start());

	info!("Display driver started");

	// We're going to cheat and just send the test pattern chunks
	loop {
		// Check if we need to swap the frame buffer at the top of each frame
		if Some(true) == SWAP_REQUESTED.try_take() {
			// TODO: Do a frame swap
		}

		// info!("Top of frame");

		// Loop through enough chunks to fill a frame
		const CHUNKS_PER_FRAME: usize = FRAME_PIXELS / chunks::CHUNK_PIXELS;
		for _ in 0..CHUNKS_PER_FRAME {
			// Get a free chunk
			let _chunk_index = FREE_CHUNKS.receive().await;

			// info!("Filled chunk, index: {}", _chunk_index);

			// Copy data from current frame buffer to chunk buffer
			// TODO: Implement this, currently we just use the pre-filled test pattern chunks

			// We don't need to send the chunk to the flush task as it just loops forever
		}
	}
}
