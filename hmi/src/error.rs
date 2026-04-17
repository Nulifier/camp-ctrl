use embassy_sync::pubsub;
use embedded_hal_async::i2c;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, defmt::Format)]
pub enum Error {
	TouchDriverInitFailed,
	PubSubError(pubsub::Error),
	I2cError(i2c::ErrorKind),
	PsramAllocationFailed,
	PioProgramLoadFailed,
	InvalidDisplayTiming,
}

impl From<pubsub::Error> for Error {
	fn from(e: pubsub::Error) -> Self {
		Error::PubSubError(e)
	}
}

impl From<i2c::ErrorKind> for Error {
	fn from(e: i2c::ErrorKind) -> Self {
		Error::I2cError(e)
	}
}
