use core::ffi::CStr;
use core::fmt::{self, Write};

pub struct SliceWriter<'a> {
	buffer: &'a mut [u8],
	position: usize,
}

impl<'a> SliceWriter<'a> {
	pub fn new(buffer: &'a mut [u8]) -> Self {
		Self {
			buffer,
			position: 0,
		}
	}

	pub fn as_str(&self) -> &str {
		core::str::from_utf8(&self.buffer[..self.position]).expect("Buffer contains invalid UTF-8")
	}

	pub fn as_cstr(&self) -> &CStr {
		CStr::from_bytes_with_nul(&self.buffer[..=self.position])
			.expect("Buffer does not contain a valid CStr")
	}
}

impl<'a> Write for SliceWriter<'a> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let bytes = s.as_bytes();
		if self.position + bytes.len() >= self.buffer.len() {
			return Err(fmt::Error);
		}
		self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
		self.position += bytes.len();
		self.buffer[self.position] = 0; // Null terminator
		Ok(())
	}
}
