use core::{fmt, str};

pub const FIELD_SEPARATOR: u8 = b'\t';
pub const BLOCK_START: u8 = b'\n';
pub const CHECKSUM_LABEL: &str = "Checksum";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record<'a> {
	pub label: &'a str,
	pub value: &'a [u8],
}

impl<'a> Record<'a> {
	pub fn new(label: &'a str, value: &'a [u8]) -> Self {
		Self { label, value }
	}

	pub fn known_label(&self) -> KnownLabel {
		KnownLabel::from_label(self.label)
	}

	pub fn value_as_str(&self) -> Result<&'a str, TextError> {
		str::from_utf8(self.value).map_err(|_| TextError::InvalidValueUtf8)
	}

	pub fn value_as_i32(&self) -> Result<i32, TextError> {
		parse_i32(self.value)
	}

	pub fn value_as_u16(&self) -> Result<u16, TextError> {
		parse_u16(self.value)
	}

	pub fn value_as_bool(&self) -> Result<bool, TextError> {
		match self.value {
			b"ON" | b"1" => Ok(true),
			b"OFF" | b"0" => Ok(false),
			_ => Err(TextError::InvalidBooleanValue),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnownLabel {
	Pid,
	Voltage,
	Current,
	PanelVoltage,
	PanelPower,
	StateOfCharge,
	ChargeState,
	TrackerState,
	ErrorCode,
	DaySequence,
	Relay,
	Load,
	Checksum,
	Unknown,
}

impl KnownLabel {
	pub fn from_label(label: &str) -> Self {
		match label {
			"PID" => Self::Pid,
			"V" => Self::Voltage,
			"I" => Self::Current,
			"VPV" => Self::PanelVoltage,
			"PPV" => Self::PanelPower,
			"SOC" => Self::StateOfCharge,
			"CS" => Self::ChargeState,
			"MPPT" => Self::TrackerState,
			"ERR" => Self::ErrorCode,
			"HSDS" => Self::DaySequence,
			"Relay" => Self::Relay,
			"LOAD" => Self::Load,
			CHECKSUM_LABEL => Self::Checksum,
			_ => Self::Unknown,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextError {
	MissingSeparator,
	InvalidLabelUtf8,
	InvalidValueUtf8,
	InvalidInteger,
	InvalidBooleanValue,
}

impl fmt::Display for TextError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::MissingSeparator => f.write_str("missing text field separator"),
			Self::InvalidLabelUtf8 => f.write_str("text label is not valid UTF-8"),
			Self::InvalidValueUtf8 => f.write_str("text value is not valid UTF-8"),
			Self::InvalidInteger => f.write_str("text value is not a valid integer"),
			Self::InvalidBooleanValue => f.write_str("text value is not a valid boolean"),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for TextError {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
	pub battery_voltage_mv: Option<i32>,
	pub battery_current_ma: Option<i32>,
	pub panel_voltage_mv: Option<i32>,
	pub panel_power_w: Option<i32>,
	pub soc_permille: Option<u16>,
	pub charge_state: Option<i32>,
	pub tracker_state: Option<i32>,
	pub error_code: Option<i32>,
	pub day_sequence: Option<u16>,
	pub relay_on: Option<bool>,
	pub load_on: Option<bool>,
}

impl Snapshot {
	pub const fn new() -> Self {
		Self {
			battery_voltage_mv: None,
			battery_current_ma: None,
			panel_voltage_mv: None,
			panel_power_w: None,
			soc_permille: None,
			charge_state: None,
			tracker_state: None,
			error_code: None,
			day_sequence: None,
			relay_on: None,
			load_on: None,
		}
	}

	pub fn clear(&mut self) {
		*self = Self::new();
	}

	pub fn apply_record(&mut self, record: Record<'_>) -> Result<(), TextError> {
		match record.known_label() {
			KnownLabel::Voltage => self.battery_voltage_mv = Some(record.value_as_i32()?),
			KnownLabel::Current => self.battery_current_ma = Some(record.value_as_i32()?),
			KnownLabel::PanelVoltage => self.panel_voltage_mv = Some(record.value_as_i32()?),
			KnownLabel::PanelPower => self.panel_power_w = Some(record.value_as_i32()?),
			KnownLabel::StateOfCharge => self.soc_permille = Some(record.value_as_u16()?),
			KnownLabel::ChargeState => self.charge_state = Some(record.value_as_i32()?),
			KnownLabel::TrackerState => self.tracker_state = Some(record.value_as_i32()?),
			KnownLabel::ErrorCode => self.error_code = Some(record.value_as_i32()?),
			KnownLabel::DaySequence => self.day_sequence = Some(record.value_as_u16()?),
			KnownLabel::Relay => self.relay_on = Some(record.value_as_bool()?),
			KnownLabel::Load => self.load_on = Some(record.value_as_bool()?),
			KnownLabel::Pid | KnownLabel::Checksum | KnownLabel::Unknown => {}
		}

		Ok(())
	}
}

pub fn checksum_sum(block: &[u8]) -> u8 {
	block.iter().fold(0u8, |acc, byte| acc.wrapping_add(*byte))
}

pub fn has_valid_checksum(block: &[u8]) -> bool {
	checksum_sum(block) == 0
}

pub fn is_checksum_label(label: &[u8]) -> bool {
	label.eq_ignore_ascii_case(CHECKSUM_LABEL.as_bytes())
}

fn split_record(line: &[u8]) -> Option<(&[u8], &[u8])> {
	let separator = line.iter().position(|byte| *byte == FIELD_SEPARATOR)?;
	Some((&line[..separator], &line[separator + 1..]))
}

fn parse_i32(bytes: &[u8]) -> Result<i32, TextError> {
	let value = str::from_utf8(bytes).map_err(|_| TextError::InvalidValueUtf8)?;
	value.parse::<i32>().map_err(|_| TextError::InvalidInteger)
}

fn parse_u16(bytes: &[u8]) -> Result<u16, TextError> {
	let value = str::from_utf8(bytes).map_err(|_| TextError::InvalidValueUtf8)?;
	value.parse::<u16>().map_err(|_| TextError::InvalidInteger)
}

#[cfg(test)]
mod tests {
	// use super::*;

	// #[test]
	// fn parses_text_record_line() {
	// 	let record = Record::parse(b"PID\t0xA042").unwrap();

	// 	assert_eq!(record.label, "PID");
	// 	assert_eq!(record.value, b"0xA042");
	// 	assert_eq!(record.known_label(), KnownLabel::Pid);
	// }

	// #[test]
	// fn recognizes_checksum_label() {
	// 	let record = Record::parse(b"Checksum\t\x9f").unwrap();
	// 	assert!(record.is_checksum());
	// 	assert!(is_checksum_label(record.label.as_bytes()));
	// }

	// #[test]
	// fn snapshot_applies_known_values() {
	// 	let mut snapshot = Snapshot::new();
	// 	snapshot
	// 		.apply_record(Record::parse(b"V\t12890").unwrap())
	// 		.unwrap();
	// 	snapshot
	// 		.apply_record(Record::parse(b"PPV\t42").unwrap())
	// 		.unwrap();
	// 	snapshot
	// 		.apply_record(Record::parse(b"LOAD\tON").unwrap())
	// 		.unwrap();

	// 	assert_eq!(snapshot.battery_voltage_mv, Some(12_890));
	// 	assert_eq!(snapshot.panel_power_w, Some(42));
	// 	assert_eq!(snapshot.load_on, Some(true));
	// }

	// #[test]
	// fn checksum_helper_matches_zero_sum_rule() {
	// 	assert!(has_valid_checksum(&[0x01, 0x02, 0xFD]));
	// }
}
