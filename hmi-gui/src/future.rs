/// This module is filled with definitions that will eventually come from other crates
/// but those crates haven't been built yet.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcWeekday {
	Sunday = 1,
	Monday = 2,
	Tuesday = 3,
	Wednesday = 4,
	Thursday = 5,
	Friday = 6,
	Saturday = 7,
}

impl RtcWeekday {
	pub fn as_str(&self) -> &'static str {
		match self {
			RtcWeekday::Sunday => "Sunday",
			RtcWeekday::Monday => "Monday",
			RtcWeekday::Tuesday => "Tuesday",
			RtcWeekday::Wednesday => "Wednesday",
			RtcWeekday::Thursday => "Thursday",
			RtcWeekday::Friday => "Friday",
			RtcWeekday::Saturday => "Saturday",
		}
	}

	pub fn as_str_short(&self) -> &'static str {
		match self {
			RtcWeekday::Sunday => "Sun",
			RtcWeekday::Monday => "Mon",
			RtcWeekday::Tuesday => "Tue",
			RtcWeekday::Wednesday => "Wed",
			RtcWeekday::Thursday => "Thu",
			RtcWeekday::Friday => "Fri",
			RtcWeekday::Saturday => "Sat",
		}
	}

	pub fn as_u8(&self) -> u8 {
		*self as u8
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtcTime {
	pub is12h: bool,
	pub is_pm: bool,
	pub year: u16,
	pub month: u8,
	pub day_of_week: RtcWeekday,
	pub day_of_month: u8,
	pub hours: u8,
	pub minutes: u8,
	pub seconds: u8,
}

impl RtcTime {
	pub fn display<'a, 'b>(&'a self, fmt: &'b str) -> Display<'a, 'b> {
		Display { inner: self, fmt }
	}
}

pub struct Display<'a, 'b> {
	inner: &'a RtcTime,
	fmt: &'b str,
}

impl core::fmt::Display for Display<'_, '_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		// Iterate through fmt_str and write the appropriate values from inner into the formatter
		// Flags are loosely based on chrono::format::strftime
		// See: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
		let mut i = 0;
		while i < self.fmt.len() {
			if &self.fmt[i..i + 2] == "%Y" {
				// Year with century, zero-padded to 4 digits
				write!(f, "{:04}", self.inner.year)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%m" {
				// Month number, zero-padded to 2 digits
				write!(f, "{:02}", self.inner.month)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%b" {
				// Short month name, always 3 letters
				write!(f, "{}", get_month_name_short(self.inner.month)?)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%B" {
				// Long month name
				write!(f, "{}", get_month_name_long(self.inner.month)?)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%d" {
				// Day of the month, zero-padded to 2 digits
				write!(f, "{:02}", self.inner.day_of_month)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%e" {
				// Day of the month, space-padded to 2 digits
				write!(f, "{:2}", self.inner.day_of_month)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%a" {
				// Short weekday name, always 3 letters
				write!(f, "{}", self.inner.day_of_week.as_str_short())?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%A" {
				// Long weekday name
				write!(f, "{}", self.inner.day_of_week.as_str())?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%w" {
				// Weekday number, Sunday = 0
				write!(f, "{}", self.inner.day_of_week.as_u8() - 1)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%u" {
				// Weekday number, Monday = 1
				write!(f, "{}", self.inner.day_of_week.as_u8())?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%H" {
				let hour = if self.inner.is12h {
					if self.inner.is_pm {
						self.inner.hours + 12
					} else {
						self.inner.hours
					}
				} else {
					self.inner.hours
				};
				write!(f, "{:02}", hour)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%I" {
				let hour = self.inner.hours % 12;
				// Hour in 12h format, zero-padded to 2 digits
				write!(f, "{:02}", if hour == 0 { 12 } else { hour })?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%M" {
				write!(f, "{:02}", self.inner.minutes)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%S" {
				write!(f, "{:02}", self.inner.seconds)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%P" {
				// am/pm in lowercase
				write!(f, "{}", if self.inner.is_pm { "pm" } else { "am" })?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%p" {
				// AM/PM in uppercase
				write!(f, "{}", if self.inner.is_pm { "PM" } else { "AM" })?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%F" {
				// Equivalent to %Y-%m-%d
				write!(
					f,
					"{:04}-{:02}-{:02}",
					self.inner.year, self.inner.month, self.inner.day_of_month
				)?;
				i += 2;
			} else if &self.fmt[i..i + 2] == "%+" {
				// ISO 8601 date and time format
				write!(
					f,
					"{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
					self.inner.year,
					self.inner.month,
					self.inner.day_of_month,
					self.inner.hours,
					self.inner.minutes,
					self.inner.seconds,
					if self.inner.is12h {
						if self.inner.is_pm { " PM" } else { " AM" }
					} else {
						""
					}
				)?;
				i += 2;
			} else {
				write!(f, "{}", &self.fmt[i..i + 1])?;
				i += 1;
			}
		}
		Ok(())
	}
}

fn get_month_name_short(month: u8) -> Result<&'static str, core::fmt::Error> {
	match month {
		1 => Ok("Jan"),
		2 => Ok("Feb"),
		3 => Ok("Mar"),
		4 => Ok("Apr"),
		5 => Ok("May"),
		6 => Ok("Jun"),
		7 => Ok("Jul"),
		8 => Ok("Aug"),
		9 => Ok("Sep"),
		10 => Ok("Oct"),
		11 => Ok("Nov"),
		12 => Ok("Dec"),
		_ => Err(core::fmt::Error),
	}
}

fn get_month_name_long(month: u8) -> Result<&'static str, core::fmt::Error> {
	match month {
		1 => Ok("January"),
		2 => Ok("February"),
		3 => Ok("March"),
		4 => Ok("April"),
		5 => Ok("May"),
		6 => Ok("June"),
		7 => Ok("July"),
		8 => Ok("August"),
		9 => Ok("September"),
		10 => Ok("October"),
		11 => Ok("November"),
		12 => Ok("December"),
		_ => Err(core::fmt::Error),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_display() {
		let time = RtcTime {
			is12h: false,
			is_pm: false,
			year: 2024,
			month: 6,
			day_of_week: RtcWeekday::Monday,
			day_of_month: 10,
			hours: 14,
			minutes: 30,
			seconds: 45,
		};

		assert_eq!(
			time.display("%Y-%m-%d %H:%M:%S").to_string(),
			"2024-06-10 14:30:45"
		);
	}
}
