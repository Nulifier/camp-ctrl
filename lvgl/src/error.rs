pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
	InvalidDisplaySize,
	DisplayCreateFailed,
	InputDeviceCreateFailed,
	IndexOutOfBounds,
	InvalidGridDescriptors,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::InvalidDisplaySize => write!(f, "invalid display size"),
			Self::DisplayCreateFailed => write!(f, "failed to create display"),
			Self::InputDeviceCreateFailed => write!(f, "failed to create input device"),
			Self::IndexOutOfBounds => write!(f, "index out of bounds"),
			Self::InvalidGridDescriptors => write!(f, "invalid grid descriptors"),
		}
	}
}

impl core::error::Error for Error {}
