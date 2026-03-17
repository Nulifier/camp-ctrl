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
	/// Main or channel 1 (battery) voltage (mV)
	Voltage1,
	/// Channel 2 (battery) voltage (mV)
	Voltage2,
	/// Channel 3 (battery) voltage (mV)
	Voltage3,
	/// Auxiliary (starter) voltage (mV)
	VoltageAux,
	/// Mid-point voltage of the battery bank (mV)
	VoltageMidPoint,
	/// Mid-point deviation of the battery bank (0.1%)
	VoltageMidPointDeviation,
	/// Panel voltage (mV)
	PanelVoltage,
	/// Panel power (W)
	PanelPower,
	/// Main or channel 1 battery current (mA)
	Current1,
	/// Channel 2 battery current (mA)
	Current2,
	/// Channel 3 battery current (mA)
	Current3,
	/// Load current (mA)
	LoadCurrent,
	/// Load output state (ON/OFF)
	LoadOutputState,
	/// Battery temperature (°C)
	BatteryTemperature,
	/// Instantaneous power (W)
	Power,
	/// Consumed amp hours (mAh)
	ConsumedAmpHours,
	/// State-of-charge (0.1%)
	StateOfCharge,
	/// Time-to-go (minutes)
	TimeToGo,
	/// Alarm condition active
	AlarmCondition,
	/// Relay state
	RelayState,
	/// Alarm reason
	AlarmReason,
	/// Off reason
	OffReason,
	/// Depth of the deepest discharge (mAh)
	DeepestDischargeDepth,
	/// Depth of the last discharge (mAh)
	LastDischargeDepth,
	/// Depth of the average discharge (mAh)
	AverageDischargeDepth,
	/// Number of charge cycles
	ChargeCycles,
	/// Number of full discharges
	FullDischarges,
	/// Cumulative amp hours drawn (mAh)
	CumulativeAmpHours,
	/// Minimum main (battery) voltage (mV)
	MinVoltage1,
	/// Maximum main (battery) voltage (mV)
	MaxVoltage1,
	/// Number of seconds since last full charge (seconds)
	SecondsSinceFullCharge,
	/// Number of automatic synchronizations
	AutomaticSynchronizations,
	/// Number of low main voltage alarms
	LowVoltageAlarms,
	/// Number of high main voltage alarms
	HighVoltageAlarms,
	/// Number of low auxiliary voltage alarms
	LowAuxVoltageAlarms,
	/// Number of high auxiliary voltage alarms
	HighAuxVoltageAlarms,
	/// Minimum auxiliary (battery) voltage (mV)
	MinVoltageAux,
	/// Maximum auxiliary (battery) voltage (mV)
	MaxVoltageAux,
	/// Amount of discharged energy (BMV) / Amount of produced energy (DC monitor) (0.01 kWh)
	Energy,
	/// Amount of charged energy (BMV) / Amount of consumed energy (DC monitor) (0.01 kWh)
	ChargedEnergy,
	/// Yield total (user resettable counter) (0.01kWh)
	YieldTotal,
	/// Yield today (0.01kWh)
	YieldToday,
	/// Maximum power today (W)
	MaxPowerToday,
	/// Yield yesterday (0.01kWh)
	YieldYesterday,
	/// Maximum power yesterday (W)
	MaxPowerYesterday,
	/// Error code
	ErrorCode,
	/// State of operation
	StateOfOperation,
	/// Model description (deprecated)
	ModelDescription,
	/// Firmware version (16 bit)
	FirmwareVersion16,
	/// Firmware version (24 bit)
	FirmwareVersion24,
	/// Product ID
	ProductId,
	/// Serial number
	SerialNumber,
	/// Day sequence number (0..364)
	DaySequence,
	/// Device mode
	DeviceMode,
	/// AC output voltage (0.01 V)
	AcOutputVoltage,
	/// AC output current (0.1 A)
	AcOutputCurrent,
	/// AC output apparent power (VA)
	AcOutputApparentPower,
	/// Warning reason
	WarningReason,
	/// Tracker operation mode
	TrackerOperationMode,
	/// DC monitor mode
	DcMonitorMode,
	/// DC input voltage (0.01 V)
	DcInputVoltage,
	/// DC input current (0.1 A)
	DcInputCurrent,
	/// DC input power (W)
	DcInputPower,
	Checksum,
	Unknown,
}

impl KnownLabel {
	pub fn from_label(label: &str) -> Self {
		match label {
			"V" => Self::Voltage1,
			"V2" => Self::Voltage2,
			"V3" => Self::Voltage3,
			"VS" => Self::VoltageAux,
			"VM" => Self::VoltageMidPoint,
			"DM" => Self::VoltageMidPointDeviation,
			"VPV" => Self::PanelVoltage,
			"PPV" => Self::PanelPower,
			"I" => Self::Current1,
			"I2" => Self::Current2,
			"I3" => Self::Current3,
			"IL" => Self::LoadCurrent,
			"LOAD" => Self::LoadOutputState,
			"T" => Self::BatteryTemperature,
			"P" => Self::Power,
			"CE" => Self::ConsumedAmpHours,
			"SOC" => Self::StateOfCharge,
			"TTG" => Self::TimeToGo,
			"Alarm" => Self::AlarmCondition,
			"Relay" => Self::RelayState,
			"AR" => Self::AlarmReason,
			"OR" => Self::OffReason,
			"H1" => Self::DeepestDischargeDepth,
			"H2" => Self::LastDischargeDepth,
			"H3" => Self::AverageDischargeDepth,
			"H4" => Self::ChargeCycles,
			"H5" => Self::FullDischarges,
			"H6" => Self::CumulativeAmpHours,
			"H7" => Self::MinVoltage1,
			"H8" => Self::MaxVoltage1,
			"H9" => Self::SecondsSinceFullCharge,
			"H10" => Self::AutomaticSynchronizations,
			"H11" => Self::LowVoltageAlarms,
			"H12" => Self::HighVoltageAlarms,
			"H13" => Self::LowAuxVoltageAlarms,
			"H14" => Self::HighAuxVoltageAlarms,
			"H15" => Self::MinVoltageAux,
			"H16" => Self::MaxVoltageAux,
			"H17" => Self::Energy,
			"H18" => Self::ChargedEnergy,
			"H19" => Self::YieldTotal,
			"H20" => Self::YieldToday,
			"H21" => Self::MaxPowerToday,
			"H22" => Self::YieldYesterday,
			"H23" => Self::MaxPowerYesterday,
			"ERR" => Self::ErrorCode,
			"CS" => Self::StateOfOperation,
			"BMV" => Self::ModelDescription,
			"FW" => Self::FirmwareVersion16,
			"FWE" => Self::FirmwareVersion24,
			"PID" => Self::ProductId,
			"SER#" => Self::SerialNumber,
			"HSDS" => Self::DaySequence,
			"MODE" => Self::DeviceMode,
			"AC_OUT_V" => Self::AcOutputVoltage,
			"AC_OUT_I" => Self::AcOutputCurrent,
			"AC_OUT_S" => Self::AcOutputApparentPower,
			"WARN" => Self::WarningReason,
			"MPPT" => Self::TrackerOperationMode,
			"MON" => Self::DcMonitorMode,
			"DC_IN_V" => Self::DcInputVoltage,
			"DC_IN_I" => Self::DcInputCurrent,
			"DC_IN_P" => Self::DcInputPower,
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
			KnownLabel::Voltage1 => self.battery_voltage_mv = Some(record.value_as_i32()?),
			KnownLabel::Checksum | KnownLabel::Unknown => {}
			_ => {}
		}

		Ok(())
	}
}

pub fn is_checksum_label(label: &[u8]) -> bool {
	label.eq_ignore_ascii_case(CHECKSUM_LABEL.as_bytes())
}

fn parse_u16(bytes: &[u8]) -> Result<u16, TextError> {
	let value = str::from_utf8(bytes).map_err(|_| TextError::InvalidValueUtf8)?;
	value.parse::<u16>().map_err(|_| TextError::InvalidInteger)
}

fn parse_i32(bytes: &[u8]) -> Result<i32, TextError> {
	let value = str::from_utf8(bytes).map_err(|_| TextError::InvalidValueUtf8)?;
	value.parse::<i32>().map_err(|_| TextError::InvalidInteger)
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
