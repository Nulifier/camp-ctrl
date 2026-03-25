pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
	InvalidDisplaySize,
	DisplayCreateFailed,
	InputDeviceCreateFailed,
	IndexOutOfBounds,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::InvalidDisplaySize => write!(f, "invalid display size"),
			Self::DisplayCreateFailed => write!(f, "failed to create display"),
			Self::InputDeviceCreateFailed => write!(f, "failed to create input device"),
			Self::IndexOutOfBounds => write!(f, "index out of bounds"),
		}
	}
}

impl core::error::Error for Error {}
