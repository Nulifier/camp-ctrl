use defmt::{info, unwrap};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel, signal::Signal};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

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

type FrameBuffers = DoubleBuffers<{ FRAME_PIXELS }, u16>;

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

#[embassy_executor::task]
pub async fn display_task(
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
	_r_fill: DisplayFillResources,
) -> ! {
	info!("Starting display driver");

	// Create the frame buffers
	let mut frame_buffers = DoubleBuffers::<{ FRAME_PIXELS }, u16>::new();

	// Fill the frame buffers with a test pattern
	test_patterns::fill_frame_buffers_with_test_pattern(unsafe {
		frame_buffers.active_slice_mut()
	});
	test_patterns::fill_frame_buffers_with_test_pattern(frame_buffers.back_slice_mut());

	// Create the chunk buffers
	let chunks = init_chunk_pool();

	let mut ctrl_engine = ctrl::CtrlEngine::new(r_ctrl);
	let mut timing_engine = timing::TimingEngine::new(r_timing);
	let mut rgb_engine = rgb::RgbEngine::new(r_data);

	ctrl_engine.reset().await;
	unwrap!(timing_engine.init());
	unwrap!(rgb_engine.init([chunks.ptr(0), chunks.ptr(1)]));

	// Start the engines
	unwrap!(rgb_engine.start());
	unwrap!(timing_engine.start());

	info!("Display driver started");

	const MAX_TRANSFER_INDEX: usize = DISPLAY_HEIGHT as usize / chunks::CHUNK_BUFFER_LINES;

	// LCD Timing Diagram
	//

	// It takes 480 lines / 80 lines per chunk = 6 chunk transfers to transfer a full frame
	// Frame layout:
	// VBLANK:
	// 		FRONT PORCH
	// - Check for swap request and swap frame buffers if requested
	// - Transfer chunk 0 (lines 0-79) for the next frame
	// 		VSYNC PULSE
	// 		BACK PORCH
	// ACTIVE:
	// - Send chunk 0 to RGB PIO
	// - Transfer chunk 1 (lines 80-159)
	// - Send chunk 1 to RGB PIO
	// - Transfer chunk 0 (lines 160-239)
	// - Send chunk 0 to RGB PIO
	// - Transfer chunk 1 (lines 240-319)
	// - Send chunk 1 to RGB PIO
	// - Transfer chunk 0 (lines 320-399)
	// - Send chunk 0 to RGB PIO
	// - Transfer chunk 1 (lines 400-479)
	// - Send chunk 1 to RGB PIO

	// | PHASE   | Fill Chunk                 | RGB Chunk    |
	// | ------- | -------------------------- | ------------ |
	// | STARTUP | FB(0) -> CH(0)             | Idle         |
	// | VBLANK  | Finish / Catch up          | Reset        |
	// | 0       | FB(1) -> CH(1)             | CH(0) -> PIO |
	// | 1       | FB(2) -> CH(0)             | CH(1) -> PIO |
	// | 2       | FB(3) -> CH(1)             | CH(0) -> PIO |
	// | 3       | FB(4) -> CH(0)             | CH(1) -> PIO |
	// | 4       | FB(5) -> CH(1)             | CH(0) -> PIO |
	// | 5       | Frame Swap, FB(0) -> CH(0) | CH(1) -> PIO |

	loop {
		timing_engine.wait_for_vblank().await;

		// On startup or while the final chunk is being pushed to the PIO
		// the first chunk can be filled with the first chunk of the active frame buffer

		// In the VBLANK interval:
		// - Swap the frame buffers if requested
		// - RGB PIO can be stopped and is just waiting for the next VSYNC signal
		// - Fill chunk buffer 0 with the first chunk of pixel data from the active frame buffer
		// - Stop RGB DMA
		// - Clear PIO FIFOs
		// - Restart the RGB DMA chain pointing to chunk buffer 0

		for transfer_index in 0..MAX_TRANSFER_INDEX {
			let transfer_index = (transfer_index + chunks::CHUNK_BUFFER_COUNT) % MAX_TRANSFER_INDEX;

			let chunk_idx = FREE_CHUNKS.receive().await;

			let pixel_start = transfer_index * chunks::CHUNK_PIXELS;
			let pixel_end = pixel_start + chunks::CHUNK_PIXELS;

			let frame_buffer_slice = &frame_buffers.active_slice()[pixel_start..pixel_end];
			let chunk_slice = chunks.get_mut(chunk_idx);

			chunk_slice.copy_from_slice(frame_buffer_slice);
		}
	}
}
