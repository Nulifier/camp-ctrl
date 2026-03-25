extern crate alloc;

use alloc::vec::Vec;
use core::ffi::{CStr, FromBytesWithNulError};
use core::fmt::{self, Write};

struct VecWriter<'a> {
	buf: &'a mut Vec<u8>,
}

impl Write for VecWriter<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.buf.extend_from_slice(s.as_bytes());
		Ok(())
	}
}

#[derive(Debug)]
pub enum FormatCStrError {
	Fmt,
	InteriorNul,
	MissingTerminator,
}

pub fn with_format_cstr<R>(
	args: fmt::Arguments<'_>,
	f: impl FnOnce(&CStr) -> R,
) -> Result<R, FormatCStrError> {
	let mut buf = Vec::with_capacity(64);
	let mut w = VecWriter { buf: &mut buf };

	w.write_fmt(args).map_err(|_| FormatCStrError::Fmt)?;

	buf.push(0); // Null terminator

	let cstr = CStr::from_bytes_with_nul(&buf).map_err(|err| match err {
		FromBytesWithNulError::InteriorNul { .. } => FormatCStrError::InteriorNul,
		_ => FormatCStrError::MissingTerminator,
	})?;

	Ok(f(cstr))
}

#[macro_export]
macro_rules! with_format_cstr {
	($f:expr, $($arg:tt)*) => {
		$crate::text::with_format_cstr(format_args!($($arg)*), $f)
	};
}
