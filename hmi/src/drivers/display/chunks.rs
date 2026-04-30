use core::cell::UnsafeCell;
use hmi_gui::DISPLAY_WIDTH;
use static_cell::{ConstStaticCell, StaticCell};

/// Number of lines of pixel data to buffer in SRAM (as opposed to PSRAM)
pub(super) const CHUNK_BUFFER_LINES: usize = 80;

/// Number of chunk buffers to use. Must be at least 2 to allow double buffering
/// WARNING: If this is changed, then make sure to update the DMA ring alignment in rgb.rs
pub(super) const CHUNK_BUFFER_COUNT: usize = 2;

pub(super) const CHUNK_PIXELS: usize = DISPLAY_WIDTH as usize * CHUNK_BUFFER_LINES;

pub(super) type ChunkBuffer = [u16; CHUNK_PIXELS];

static CHUNK_BUFFERS: ConstStaticCell<UnsafeCell<[ChunkBuffer; CHUNK_BUFFER_COUNT]>> =
	ConstStaticCell::new(UnsafeCell::new([[0; CHUNK_PIXELS]; CHUNK_BUFFER_COUNT]));
static CHUNK_POOL: StaticCell<ChunkPool> = StaticCell::new();

pub(super) struct ChunkPool {
	chunks: &'static UnsafeCell<[ChunkBuffer; CHUNK_BUFFER_COUNT]>,
}

unsafe impl Sync for ChunkPool {}

impl ChunkPool {
	pub fn ptr(&self, index: usize) -> *const u16 {
		unsafe { (*self.chunks.get())[index].as_ptr() }
	}

	pub fn get_mut(&self, index: usize) -> &'static mut ChunkBuffer {
		unsafe { &mut (*self.chunks.get())[index] }
	}
}

pub(super) fn init_chunk_pool() -> &'static ChunkPool {
	let mem = CHUNK_BUFFERS.take();
	CHUNK_POOL.init(ChunkPool { chunks: mem })
}
