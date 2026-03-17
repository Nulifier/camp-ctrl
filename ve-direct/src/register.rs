use bitflags::bitflags;
use core::fmt;
use heapless::String;
use num_enum::{IntoPrimitive, TryFromPrimitive};

//////////////////////////////////////////////////////////////////////////////
// NOTE: This module is meant to be used with the data sheet. There are many
//       fields that have notes and usage information in the data sheet.
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum RegisterId {
	// Product information registers
	ProductId = 0x0100,
	GroupId = 0x0104,
	SerialNumber = 0x010A,
	ModelName = 0x010B,
	Capabilities = 0x0140,
	// Generic device control registers
	DeviceMode = 0x0200,
	DeviceState = 0x0201,
	RemoteControlUsed = 0x0202,
	DeviceOffReason = 0x0207, // In versions prior to 1.44, this was a u8 in 0x0205
	// Battery settings registers
	BatterySafeMode = 0xEDFF,	// Disabled in firmware 1.13, removed in firmware 1.42
	AdaptiveMode = 0xEDFE,
	AutomaticEqualisationMode = 0xEDFD,
	BatteryBulkTimeLimit = 0xEDFC,
	BatteryAbsorptionTimeLimit = 0xEDFB,
	BatteryAbsorptionVoltage = 0xEDF7,
	BatteryFloatVoltage = 0xEDF6,
	BatteryEqualisationVoltage = 0xEDF4,
	BatteryTempCompensation = 0xEDF2,
	BatteryType = 0xEDF1,
	BatteryMaximumCurrent = 0xEDF0,
	BatteryVoltage = 0xEDEF,
	BatteryTemperature = 0xEDEC,
	BatteryVoltageSetting = 0xEDEA,
	BmsPresent = 0xEDE8,
	TailCurrent = 0xEDE7,
	LowTemperatureChargeCurrent = 0xEDE6,
	AutoEqualiseStopOnVoltage = 0xEDE5,
	EqualisationCurrentLevel = 0xEDE4,
	EqualisationDuration = 0xEDE3,
	RebulkVoltageOffset = 0xEDE2, // @TODO Datasheet has this as 0xED2E
	BatteryLowTemperatureLevel = 0xEDE0,
	VoltageCompensation = 0xEDCA,
	// 2-wire BMS input - MPPT RS models only
	RemoteInputMode = 0xD0C0,
	TwoWireBmsInput = 0xD01F,
	// Charger data registers
	//BatteryTemperature = 0xEDEC,
	ChargerMaximumCurrent = 0xEDDF,
	SystemYield = 0xEDDD,
	UserYield = 0xEDDC,
	ChargerInternalTemperature = 0xEDDB,
	ChargerErrorCode = 0xEDDA,
	ChargerCurrent = 0xEDD7,
	ChargerVoltage = 0xEDD5,
	AdditionalChargerInfo = 0xEDD4,
	YieldToday = 0xEDD3,
	MaxPowerToday = 0xEDD2,
	YieldYesterday = 0xEDD1,
	MaxPowerYesterday = 0xEDD0,
	VoltageSettingsRange = 0xEDCE,
	HistoryVersion = 0xEDCD,
	StreetlightVersion = 0xEDCC,
	EqualiseCurrentMax = 0xEDC7,
	EqualiseVoltageMax = 0xEDC6,
	AdjVoltageMin = 0x2211,
	AdjVoltageMax = 0x2212,
	// DC channel registers - MPPT RS models only
	BatteryRippleVoltage = 0xED8B,
	BatteryVoltageRs = 0xED8D, // Replaces 0xEDD5
	BatteryCurrentRs = 0xED8F, // Replaces 0xEDD7
	// Solar panel data registers
	NumberOfMpptTrackers = 0x0244,
	PanelMaxCurrent = 0xEDBF,
	PanelPower = 0xEDBC,
	PanelVoltage = 0xEDBB,
	PanelCurrent = 0xEDBD,
	PanelMaxVoltage = 0xEDB8,
	TrackerMode = 0xEDB3,
	PanelStartingVoltage = 0xEDB2,
	PanelInputResistance = 0xEDB1,
	// Solar panel data individual MPPT tracker - MPPT RS models only
	PanelPowerTracker1 = 0xECCC,
	PanelPowerTracker2 = 0xECDC,
	PanelPowerTracker3 = 0xECEC,
	PanelPowerTracker4 = 0xECFC,
	PanelVoltageTracker1 = 0xECCB,
	PanelVoltageTracker2 = 0xECDB,
	PanelVoltageTracker3 = 0xECEB,
	PanelVoltageTracker4 = 0xECFB,
	PanelCurrentTracker1 = 0xECCD,
	PanelCurrentTracker2 = 0xECDD,
	PanelCurrentTracker3 = 0xECED,
	PanelCurrentTracker4 = 0xECFD,
	Tracker1Mode = 0xECC3,
	Tracker2Mode = 0xECD3,
	Tracker3Mode = 0xECE3,
	Tracker4Mode = 0xECF3,
	// Load output data/settings registers
	LoadCurrent = 0xEDAD,
	LoadOffsetVoltage = 0xEDAC,
	LoadOutputControl = 0xEDAB,
	LoadOutputVoltage = 0xEDA9,
	LoadOutputState = 0xEDA8,
	LoadSwitchHighLevel = 0xED9D,
	LoadSwitchLowLevel = 0xED9C,
	LoadOutputOffReason = 0xED91,
	//LoadAesTimer = 0xED90,
	// Relay settings registers
	RelayOperationMode = 0xEDD9,
	RelayBatteryLowVoltageSet = 0x0350,
	RelayBatteryLowVoltageClear = 0x0351,
	RelayBatteryHighVoltageSet = 0x0352,
	RelayBatteryHighVoltageClear = 0x0353,
	RelayPanelHighVoltageSet = 0xEDBA,
	RelayPanelHighVoltageClear = 0xEDB9,
	RelayMinimumEnabledTime = 0x100A,
	// Lighting controller timer
	TimerEvent0 = 0xEDA0,
	TimerEvent1 = 0xEDA1,
	TimerEvent2 = 0xEDA2,
	TimerEvent3 = 0xEDA3,
	TimerEvent4 = 0xEDA4,
	TimerEvent5 = 0xEDA5,
	MidPointShift = 0xEDA7,
	GradualDimSpeed = 0xED9B,
	PanelVoltageNight = 0xED9A,
	PanelVoltageDay = 0xED99,
	SunsetDelay = 0xED96,
	SunriseDelay = 0xED97,
	AesTimer = 0xED90,	// Duplicate of LoadAesTimer
	SolarActivity = 0x2030,
	TimeOfDay = 0x2031,
	// VE.Direct port functions
	TxPortOperationMode = 0xED9E,
	RxPortOperationMode = 0xED98,
	// Restore factory defaults
	RestoreDefault = 0x0004,
	// History data
	ClearHistory = 0x1030,
	// Total history TODO
	// Daily history TODO
	// Daily MPPT history TODO
	// Pluggable display settings
	DisplayBacklightModePluggable = 0x0400,
	DisplayBacklightIntensity = 0x0401,
	DisplayScrollTextSpeed = 0x0402,
	DisplaySetupLock = 0x0403,
	DisplayTemperatureUnit = 0x0404,
	DisplayContrast = 0x0406,
	DisplayBacklightMode = 0x0408,
	// Remote control registers
	ChargeAlgorithmVersion = 0x2000,
	ChargeVoltageSetPoint = 0x2001,
	BatteryVoltageSense = 0x2002,
	BatteryTemperatureSense = 0x2003,
	RemoteCommand = 0x2004,
	ChargeStateElapsedTime = 0x2007,
	AbosrptionTime = 0x2008,
	ErrorCode = 0x2009,
	BatteryChargeCurrent = 0x200A,
	BatteryIdleVoltage = 0x200B,
	DeviceStateRemote = 0x200C,
	NetworkInfo = 0x200D,
	NetworkStatusRegister = 0x200F,
	TotalChargeCurrent = 0x2013,
	ChargeCurrentPercentage = 0x2014,
	ChargeCurrentlLimit = 0x2015,
	ManualEqualisationPending = 0x2018,
	TotalDcInputPower = 0x2027,
}

pub trait DecodeValue: Sized {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError>;
}

pub trait EncodeValue {
	fn encoded_len(&self) -> usize;

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError>;
}

pub trait Register: Sized {
	const ID: RegisterId;
	type Value: DecodeValue + EncodeValue;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegisterError {
	InvalidLength {
		expected: usize,
		actual: usize,
	},
	InvalidLengthRange {
		min: usize,
		max: usize,
		actual: usize,
	},
	InvalidUtf8,
	StringTooLong {
		actual: usize,
	},
	InvalidEnum(u32),
	BufferTooSmall {
		required: usize,
		actual: usize,
	},
}

impl fmt::Display for RegisterError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidLength { expected, actual } => {
				write!(
					f,
					"invalid register value length: expected {expected}, got {actual}"
				)
			}
			Self::InvalidLengthRange { min, max, actual } => write!(
				f,
				"invalid register value length: expected {min}..={max}, got {actual}"
			),
			Self::InvalidUtf8 => f.write_str("register value is not valid UTF-8"),
			Self::StringTooLong { actual } => write!(
				f,
				"register string exceeds capacity {MAX_STRING_LENGTH} bytes: got {actual}"
			),
			Self::InvalidEnum(bits) => {
				write!(
					f,
					"register value contains unsupported capability bits: {bits:#010X}"
				)
			}
			Self::BufferTooSmall { required, actual } => {
				write!(
					f,
					"output buffer too small: need {required} bytes, got {actual}"
				)
			}
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for RegisterError {}

impl DecodeValue for u8 {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		expect_len(bytes, 1)?;
		Ok(bytes[0])
	}
}

impl EncodeValue for u8 {
	fn encoded_len(&self) -> usize {
		1
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, 1)?;
		out[0] = *self;
		Ok(&out[..1])
	}
}

impl DecodeValue for u16 {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		expect_len(bytes, 2)?;
		Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
	}
}

impl EncodeValue for u16 {
	fn encoded_len(&self) -> usize {
		2
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, 2)?;
		out[..2].copy_from_slice(&self.to_le_bytes());
		Ok(&out[..2])
	}
}

mod private {
	pub trait Sealed {}
}

/// Marker trait for `#[repr(u8)]`/`#[repr(u16)]` enums that map bijectively
/// to their primitive. Implement this (along with `TryFromPrimitive` and
/// `IntoPrimitive`) to get automatic `DecodeValue` and `EncodeValue` impls.
pub trait ReprEnum: private::Sealed + TryFromPrimitive + Copy
where
	Self: Into<<Self as TryFromPrimitive>::Primitive>,
{
}

impl<T> DecodeValue for T
where
	T: ReprEnum,
	<T as TryFromPrimitive>::Primitive: DecodeValue + Into<u32>,
{
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		let raw = <T as TryFromPrimitive>::Primitive::decode(bytes)?;
		T::try_from_primitive(raw).map_err(|_| RegisterError::InvalidEnum(raw.into()))
	}
}

impl<T> EncodeValue for T
where
	T: ReprEnum,
	<T as TryFromPrimitive>::Primitive: EncodeValue,
{
	fn encoded_len(&self) -> usize {
		let raw: <T as TryFromPrimitive>::Primitive = (*self).into();
		raw.encoded_len()
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		let raw: <T as TryFromPrimitive>::Primitive = (*self).into();
		raw.encode_into(out)
	}
}

pub const MAX_STRING_LENGTH: usize = 32;
pub type VeDirectString = String<MAX_STRING_LENGTH>;

impl DecodeValue for VeDirectString {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		if bytes.len() > MAX_STRING_LENGTH {
			return Err(RegisterError::StringTooLong {
				actual: bytes.len(),
			});
		}

		let value = core::str::from_utf8(bytes).map_err(|_| RegisterError::InvalidUtf8)?;
		let mut out = VeDirectString::new();
		out.push_str(value)
			.map_err(|_| RegisterError::StringTooLong {
				actual: bytes.len(),
			})?;
		Ok(out)
	}
}

impl EncodeValue for VeDirectString {
	fn encoded_len(&self) -> usize {
		self.len()
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, self.len())?;
		out[..self.len()].copy_from_slice(self.as_bytes());
		Ok(&out[..self.len()])
	}
}

pub struct ProductIdRegister;
impl Register for ProductIdRegister {
	const ID: RegisterId = RegisterId::ProductId;
	type Value = u16;
}

pub struct GroupIdRegister;
impl Register for GroupIdRegister {
	const ID: RegisterId = RegisterId::GroupId;
	type Value = u8;
}

pub struct SerialNumberRegister;
impl Register for SerialNumberRegister {
	const ID: RegisterId = RegisterId::SerialNumber;
	type Value = VeDirectString;
}

pub struct ModelNameRegister;
impl Register for ModelNameRegister {
	const ID: RegisterId = RegisterId::ModelName;
	type Value = VeDirectString;
}


bitflags! {
	#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
	pub struct Capabilities: u32 {
		/// Load output present
		const LOAD_OUTPUT = 0b0000_0000_0000_0000_0000_0000_0000_0001;
		/// Rotary encoder present
		const ROTARY_ENCODER = 0b0000_0000_0000_0000_0000_0000_0000_0010;
		/// History support
		const HISTORY_SUPPORT = 0b0000_0000_0000_0000_0000_0000_0000_0100;
		/// Batterysafe mode
		const BATTERYSAFE_MODE = 0b0000_0000_0000_0000_0000_0000_0000_1000;
		/// Adaptive mode
		const ADAPTIVE_MODE = 0b0000_0000_0000_0000_0000_0000_0001_0000;
		/// Manual equalise
		const MANUAL_EQUALIZE = 0b0000_0000_0000_0000_0000_0000_0010_0000;
		/// Automatic equalise
		const AUTOMATIC_EQUALIZE = 0b0000_0000_0000_0000_0000_0000_0100_0000;
		/// Storage mode
		const STORAGE_MODE = 0b0000_0000_0000_0000_0000_0000_1000_0000;
		/// Remote on/off via rx pin
		const REMOTE_ON_OFF = 0b0000_0000_0000_0000_0000_0001_0000_0000;
		/// Solar timer/streetlighting
		const SOLAR_TIMER = 0b0000_0000_0000_0000_0000_0010_0000_0000;
		/// Alternative VE.Direct TX pin function
		const ALT_TX_PIN_FUNC = 0b0000_0000_0000_0000_0000_0100_0000_0000;
		/// User defined load switch
		const USER_LOAD_SWITCH = 0b0000_0000_0000_0000_0000_1000_0000_0000;
		/// Load current in TEXT protocol
		const LOAD_CURRENT_TEXT = 0b0000_0000_0000_0000_0001_0000_0000_0000;
		/// Panel current
		const PANEL_CURRENT = 0b0000_0000_0000_0000_0010_0000_0000_0000;
		/// BMS support
		const BMS_SUPPORT = 0b0000_0000_0000_0000_0100_0000_0000_0000;
		/// External control support
		const EXTERNAL_CONTROL = 0b0000_0000_0000_0000_1000_0000_0000_0000;
		/// Synchronized charging support
		const SYNCHRONIZED_CHARGING = 0b0000_0000_0000_0001_0000_0000_0000_0000;
		/// Alarm relay
		const ALARM_RELAY = 0b0000_0000_0000_0010_0000_0000_0000_0000;
		/// Alternative VE.Direct RX pin function
		const ALT_RX_PIN_FUNC = 0b0000_0000_0000_0100_0000_0000_0000_0000;
		/// Virtual load support
		const VIRTUAL_LOAD = 0b0000_0000_0000_1000_0000_0000_0000_0000;
		/// Virtual relay
		const VIRTUAL_RELAY = 0b0000_0000_0001_0000_0000_0000_0000_0000;
		/// Plugin display support
		const PLUGIN_DISPLAY = 0b0000_0000_0010_0000_0000_0000_0000_0000;
		/// Load Automatic Energy Selector
		const LOAD_AUTO_ENERGY_SELECTOR = 0b0000_0000_0100_0000_0000_0000_0000_0000;
		/// Battery test
		const BATTERY_TEST = 0b0000_0000_1000_0000_0000_0000_0000_0000;
		/// PAYGO support
		const PAYGO = 0b0000_0001_0000_0000_0000_0000_0000_0000;
	}
}

impl DecodeValue for Capabilities {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		expect_len(bytes, 4)?;
		let bits = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
		Capabilities::from_bits(bits).ok_or(RegisterError::InvalidEnum(bits))
	}
}

impl EncodeValue for Capabilities {
	fn encoded_len(&self) -> usize {
		4
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, 4)?;
		out[..4].copy_from_slice(&self.bits().to_le_bytes());
		Ok(&out[..4])
	}
}

pub struct CapabilitiesRegister;
impl Register for CapabilitiesRegister {
	const ID: RegisterId = RegisterId::Capabilities;
	type Value = Capabilities;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DeviceMode {
	/// Charger off.
	/// Firmware 1.16 and lower use 0 and firmware 1.17 and higher return 4. Both values are accepted.
	Off = 4,
	/// Charger on
	On = 1,
}

impl DecodeValue for DeviceMode {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		expect_len(bytes, 1)?;
		match bytes[0] {
			0 | 4 => Ok(Self::Off),
			1 => Ok(Self::On),
			value => Err(RegisterError::InvalidEnum(value as u32)),
		}
	}
}

impl EncodeValue for DeviceMode {
	fn encoded_len(&self) -> usize {
		1
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, 1)?;
		out[0] = *self as u8;
		Ok(&out[..1])
	}
}

pub struct DeviceModeRegister;
impl Register for DeviceModeRegister {
	const ID: RegisterId = RegisterId::DeviceMode;
	type Value = DeviceMode;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DeviceState {
	/// Not charging
	NotCharging = 0,
	/// Failure
	Fault = 2,
	/// Full current charge with charge current set-point
	Bulk = 3,
	/// Voltage controlled with absorption voltage set-point
	Absorption = 4,
	/// Voltage controlled with float voltage set-point
	Float = 5,
	/// Voltage controlled with storage voltage set-point
	Storage = 6,
	/// Voltage controlled with manual equalisation voltage set-point
	ManualEqualise = 7,
	// The device is about to start (signal to external control)
	WakeUp = 245,
	/// Voltage controlled with equalisation voltage set-point
	AutoEqualise = 247,
	// Unit being updated, it is not available at the moment
	Blocked = 250,
	/// Voltage controlled with remote voltage set-point
	ExternalControl = 252,
	/// No information available
	Unavailable = 255,
}

impl private::Sealed for DeviceState {}
impl ReprEnum for DeviceState {}

pub struct DeviceStateRegister;
impl Register for DeviceStateRegister {
	const ID: RegisterId = RegisterId::DeviceState;
	type Value = DeviceState;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum RemoteControlUsed {
	// Enable remote ON/OFF control
	RemoteOnOff = 1,
}

impl private::Sealed for RemoteControlUsed {}
impl ReprEnum for RemoteControlUsed {}

pub struct RemoteControlUsedRegister;
impl Register for RemoteControlUsedRegister {
	const ID: RegisterId = RegisterId::RemoteControlUsed;
	type Value = RemoteControlUsed;
}

bitflags! {
	#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
	pub struct DeviceOffReason: u32 {
		/// No input power (solar panels)
		const NO_INPUT_POWER = 0b0000_0000_0000_0000_0000_0000_0000_0001;
		/// Physical power switch (MPPT RS models only)
		const POWER_SWITCH = 0b0000_0000_0000_0000_0000_0000_0010;
		/// Soft power switch (device mode or pluggable display)
		const SOFT_POWER_SWITCH = 0b0000_0000_0000_0000_0000_0000_0100;
		/// Remote input (either via VE.Direct RX pin alternative function or dedicated remote input)
		const REMOTE_INPUT = 0b0000_0000_0000_0000_0000_0000_1000;
		/// Internal reason
		const INTERNAL = 0b0000_0000_0000_0000_0000_0001_0000;
		/// Pay-as-you-go out of credit
		const PAYGO = 0b0000_0000_0000_0000_0000_0010_0000;
		/// BMS shutdown
		const BMS_SHUTDOWN = 0b0000_0000_0000_0000_0000_0100_0000;
		/// Battery temperature too low (charging not allowed)
		const BATTERY_TEMP_LOW = 1 << 9;
	}
}

impl DecodeValue for DeviceOffReason {
	fn decode(bytes: &[u8]) -> Result<Self, RegisterError> {
		expect_len(bytes, 4)?;
		let bits = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
		DeviceOffReason::from_bits(bits).ok_or(RegisterError::InvalidEnum(bits))
	}
}

impl EncodeValue for DeviceOffReason {
	fn encoded_len(&self) -> usize {
		4
	}

	fn encode_into<'a>(&self, out: &'a mut [u8]) -> Result<&'a [u8], RegisterError> {
		require_buffer(out, 4)?;
		out[..4].copy_from_slice(&self.bits().to_le_bytes());
		Ok(&out[..4])
	}
}

pub struct DeviceOffReasonRegister;
impl Register for DeviceOffReasonRegister {
	const ID: RegisterId = RegisterId::DeviceOffReason;
	type Value = DeviceOffReason;
}

fn expect_len(bytes: &[u8], expected: usize) -> Result<(), RegisterError> {
	if bytes.len() == expected {
		Ok(())
	} else {
		Err(RegisterError::InvalidLength {
			expected,
			actual: bytes.len(),
		})
	}
}

fn require_buffer(out: &[u8], required: usize) -> Result<(), RegisterError> {
	if out.len() >= required {
		Ok(())
	} else {
		Err(RegisterError::BufferTooSmall {
			required,
			actual: out.len(),
		})
	}
}

#[cfg(test)]
mod tests {
	// use super::*;

	// #[test]
	// fn known_register_lookup_roundtrips() {
	// 	let register = KnownRegister::from_id(RegisterId::DeviceState);
	// 	assert_eq!(register, Some(KnownRegister::DeviceState));
	// 	assert_eq!(register.unwrap().id(), RegisterId::DeviceState);
	// }

	// #[test]
	// fn product_id_value_roundtrips() {
	// 	let value = ProductIdValue::new(0x00, 0xA042);
	// 	let mut encoded = [0u8; 4];
	// 	let encoded = value.encode_into(&mut encoded).unwrap();

	// 	assert_eq!(encoded, &[0x00, 0x42, 0xA0, 0xFF]);
	// 	assert_eq!(ProductIdValue::decode(encoded).unwrap(), value);
	// }

	// #[test]
	// fn serial_number_roundtrips() {
	// 	let decoded = SerialNumber::decode(b"HQ1234ABCDEF").unwrap();
	// 	assert_eq!(decoded.as_str(), "HQ1234ABCDEF");

	// 	let mut encoded = [0u8; SERIAL_NUMBER_CAPACITY];
	// 	assert_eq!(decoded.encode_into(&mut encoded).unwrap(), b"HQ1234ABCDEF");
	// }

	// #[test]
	// fn capabilities_roundtrip() {
	// 	let caps = Capabilities::LOAD_OUTPUT | Capabilities::ROTARY_ENCODER;
	// 	let mut encoded = [0u8; 4];
	// 	assert_eq!(
	// 		caps.encode_into(&mut encoded).unwrap(),
	// 		&[0x03, 0x00, 0x00, 0x00]
	// 	);
	// 	assert_eq!(Capabilities::decode(&encoded).unwrap(), caps);
	// }

	// #[test]
	// fn device_mode_accepts_legacy_off_value() {
	// 	assert_eq!(DeviceMode::decode(&[0]).unwrap(), DeviceMode::Off);
	// 	assert_eq!(DeviceMode::decode(&[4]).unwrap(), DeviceMode::Off);
	// 	assert_eq!(DeviceMode::decode(&[1]).unwrap(), DeviceMode::On);
	// }

	// #[test]
	// fn device_state_roundtrips() {
	// 	let state = DeviceState::ExternalControl;
	// 	let mut encoded = [0u8; 1];
	// 	assert_eq!(state.encode_into(&mut encoded).unwrap(), &[252]);
	// 	assert_eq!(DeviceState::decode(&encoded).unwrap(), state);
	// }

	// #[test]
	// fn remote_control_mask_reports_flag() {
	// 	let value = RemoteControlUsed::new(RemoteControlUsed::REMOTE_ON_OFF);
	// 	assert!(value.remote_on_off_enabled());
	// 	assert_eq!(
	// 		RemoteControlUsed::decode(&[0x02, 0x00, 0x00, 0x00]).unwrap(),
	// 		value
	// 	);
	// }

	// #[test]
	// fn rejects_unsupported_capability_bits() {
	// 	let error = Capabilities::decode(&[0x10, 0x00, 0x00, 0x00]).unwrap_err();
	// 	assert_eq!(error, RegisterError::InvalidCapabilities(0x0000_0010));
	// }
}
