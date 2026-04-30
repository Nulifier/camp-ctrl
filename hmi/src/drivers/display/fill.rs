use embassy_rp::{Peri, peripherals};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

use crate::{
	board::DisplayFillResources,
	drivers::display::{
		FRAME_PIXELS, FrameBuffers,
		chunks::{CHUNK_BUFFER_COUNT, CHUNK_PIXELS, ChunkPool},
	},
};

/// How many chunks are needed to fill an entire frame buffer
const FRAME_BUFFER_CHUNKS: usize = FRAME_PIXELS / CHUNK_PIXELS;

static SWAP_REQUESTED: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub(super) struct FillEngine {
	#[allow(dead_code)]
	dma_channel: Peri<'static, peripherals::DMA_CH13>,
	frame_buffers: FrameBuffers,
	chunks: &'static ChunkPool,

	next_frame_buffer_index: usize,
	next_chunk_index: usize,
}

impl FillEngine {
	pub fn new(
		r: DisplayFillResources,
		frame_buffers: FrameBuffers,
		chunks: &'static ChunkPool,
	) -> Self {
		assert_eq!(
			FRAME_PIXELS % CHUNK_PIXELS,
			0,
			"Frame buffer size must be a multiple of chunk size"
		);

		FillEngine {
			dma_channel: r.dma_fill,
			frame_buffers,
			chunks,
			next_frame_buffer_index: 0,
			next_chunk_index: 0,
		}
	}

	/// Initialize the fill engine. This must be called before any other methods.
	/// This copies the initial frame buffer data into the first chunk
	pub fn init(&mut self) {
		let frame_buffer_index = self.get_next_frame_buffer_index().unwrap();
		self.fill_next_chunk(frame_buffer_index);
	}

	pub fn on_vblank(&mut self) {}

	/// Request that the fill engine swap the active frame buffer at the next frame start.
	#[allow(dead_code)]
	pub fn request_swap(&mut self) {
		SWAP_REQUESTED.signal(());
	}

	/// Check if a swap was requested and swap if so.
	pub fn swap_if_requested(&mut self) {
		if let Some(_) = SWAP_REQUESTED.try_take() {
			self.frame_buffers.swap();
		}
	}

	fn get_next_chunk_index(&mut self) -> usize {
		let index = self.next_chunk_index;
		// TODO: Optimize once we settle on two chunk buffers
		self.next_chunk_index = (self.next_chunk_index + 1) % CHUNK_BUFFER_COUNT;
		index
	}

	/// Get the next frame buffer index to copy to a chunk.
	/// This will return None if we've already filled all the chunks for the current frame buffer.
	pub fn get_next_frame_buffer_index(&mut self) -> Option<usize> {
		let index = self.next_frame_buffer_index;
		if index >= FRAME_BUFFER_CHUNKS {
			self.next_frame_buffer_index = 0;
			None
		} else {
			self.next_frame_buffer_index = self.next_frame_buffer_index + 1;
			Some(index)
		}
	}

	pub fn fill_next_chunk(&mut self, frame_buffer_index: usize) {
		assert!(
			frame_buffer_index < FRAME_BUFFER_CHUNKS,
			"Frame buffer index out of bounds"
		);

		let chunk_index = self.get_next_chunk_index();

		// If we're starting a new frame, we might want to trigger a buffer swap here
		// This chunk will typically be filled while we're pushing the last chunk to the PIO
		if frame_buffer_index == 0 {
			// Swap if requested
		}

		// Update the frame buffer index
		self.fill_chunk(frame_buffer_index, chunk_index);
	}

	fn fill_chunk(&mut self, frame_buffer_index: usize, chunk_index: usize) {
		let frame_buffer = self.frame_buffers.active_slice();
		let chunk_buffer = self.chunks.get_mut(chunk_index);

		// Determine the portion of the frame buffer that corresponds to this chunk
		let pixel_start = frame_buffer_index * CHUNK_PIXELS;
		let pixel_end = pixel_start + CHUNK_PIXELS;

		let frame_buffer_chunk = &frame_buffer[pixel_start..pixel_end];

		// Copy the relevant portion of the frame buffer into the chunk buffer
		chunk_buffer.copy_from_slice(frame_buffer_chunk);
	}
}
