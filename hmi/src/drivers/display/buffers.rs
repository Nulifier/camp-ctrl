use crate::utils::psram::PsramBuffer;

pub struct DoubleBuffers<const N: usize, T> {
	fb0: PsramBuffer<T, N>,
	fb1: PsramBuffer<T, N>,
	active: usize,
}

impl<const N: usize, T> DoubleBuffers<N, T> {
	pub fn new() -> Self {
		Self {
			fb0: PsramBuffer::new().unwrap(),
			fb1: PsramBuffer::new().unwrap(),
			active: 0,
		}
	}

	pub fn ptrs_for_lvgl(&mut self) -> (*mut T, *mut T) {
		(self.fb0.as_mut_ptr(), self.fb1.as_mut_ptr())
	}

	pub fn active_ptr(&self) -> *const T {
		match self.active {
			0 => self.fb0.as_ptr(),
			_ => self.fb1.as_ptr(),
		}
	}

	pub fn active_slice(&self) -> &[T] {
		match self.active {
			0 => self.fb0.as_ref(),
			_ => self.fb1.as_ref(),
		}
	}

	pub unsafe fn active_slice_mut(&mut self) -> &mut [T] {
		match self.active {
			0 => self.fb0.as_mut(),
			_ => self.fb1.as_mut(),
		}
	}

	pub fn back_ptr(&self) -> *const T {
		match self.active {
			0 => self.fb1.as_ptr(),
			_ => self.fb0.as_ptr(),
		}
	}

	pub fn back_ptr_mut(&mut self) -> *mut T {
		match self.active {
			0 => self.fb1.as_mut_ptr(),
			_ => self.fb0.as_mut_ptr(),
		}
	}

	pub fn back_slice_mut(&mut self) -> &mut [T] {
		match self.active {
			0 => self.fb1.as_mut(),
			_ => self.fb0.as_mut(),
		}
	}

	pub fn swap(&mut self) {
		self.active ^= 1;
	}
}
