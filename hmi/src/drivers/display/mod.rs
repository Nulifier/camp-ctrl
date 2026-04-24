use defmt::{info, unwrap};
use embassy_executor::SendSpawner;
use embassy_sync::{
	blocking_mutex::raw::CriticalSectionRawMutex,
	signal::Signal,
	zerocopy_channel::{Channel, Receiver, Sender},
};
use hmi_gui::DISPLAY_WIDTH;
use static_cell::{ConstStaticCell, StaticCell};

use crate::board::{
	DisplayCtrlResources, DisplayDataResources, DisplayFillResources, DisplayTimingResources,
};

mod buffers;
mod ctrl;
mod fill;
mod pio_progs;
mod rgb;
mod test_patterns;
mod timing;

const PCLK_FREQUENCY: u32 = 25_000_000; // 25 MHz

/// Number of lines of pixel data to buffer in SRAM (as opposed to PSRAM)
const SRAM_BUFFER_LINES: usize = 8;

/// Number of chunk buffers to use. Must be at least 2 to allow double buffering
const SRAM_BUFFER_COUNT: usize = 4;

type ChunkBuffer = [u16; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES];

static SWAP_REQUESTED: Signal<CriticalSectionRawMutex, bool> = Signal::new();

/// Request that the display driver swaps the active frame buffer at the next frame start.
pub fn request_swap() {
	SWAP_REQUESTED.signal(true);
}

pub fn spawn_display_tasks<'a>(
	spawner: &SendSpawner,
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
	_r_fill: DisplayFillResources,
) {
	// Create the chunk buffers
	static CHUNK_BUFS: ConstStaticCell<[ChunkBuffer; SRAM_BUFFER_COUNT]> =
		ConstStaticCell::new([[0; DISPLAY_WIDTH as usize * SRAM_BUFFER_LINES]; SRAM_BUFFER_COUNT]);

	// Fill the chunk buffers with a test pattern
	let chunks = CHUNK_BUFS.take();
	for chunk in chunks.iter_mut() {
		test_patterns::fill_chunk_with_test_pattern(chunk);
	}

	// Create a channel through which we'll send filled chunks to the flush task
	static READY_CHUNK_CHANNEL: StaticCell<Channel<'_, CriticalSectionRawMutex, ChunkBuffer>> =
		StaticCell::new();
	let channel = READY_CHUNK_CHANNEL.init(Channel::new(chunks));

	let (sender, receiver) = channel.split();

	info!("Spawning display tasks");
	// Spawn the fill and flush tasks
	spawner.spawn(unwrap!(display_fill_task(sender)));
	spawner.spawn(unwrap!(display_flush_task(
		receiver, r_ctrl, r_timing, r_data
	)));
}

#[embassy_executor::task]
pub async fn display_fill_task(
	mut chunk_sender: Sender<'static, CriticalSectionRawMutex, ChunkBuffer>,
) {
	info!("Starting display fill task");

	// We're going to cheat and just send the test pattern chunks
	loop {
		// Check if we need to swap the frame buffer at the top of each frame
		if Some(true) == SWAP_REQUESTED.try_take() {
			// TODO: Do a frame swap
		}

		let mut _chunk = chunk_sender.send().await;

		// Do nothing

		chunk_sender.send_done();
	}
}

#[embassy_executor::task]
pub async fn display_flush_task(
	mut chunk_receiver: Receiver<'static, CriticalSectionRawMutex, ChunkBuffer>,
	r_ctrl: DisplayCtrlResources,
	r_timing: DisplayTimingResources,
	r_data: DisplayDataResources,
) {
	// let mut display_driver = DisplayDriver::new(r_ctrl, r_timing, r_data);
	let mut ctrl_engine = ctrl::CtrlEngine::new(r_ctrl);
	let mut timing_engine = timing::TimingEngine::new(r_timing);
	let mut rgb_engine = rgb::RgbEngine::new(r_data);

	info!("Starting display flush task");

	ctrl_engine.reset().await;
	unwrap!(timing_engine.init());
	unwrap!(rgb_engine.init());

	// Wait for the first chunk before we start the display timing.
	unwrap!(rgb_engine.start());
	unwrap!(timing_engine.start());

	loop {
		let chunk = chunk_receiver.receive().await;

		// Flush the chunk to the display
		unwrap!(rgb_engine.flush_chunk(chunk).await);

		// Mark the chunk as free
		chunk_receiver.receive_done();
	}
}
