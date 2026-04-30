use defmt::{info, unwrap};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel};
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

const FRAME_PIXELS: usize = (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize);

type FrameBuffers = DoubleBuffers<{ FRAME_PIXELS }, u16>;

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
	r_fill: DisplayFillResources,
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
	let mut fill_engine = fill::FillEngine::new(r_fill, frame_buffers, chunks);

	ctrl_engine.reset().await;
	unwrap!(timing_engine.init());
	unwrap!(rgb_engine.init([chunks.ptr(0), chunks.ptr(1)]));
	fill_engine.init();

	// Start the engines
	unwrap!(rgb_engine.start());
	unwrap!(timing_engine.start());

	info!("Display driver started");

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
		fill_engine.on_vblank();

		// On startup or while the final chunk is being pushed to the PIO
		// the first chunk can be filled with the first chunk of the active frame buffer

		// In the VBLANK interval:
		// - Swap the frame buffers if requested
		// - RGB PIO can be stopped and is just waiting for the next VSYNC signal
		// - Fill chunk buffer 0 with the first chunk of pixel data from the active frame buffer
		// - Stop RGB DMA
		// - Clear PIO FIFOs
		// - Restart the RGB DMA chain pointing to chunk buffer 0

		while let Some(frame_buffer_index) = fill_engine.get_next_frame_buffer_index() {
			fill_engine.fill_next_chunk(frame_buffer_index);
		}

		// Check if a swap was requested during the VBLANK and swap if so
		fill_engine.swap_if_requested();

		// Fill the first chunk of the next frame while the last chunk of the current frame is being pushed to the PIO
		{
			let frame_buffer_index = fill_engine.get_next_frame_buffer_index().unwrap();
			assert!(
				frame_buffer_index == 0,
				"Expected to fill the first chunk of the frame buffer after VBLANK"
			);
			fill_engine.fill_next_chunk(frame_buffer_index);
		}
	}
}
