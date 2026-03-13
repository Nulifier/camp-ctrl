use crate::hex::{self, RawFrame};
use core::fmt;
use heapless::Vec;

// @TODO Validate these lengths, document source
pub const MAX_TEXT_LABEL: usize = 9;
pub const MAX_TEXT_VALUE: usize = 33;
pub const MAX_HEX_FRAME: usize = 33 * 2 + 1; // Max size is a 32 character string (BMV7xx)
pub const MAX_HEX_PAYLOAD: usize = 33;

pub const CHECKSUM_LABEL: &[u8] = b"CHECKSUM";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextRecord<'a> {
	pub label: &'a [u8],
	pub value: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event<'a> {
	TextRecord(TextRecord<'a>),
	TextBlockEnd,
	HexFrame(RawFrame<'a>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
	TextLabelTooLong,
	TextValueTooLong,
	TextChecksumInvalid,
	HexFrameTooLong,
	InvalidHex(hex::HexError),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::TextLabelTooLong => write!(f, "text label exceeds max {MAX_TEXT_LABEL} bytes"),
			Self::TextValueTooLong => write!(f, "text value exceeds max {MAX_TEXT_VALUE} bytes"),
			Self::TextChecksumInvalid => write!(f, "text checksum is invalid"),
			Self::HexFrameTooLong => write!(f, "HEX frame exceeds max {MAX_HEX_FRAME} bytes"),
			Self::InvalidHex(error) => write!(f, "invalid HEX frame: {error}"),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
	Idle,
	TextLabel,
	TextValue,
	TextChecksum,
	HexFrame,
}

pub struct Parser {
	state: State,
	text_label: Vec<u8, MAX_TEXT_LABEL>,
	text_value: Vec<u8, MAX_TEXT_VALUE>,
	text_checksum: u8,
	hex_frame: Vec<u8, MAX_HEX_FRAME>,
	hex_payload: Vec<u8, MAX_HEX_PAYLOAD>,
	/// Buffer to store the raw bytes of the current hex frame being parsed. This is needed because
	/// we need to accumulate the bytes of the hex frame until we encounter the end byte, at which
	/// point we can decode the frame and emit it as an event.Y	],
	in_text_block: bool,
}

impl Parser {
	pub const fn new() -> Self {
		Self {
			state: State::Idle,
			text_label: Vec::new(),
			text_value: Vec::new(),
			text_checksum: 0,
			hex_frame: Vec::new(),
			hex_payload: Vec::new(),
			in_text_block: false,
		}
	}

	pub fn push_byte(&mut self, byte: u8) -> Result<Option<Event<'_>>, ParseError> {
		if byte == hex::START_BYTE && self.state != State::TextChecksum {
			self.begin_hex_frame();
		}

		if self.state != State::HexFrame {
			self.text_checksum = self.text_checksum.wrapping_add(byte);
		}

		// Capitalize the byte
		let byte = byte.to_ascii_uppercase();

		match self.state {
			State::Idle => self.push_idle(byte),
			State::TextLabel => self.push_text_label(byte),
			State::TextValue => self.push_text_value(byte),
			State::TextChecksum => self.push_text_checksum(),
			State::HexFrame => self.push_hex_byte(byte),
		}
	}

	fn push_idle(&mut self, byte: u8) -> Result<Option<Event<'_>>, ParseError> {
		match byte {
			// Ignore carriage returns
			b'\r' => Ok(None),
			// Newline indicates the start of a text line
			b'\n' => {
				self.begin_text_line();
				Ok(None)
			}
			// Ignore other bytes in idle state
			_ => Ok(None),
		}
	}

	fn begin_text_line(&mut self) {
		self.state = State::TextLabel;
		self.in_text_block = true;
		self.text_label.clear();
		self.text_value.clear();
	}

	fn push_text_label(&mut self, byte: u8) -> Result<Option<Event<'_>>, ParseError> {
		match byte {
			b'\t' => {
				// Check for checksum label to determine if this is a checksum line or not
				if self.text_label.as_slice() == CHECKSUM_LABEL {
					self.state = State::TextChecksum;
				} else {
					self.state = State::TextValue;
				}
				Ok(None)
			}
			_ => {
				self.text_label
					.push(byte)
					.map_err(|_| ParseError::TextLabelTooLong)?;
				Ok(None)
			}
		}
	}

	fn push_text_value(&mut self, byte: u8) -> Result<Option<Event<'_>>, ParseError> {
		match byte {
			// Ignore carriage returns
			b'\r' => Ok(None),
			b'\n' => {
				self.state = State::TextLabel;
				let record = TextRecord {
					label: self.text_label.as_slice(),
					value: self.text_value.as_slice(),
				};
				Ok(Some(Event::TextRecord(record)))
			}
			_ => {
				self.text_value
					.push(byte)
					.map_err(|_| ParseError::TextValueTooLong)?;
				Ok(None)
			}
		}
	}

	fn push_text_checksum(&mut self) -> Result<Option<Event<'_>>, ParseError> {
		// Reset the state and text block flag regardless of whether the checksum is valid or not
		self.state = State::Idle;
		self.in_text_block = false;

		let is_valid = self.text_checksum == 0;
		if !is_valid {
			return Err(ParseError::TextChecksumInvalid);
		} else {
			return Ok(Some(Event::TextBlockEnd));
		}
	}

	fn begin_hex_frame(&mut self) {
		self.state = State::HexFrame;
		self.hex_frame.clear();
	}

	fn push_hex_byte(&mut self, byte: u8) -> Result<Option<Event<'_>>, ParseError> {
		match byte {
			hex::END_BYTE => return self.finish_hex_frame(),
			_ => {
				self.hex_frame
					.push(byte)
					.map_err(|_| ParseError::HexFrameTooLong)?;
				Ok(None)
			}
		}
	}

	fn finish_hex_frame(&mut self) -> Result<Option<Event<'_>>, ParseError> {
		self.state = State::Idle;

		let raw_frame =
			hex::RawFrame::from_ascii_hex(self.hex_frame.as_slice(), &mut self.hex_payload)
				.map_err(ParseError::InvalidHex)?;
		Ok(Some(Event::HexFrame(raw_frame)))
	}
}
