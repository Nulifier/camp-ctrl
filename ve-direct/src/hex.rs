use bitflags::bitflags;
use core::fmt;
use heapless::VecView;

pub const START_BYTE: u8 = b':';
pub const END_BYTE: u8 = b'\n';
pub const ENTER_BOOT_PAYLOAD: [u8; 10] =
	[0x51, 0xFA, 0x51, 0xFA, 0x51, 0xFA, 0x51, 0xFA, 0x51, 0xFA];
pub const VALID_CHECKSUM: u8 = 0x55;

bitflags! {
	#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
	pub struct Flags: u8 {
		const UNKNOWN_ID = 0b0000_0001;
		const NOT_SUPPORTED = 0b0000_0010;
		const PARAMETER_ERROR = 0b0000_0100;

		const _ = !0;
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FirmwareType {
	Bootloader = 0b00000000,
	Application = 0b01000000,
	Tester = 0b10000000,
	ReleaseCandidateC = 0b11000000,
	ReleaseCandidateD = 0b11010000,
	ReleaseCandidateE = 0b11100000,
	ReleaseCandidateF = 0b11110000,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
	pub digits: [u8; 3],
	pub firmware_type: FirmwareType,
}

impl Version {
	pub fn from_raw(raw: u16) -> Self {
		// The version is encoded literally into the 3 nibbles of the version field, with the
		// firmware type in the top 2 bits
		// Eg. 0xD101 is version 1.0.1 release candidate D

		let digits = [
			((raw >> 8) & 0x0F) as u8,
			((raw >> 4) & 0x0F) as u8,
			(raw & 0x0F) as u8,
		];

		// Shift once and match the upper nibble (bits 12..15) using binary literals.
		let ft = ((raw >> 12) & 0b0000_1111) as u8;
		let firmware_type = match ft {
			0b0000..=0b0011 => FirmwareType::Bootloader, // Bits 12 and 13 are undefined
			0b0100..=0b0111 => FirmwareType::Application, // Bits 12 and 13 are undefined
			0b1000..=0b1011 => FirmwareType::Tester,     // Bits 12 and 13 are undefined
			0b1100 => FirmwareType::ReleaseCandidateC,
			0b1101 => FirmwareType::ReleaseCandidateD,
			0b1110 => FirmwareType::ReleaseCandidateE,
			0b1111 => FirmwareType::ReleaseCandidateF,
			_ => unreachable!(),
		};

		Self {
			digits,
			firmware_type,
		}
	}

	pub fn to_raw(self) -> u16 {
		let mut raw = 0u16;
		raw |= (self.digits[0] as u16) << 8;
		raw |= (self.digits[1] as u16) << 4;
		raw |= self.digits[2] as u16;

		// The enum values are already shifted to the correct position, so we can just OR them in.
		raw |= (self.firmware_type as u16) << 8;

		raw
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawFrame<'a> {
	pub code: u8,
	pub payload: &'a [u8],
	pub checksum: u8,
}

impl<'a> RawFrame<'a> {
	pub fn from_ascii_hex(
		input: &'a [u8],
		payload_buffer: &'a mut VecView<u8>,
	) -> Result<Self, HexError> {
		// The buffer contains the frame between the start (':') and end ('\n') bytes, not including
		// them. Important to note that the on-the-line representation is in ASCII hex, so the
		// buffer contains ASCII hex bytes, not the raw payload bytes.

		// Decode the command nibble
		let code = decode_nibble_from_ascii_hex(input[1])?;

		// Get the payload, skipping the command nibble and checksum
		let ascii_payload = &input[1..input.len() - 2];
		let checksum =
			decode_byte_from_ascii_hex([input[input.len() - 2], input[input.len() - 1]])?;

		// The payload is pairs of ASCII hex digits, so the length of the payload must be even
		if ascii_payload.len() % 2 != 0 {
			return Err(HexError::InvalidLength);
		}

		// Ensure the output buffer is large enough for the decoded payload
		let payload_len = ascii_payload.len() / 2;
		if payload_buffer.capacity() < payload_len {
			return Err(HexError::BufferTooSmall);
		}

		// Clear existing payload
		payload_buffer.clear();

		// Decode payload
		let mut input_index = 2;
		let mut output_index = 0;
		while output_index < payload_len {
			payload_buffer[output_index] =
				decode_byte_from_ascii_hex([input[input_index], input[input_index + 1]])?;
			input_index += 2;
			output_index += 1;
		}

		let frame = RawFrame {
			code,
			payload: payload_buffer,
			checksum,
		};

		if !frame.is_checksum_valid() {
			return Err(HexError::InvalidChecksum);
		}

		Ok(frame)
	}

	pub fn from_parts(code: u8, payload: &'a [u8]) -> Self {
		Self {
			code,
			payload,
			checksum: calculate_required_checksum(code, payload),
		}
	}

	pub fn to_ascii_hex<'b>(&self, frame_buffer: &'b mut VecView<u8>) -> Result<(), HexError> {
		// Calculate the required buffer size
		// Command/response nibble plus 2 ASCII hex digits per payload byte plus 2 ASCII hex digits for the checksum
		let frame_size = 1 + self.payload.len() * 2 + 2;

		frame_buffer
			.resize_default(frame_size)
			.map_err(|_| HexError::BufferTooSmall)?;

		// Command/Response byte
		frame_buffer[0] = encode_nibble_to_ascii_hex(self.code);

		// Payload bytes
		let mut buffer_index = 1;
		for &byte in self.payload {
			let ascii_hex = encode_byte_to_ascii_hex(byte);
			frame_buffer[buffer_index] = ascii_hex[0];
			frame_buffer[buffer_index + 1] = ascii_hex[1];
			buffer_index += 2;
		}

		// Checksum byte
		let ascii_hex = encode_byte_to_ascii_hex(self.checksum);
		frame_buffer[buffer_index] = ascii_hex[0];
		frame_buffer[buffer_index + 1] = ascii_hex[1];

		Ok(())
	}

	pub fn calculate_checksum(&self) -> u8 {
		// The checksum is calculated by summing the command nibble and the payload bytes, then
		// taking the result modulo 256. This checksum must equal 0x55

		let mut sum = self.code;
		for &byte in self.payload {
			sum = sum.wrapping_add(byte);
		}

		sum
	}

	pub fn is_checksum_valid(&self) -> bool {
		self.calculate_checksum() == VALID_CHECKSUM
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command<'a> {
	/// Enable bootloader mode.
	/// @TODO Does this return a done response?
	EnterBoot,
	/// Checks for device presence.
	/// Returns Response::Ping with the firmware version and type.
	Ping,
	/// Returns Response::Done with the firmware version.
	AppVersion,
	/// Returns Response::Done with the product ID.
	ProductId,
	/// Restarts the device.
	/// No response is sent.
	Restart,
	/// Returns a Response::Get with the value of the specified parameter.
	Get { id: u16, flags: Flags },
	/// Returns a Response::Set acknowledging the new value of the specified parameter.
	Set {
		id: u16,
		flags: Flags, // @TODO This is marked as set to zero in the docs
		value: &'a [u8],
	},
	/// Asynchronous data message.
	/// Should not be replied
	Async {
		id: u16,
		flags: Flags,
		value: &'a [u8],
	},
}

impl<'a> Command<'a> {
	pub fn from_raw(frame: RawFrame<'a>) -> Result<Self, HexError> {
		match frame.code {
			// Enter boot
			0x0 => {
				if frame.payload == ENTER_BOOT_PAYLOAD {
					Ok(Self::EnterBoot)
				} else {
					Err(HexError::InvalidPayloadLength {
						expected: ENTER_BOOT_PAYLOAD.len(),
						actual: frame.payload.len(),
					})
				}
			}
			// Ping
			0x1 => expect_empty(frame.payload, Self::Ping),
			// App version
			0x3 => expect_empty(frame.payload, Self::AppVersion),
			// Product Id
			0x4 => expect_empty(frame.payload, Self::ProductId),
			// Restart
			0x6 => expect_empty(frame.payload, Self::Restart),
			// Get
			0x7 => {
				if frame.payload.len() != 3 {
					return Err(HexError::InvalidPayloadLength {
						expected: 3,
						actual: frame.payload.len(),
					});
				}

				Ok(Self::Get {
					id: read_u16_le(&frame.payload[..2]),
					flags: read_flags(frame.payload[2])?,
				})
			}
			// Set
			0x8 => {
				if frame.payload.len() < 3 {
					return Err(HexError::InvalidPayloadLength {
						expected: 3,
						actual: frame.payload.len(),
					});
				}

				Ok(Self::Set {
					id: read_u16_le(&frame.payload[..2]),
					flags: read_flags(frame.payload[2])?,
					value: &frame.payload[3..],
				})
			}
			// Async
			0xA => {
				if frame.payload.len() < 3 {
					return Err(HexError::InvalidPayloadLength {
						expected: 3,
						actual: frame.payload.len(),
					});
				}

				Ok(Self::Async {
					id: read_u16_le(&frame.payload[..2]),
					flags: read_flags(frame.payload[2])?,
					value: &frame.payload[3..],
				})
			}
			// Reserved
			_ => Err(HexError::UnknownCode(frame.code)),
		}
	}

	pub fn to_raw<'b>(
		&self,
		payload_buffer: &'b mut VecView<u8>,
	) -> Result<RawFrame<'b>, HexError> {
		match self {
			Self::EnterBoot => Ok(RawFrame::from_parts(0x0, &ENTER_BOOT_PAYLOAD)),
			Self::Ping => Ok(RawFrame::from_parts(0x1, &[])),
			Self::AppVersion => Ok(RawFrame::from_parts(0x3, &[])),
			Self::ProductId => Ok(RawFrame::from_parts(0x4, &[])),
			Self::Restart => Ok(RawFrame::from_parts(0x6, &[])),
			Self::Get { id, flags } => {
				// { id: u16, flags: u8 } = 3 bytes
				payload_buffer
					.resize_default(3)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], *id);
				payload_buffer[2] = flags.bits();
				Ok(RawFrame::from_parts(0x7, payload_buffer.as_slice()))
			}
			Self::Set { id, flags, value } => {
				// { id: u16, flags: u8, value: &[u8] } = 3 bytes + value length
				let total_len = 3 + value.len();
				payload_buffer
					.resize_default(total_len)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], *id);
				payload_buffer[2] = flags.bits();
				payload_buffer[3..].copy_from_slice(value);
				Ok(RawFrame::from_parts(0x8, payload_buffer.as_slice()))
			}
			Self::Async { id, flags, value } => {
				// { id: u16, flags: u8, value: &[u8] } = 3 bytes + value length
				let total_len = 3 + value.len();
				payload_buffer
					.resize_default(total_len)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], *id);
				payload_buffer[2] = flags.bits();
				payload_buffer[3..].copy_from_slice(value);
				Ok(RawFrame::from_parts(0xA, payload_buffer.as_slice()))
			}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response<'a> {
	Done(&'a [u8]),
	Unknown(u8),
	Error(u16),
	Ping(Version),
	Get {
		id: u16,
		flags: Flags,
		value: &'a [u8],
	},
	Set {
		id: u16,
		flags: Flags,
		value: &'a [u8],
	},
}

impl<'a> Response<'a> {
	pub fn from_raw(frame: RawFrame<'a>) -> Result<Self, HexError> {
		match frame.code {
			// Done
			0x1 => Ok(Self::Done(frame.payload)),
			// Unknown
			0x3 => {
				expect_payload_length(frame, 1)?;
				Ok(Self::Unknown(frame.payload[0] & 0x0F))
			}
			// Error
			0x4 => {
				expect_payload_length(frame, 2)?;
				Ok(Self::Error(read_u16_le(frame.payload)))
			}
			// Ping
			0x5 => {
				expect_payload_length(frame, 2)?;
				Ok(Self::Ping(Version::from_raw(read_u16_le(frame.payload))))
			}
			// Get
			0x7 => {
				// Smallest value is a u8, so the payload must be at least 4 bytes
				// {id: u16, flags: u8, value: u8} = 4 bytes minimum
				if frame.payload.len() < 4 {
					return Err(HexError::InvalidPayloadLength {
						expected: 4,
						actual: frame.payload.len(),
					});
				}

				Ok(Self::Get {
					id: read_u16_le(&frame.payload[..2]),
					flags: read_flags(frame.payload[2])?,
					value: &frame.payload[3..],
				})
			}
			// Set
			0x8 => {
				// Smallest value is a u8, so the payload must be at least 4 bytes
				// {id: u16, flags: u8, value: u8} = 4 bytes minimum
				if frame.payload.len() < 4 {
					return Err(HexError::InvalidPayloadLength {
						expected: 4,
						actual: frame.payload.len(),
					});
				}

				Ok(Self::Set {
					id: read_u16_le(&frame.payload[..2]),
					flags: read_flags(frame.payload[2])?,
					value: &frame.payload[3..],
				})
			}
			// Reserved
			_ => Err(HexError::UnknownCode(frame.code)),
		}
	}

	pub fn to_raw<'b>(
		&self,
		payload_buffer: &'b mut VecView<u8>,
	) -> Result<RawFrame<'b>, HexError> {
		match self {
			Self::Done(payload) => {
				payload_buffer
					.resize_default(payload.len())
					.map_err(|_| HexError::BufferTooSmall)?;
				payload_buffer.copy_from_slice(payload);
				Ok(RawFrame::from_parts(0x1, payload_buffer.as_slice()))
			}
			Self::Unknown(code) => {
				payload_buffer
					.resize_default(1)
					.map_err(|_| HexError::BufferTooSmall)?;
				payload_buffer[0] = code & 0x0F;
				Ok(RawFrame::from_parts(0x3, payload_buffer.as_slice()))
			}
			Self::Error(error_code) => {
				payload_buffer
					.resize_default(2)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], *error_code);
				Ok(RawFrame::from_parts(0x4, payload_buffer.as_slice()))
			}
			Self::Ping(version) => {
				payload_buffer
					.resize_default(2)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], version.to_raw());
				Ok(RawFrame::from_parts(0x5, payload_buffer.as_slice()))
			}
			Self::Get { id, flags, value } | Self::Set { id, flags, value } => {
				// { id: u16, flags: u8, value: &[u8] } = 3 bytes + value length
				let total_len = 3 + value.len();
				payload_buffer
					.resize_default(total_len)
					.map_err(|_| HexError::BufferTooSmall)?;
				write_u16_le(&mut payload_buffer[..2], *id);
				payload_buffer[2] = flags.bits();
				payload_buffer[3..].copy_from_slice(value);

				// Determine the code based on get or set
				let code = if matches!(self, Self::Get { .. }) {
					0x7
				} else {
					0x8
				};

				Ok(RawFrame::from_parts(code, payload_buffer.as_slice()))
			}
		}
	}

	pub fn done_as_app_version(&self) -> Option<Version> {
		match self {
			Self::Done(payload) if payload.len() == 2 => {
				Some(Version::from_raw(read_u16_le(payload)))
			}
			_ => None,
		}
	}

	pub fn done_as_product_id(&self) -> Option<u16> {
		match self {
			Self::Done(payload) if payload.len() == 2 => Some(read_u16_le(payload)),
			_ => None,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexError {
	BufferTooSmall,
	InvalidFlags(u8),
	InvalidStart(u8),
	InvalidEnd(u8),
	InvalidLength,
	InvalidCommandNibble(u8),
	InvalidHexDigit(u8),
	InvalidChecksum,
	InvalidPayloadLength { expected: usize, actual: usize },
	UnknownCode(u8),
}

impl fmt::Display for HexError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::BufferTooSmall => f.write_str("buffer too small"),
			Self::InvalidFlags(flags) => write!(f, "invalid flags: {flags:#04X}"),
			Self::InvalidStart(byte) => write!(f, "invalid start byte: {byte:#04X}"),
			Self::InvalidEnd(byte) => write!(f, "invalid end byte: {byte:#04X}"),
			Self::InvalidLength => f.write_str("invalid frame length"),
			Self::InvalidCommandNibble(code) => write!(f, "invalid command nibble: {code:#04X}"),
			Self::InvalidHexDigit(byte) => write!(f, "invalid HEX digit: {byte:#04X}"),
			Self::InvalidChecksum => {
				write!(f, "invalid checksum")
			}
			Self::InvalidPayloadLength { expected, actual } => {
				write!(
					f,
					"invalid payload length: expected {expected}, got {actual}"
				)
			}
			Self::UnknownCode(code) => write!(f, "unknown command/response code: {code:#04X}"),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for HexError {}

/// Encodes a value 0..=15 as an ASCII hex digit (b'0'..=b'9', b'A'..=b'F')
pub fn encode_nibble_to_ascii_hex(value: u8) -> u8 {
	debug_assert!(value < 16);
	match value {
		0..=9 => b'0' + value,
		10..=15 => b'A' + (value - 10),
		_ => unreachable!(),
	}
}

/// Decodes a single ASCII hex digit (b'0'..=b'9', b'A'..=b'F') into its value 0..=15
pub fn decode_nibble_from_ascii_hex(byte: u8) -> Result<u8, HexError> {
	match byte {
		b'0'..=b'9' => Ok(byte - b'0'),
		b'A'..=b'F' => Ok(byte - b'A' + 10),
		_ => Err(HexError::InvalidHexDigit(byte)),
	}
}

/// Encodes a byte as two ASCII hex digits
pub fn encode_byte_to_ascii_hex(value: u8) -> [u8; 2] {
	[
		encode_nibble_to_ascii_hex(value >> 4),
		encode_nibble_to_ascii_hex(value & 0x0F),
	]
}

pub fn decode_byte_from_ascii_hex(bytes: [u8; 2]) -> Result<u8, HexError> {
	Ok((decode_nibble_from_ascii_hex(bytes[0])? << 4) | decode_nibble_from_ascii_hex(bytes[1])?)
}

fn expect_empty<'a>(payload: &'a [u8], value: Command<'a>) -> Result<Command<'a>, HexError> {
	if payload.is_empty() {
		Ok(value)
	} else {
		Err(HexError::InvalidPayloadLength {
			expected: 0,
			actual: payload.len(),
		})
	}
}

fn expect_payload_length<'a>(frame: RawFrame<'a>, expected: usize) -> Result<(), HexError> {
	if frame.payload.len() == expected {
		Ok(())
	} else {
		Err(HexError::InvalidPayloadLength {
			expected,
			actual: frame.payload.len(),
		})
	}
}

fn read_flags(byte: u8) -> Result<Flags, HexError> {
	Flags::from_bits(byte).ok_or(HexError::InvalidFlags(byte))
}

/// Calculate what number needs to be added to the code and payload for the sum to be 0x55
fn calculate_required_checksum(code: u8, payload: &[u8]) -> u8 {
	let mut sum = code;
	for &byte in payload {
		sum = sum.wrapping_add(byte);
	}

	VALID_CHECKSUM.wrapping_sub(sum)
}

fn read_u16_le(bytes: &[u8]) -> u16 {
	u16::from_le_bytes([bytes[0], bytes[1]])
}

fn write_u16_le(out: &mut [u8], value: u16) {
	let bytes = value.to_le_bytes();
	out[0] = bytes[0];
	out[1] = bytes[1];
}

#[cfg(test)]
mod tests {
	// use super::*;

	// #[test]
	// fn parse_bootloader_version() {
	// 	let version = Version::from_raw(0x0101);
	// 	assert_eq!(version.digits, [1, 0, 1]);
	// 	assert_eq!(version.firmware_type, FirmwareType::Bootloader);
	// }

	// #[test]
	// fn encodes_ping_command() {
	// 	let mut out = [0u8; 8];
	// 	let len = Command::Ping.encode(&mut out).unwrap();
	// 	assert_eq!(&out[..len], b":154\n");
	// }

	// #[test]
	// fn decodes_ping_command() {
	// 	let mut payload = [0u8; 0];
	// 	let command = decode_command(b":154\n", &mut payload).unwrap();
	// 	assert_eq!(command, Command::Ping);
	// }

	// #[test]
	// fn get_command_roundtrips() {
	// 	let command = Command::Get {
	// 		id: 0x0100,
	// 		flags: 0,
	// 	};
	// 	let mut encoded = [0u8; 16];
	// 	let len = command.encode(&mut encoded).unwrap();

	// 	let mut payload = [0u8; 3];
	// 	let decoded = decode_command(&encoded[..len], &mut payload).unwrap();
	// 	assert_eq!(decoded, command);
	// }

	// #[test]
	// fn rejects_invalid_checksum() {
	// 	let mut payload = [0u8; 0];
	// 	let error = decode_command(b":155\n", &mut payload).unwrap_err();
	// 	assert_eq!(
	// 		error,
	// 		HexError::InvalidChecksum {
	// 			expected: 0x54,
	// 			actual: 0x55,
	// 		}
	// 	);
	// }

	// #[test]
	// fn rejects_lowercase_hex() {
	// 	let mut payload = [0u8; 0];
	// 	let error = decode_command(b":a4B\n", &mut payload).unwrap_err();
	// 	assert_eq!(error, HexError::InvalidHexDigit(b'a'));
	// }

	// #[test]
	// fn response_done_helpers_decode_common_payloads() {
	// 	let response = Response::Done(&[0x01, 0xD1]);
	// 	assert_eq!(response.done_as_product_id(), Some(0xD101));

	// 	let version = response.done_as_app_version().unwrap();
	// 	assert_eq!(version.digits, [1, 0, 1]);
	// 	assert_eq!(version.firmware_type, FirmwareType::ReleaseCandidate(b'D'));
	// }
}
