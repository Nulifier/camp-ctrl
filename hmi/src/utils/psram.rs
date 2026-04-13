use crate::board::PsramResources;
use crate::error::{Error, Result};
use const_default::ConstDefault;
use core::alloc::Layout;
use core::cell::RefCell;
use core::ptr::NonNull;
use critical_section::Mutex;
use defmt::{assert, panic, unwrap};
use embassy_rp::psram;
use embassy_rp::qmi_cs1::QmiCs1;
use rlsf::Tlsf;

// Implementation of PsramHeap taken from embedded-alloc crate to avoid nightly requirement

type TlsHeap = Tlsf<'static, usize, usize, { usize::BITS as usize }, { usize::BITS as usize }>;

struct PsramHeapInner {
	tlsf: TlsHeap,
	initialized: bool,
	raw_block: Option<NonNull<[u8]>>,
	raw_block_size: usize,
}

// Safety: The whole inner type is wrapped by a Mutex
unsafe impl Sync for PsramHeapInner {}
unsafe impl Send for PsramHeapInner {}

struct PsramHeap {
	heap: Mutex<RefCell<PsramHeapInner>>,
}

impl PsramHeap {
	pub const fn empty() -> Self {
		Self {
			heap: Mutex::new(RefCell::new(PsramHeapInner {
				tlsf: ConstDefault::DEFAULT,
				initialized: false,
				raw_block: None,
				raw_block_size: 0,
			})),
		}
	}

	pub unsafe fn init(&self, start_addr: usize, size: usize) {
		assert!(size > 0, "Heap size must be greater than 0");
		critical_section::with(|cs| {
			let mut heap = self.heap.borrow_ref_mut(cs);
			assert!(!heap.initialized, "Heap can only be initialized once");

			// Create a block from the provided address and size, and insert it into the TLSF heap
			let block = NonNull::slice_from_raw_parts(
				unsafe { NonNull::new_unchecked(start_addr as *mut u8) },
				size,
			);
			let Some(actual_size) = (unsafe { heap.tlsf.insert_free_block_ptr(block) }) else {
				panic!("Failed to initialize heap: block is too small");
			};

			// Create the raw block with the actual size used by TLSF (which may be smaller than the requested size due to alignment and metadata overhead)
			let block = NonNull::slice_from_raw_parts(
				unsafe { NonNull::new_unchecked(start_addr as *mut u8) },
				actual_size.get(),
			);

			heap.initialized = true;
			heap.raw_block = Some(block);
			heap.raw_block_size = size;
		});
	}

	pub fn alloc(&self, layout: Layout) -> Option<NonNull<u8>> {
		critical_section::with(|cs| self.heap.borrow_ref_mut(cs).tlsf.allocate(layout))
	}

	pub unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		critical_section::with(|cs| unsafe {
			self.heap
				.borrow_ref_mut(cs)
				.tlsf
				.deallocate(NonNull::new_unchecked(ptr), layout.align())
		})
	}

	pub unsafe fn realloc(&self, ptr: *mut u8, new_layout: Layout) -> Option<NonNull<u8>> {
		critical_section::with(|cs| unsafe {
			self.heap
				.borrow_ref_mut(cs)
				.tlsf
				.reallocate(NonNull::new_unchecked(ptr), new_layout)
		})
	}

	/// Number of bytes used by the allocator
	pub fn used(&self) -> usize {
		critical_section::with(|cs| {
			let free = self.free_with_cs(cs);
			self.heap.borrow_ref_mut(cs).raw_block_size - free
		})
	}

	/// Number of free bytes in the allocator
	pub fn free(&self) -> usize {
		critical_section::with(|cs| self.free_with_cs(cs))
	}

	fn free_with_cs(&self, cs: critical_section::CriticalSection) -> usize {
		let inner_mut = self.heap.borrow_ref_mut(cs);
		if !inner_mut.initialized {
			return 0;
		}

		// Safety: We pass the memory block we previously initialized the with with
		// to the `iter_blocks` method.
		unsafe {
			inner_mut
				.tlsf
				.iter_blocks(inner_mut.raw_block.unwrap())
				.filter(|block_info| !block_info.is_occupied())
				.map(|block_info| block_info.max_payload_size())
				.sum()
		}
	}
}

static PSRAM_HEAP: PsramHeap = PsramHeap::empty();

fn psram_config() -> psram::Config {
	// Config based on the W25Q128JVSIQ PSRAM chip used on the board
	psram::Config::custom(
		125_000_000,             // Default to 125MHz
		109_000_000,             // Max frequency for 3.3V operation
		8,                       // 8µs max CS assert
		50,                      // 50ns min CS deassert
		1,                       // Assume 1 SCLK cycle cooldown
		psram::PageBreak::_1024, // 1024-byte page size
		2,                       // Clock divider is 125MHz/ 109Mhz = 2 (round up)
		Some(0x35),              // Enter quad mode command
		0xEB,                    // Fast quad read command
		Some(0x38),              // Quad page program (write) command
		24,                      // 24 dummy cycles for quad read
		psram::FormatConfig {
			// Read format
			prefix_width: psram::Width::Quad,
			addr_width: psram::Width::Quad,
			suffix_width: psram::Width::Quad,
			dummy_width: psram::Width::Quad,
			data_width: psram::Width::Quad,
			prefix_len: true,  // 8-bit prefix
			suffix_len: false, // No suffix
		},
		Some(psram::FormatConfig {
			// Write format
			prefix_width: psram::Width::Quad,
			addr_width: psram::Width::Quad,
			suffix_width: psram::Width::Quad,
			dummy_width: psram::Width::Quad,
			data_width: psram::Width::Quad,
			prefix_len: true,  // 8-bit prefix
			suffix_len: false, // No suffix
		}),
		2 * 1024 * 1024,               // 2MB total size
		psram::VerificationType::None, // Skip device verification
		true,                          // PSRAM is writable
	)
}

pub fn init_psram_heap(r: PsramResources) {
	let config = psram_config();
	let qmi_cs1 = QmiCs1::new(r.qmi_cs1, r.cs);
	let psram: psram::Psram<'_> = unwrap!(psram::Psram::new(qmi_cs1, config));
	unsafe { PSRAM_HEAP.init(psram.base_address() as usize, psram.size()) };
}

pub struct PsramBuffer<T: Sized, const N: usize> {
	ptr: NonNull<[T; N]>,
}

impl<T: Sized, const N: usize> PsramBuffer<T, N> {
	pub fn new() -> Result<Self> {
		let layout =
			core::alloc::Layout::array::<T>(N).map_err(|_| Error::PsramAllocationFailed)?;
		let raw_ptr = PSRAM_HEAP
			.alloc(layout)
			.ok_or_else(|| Error::PsramAllocationFailed)?;
		Ok(Self {
			ptr: raw_ptr.cast::<[T; N]>(),
		})
	}
}

impl<T: Sized, const N: usize> Drop for PsramBuffer<T, N> {
	fn drop(&mut self) {
		let layout = core::alloc::Layout::array::<T>(N).unwrap();
		unsafe {
			PSRAM_HEAP.dealloc(self.ptr.as_ptr() as *mut u8, layout);
		}
	}
}

impl<T: Sized, const N: usize> core::ops::Deref for PsramBuffer<T, N> {
	type Target = [T; N];

	fn deref(&self) -> &Self::Target {
		unsafe { self.ptr.as_ref() }
	}
}
