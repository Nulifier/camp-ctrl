use core::cmp::min;

use crate::board::TouchResources;
use crate::error::{Error, Result};
use embassy_rp::gpio::{Flex, Level, Output};
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::{Error as _, Operation};
use heapless::Vec;

const GT911_DEVICE_ADDR: u8 = 0x5D;

#[allow(dead_code)]
const GT911_REG_CONFIG: u16 = 0x8047;
const GT911_REG_PRODUCT_ID: u16 = 0x8140;
const GT911_REG_BUFF_STATUS: u16 = 0x814E;
const GT911_REG_POINTS: u16 = 0x814F;

pub const MAX_NUM_POINTS: usize = 5;

const GT911_POINT_REG_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct TouchPoint {
	pub id: u8,
	pub x: u16,
	pub y: u16,
	pub size: u16,
}

impl TouchPoint {
	pub fn from_bytes(bytes: &[u8; GT911_POINT_REG_SIZE]) -> Self {
		// Byte layout
		// Byte 0: Track ID
		// Byte 1-2: X coordinate (little-endian)
		// Byte 3-4: Y coordinate (little-endian)
		// Byte 5-6: Touch size (little-endian)
		// Byte 7: Reserved
		TouchPoint {
			id: bytes[0],
			x: u16::from_le_bytes([bytes[1], bytes[2]]),
			y: u16::from_le_bytes([bytes[3], bytes[4]]),
			size: u16::from_le_bytes([bytes[5], bytes[6]]),
		}
	}
}

pub type TouchPoints = Vec<TouchPoint, MAX_NUM_POINTS>;

#[derive(Debug, Clone)]
pub struct TouchStatus {
	pub palm: bool,
	pub points: TouchPoints,
}

pub struct TouchDriver<I2C: embedded_hal_async::i2c::I2c> {
	rst_pin: Output<'static>,
	int_pin: Flex<'static>,
	i2c: I2C,
}

impl<I2C: embedded_hal_async::i2c::I2c> TouchDriver<I2C> {
	pub fn new(
		i2c_dev: I2C,
		r: TouchResources,
		// rst_pin: Peri<'static, AnyPin>,
		// int_pin: Peri<'static, AnyPin>,
	) -> Self {
		TouchDriver {
			rst_pin: Output::new(r.tp_rst, Level::Low),
			int_pin: Flex::new(r.tp_int),
			i2c: i2c_dev,
		}
	}

	pub async fn initialize(&mut self) -> Result<()> {
		self.reset().await;

		// Read the product ID
		let product_id = self
			.read_product_id()
			.await
			.map_err(|_| Error::TouchDriverInitFailed)?;

		// Validate we can see the GT911
		if &product_id != b"911\0" {
			return Err(Error::TouchDriverInitFailed);
		}

		Ok(())
	}

	/// Resets the touch sensor
	pub async fn reset(&mut self) {
		// In order to select the correct i2c address, the INT pin needs to be held low during reset.
		self.int_pin.set_as_output();
		self.int_pin.set_low();

		// Reset the touch sensor by toggling the reset pin (active low)
		self.rst_pin.set_low();
		Timer::after(Duration::from_millis(50)).await;
		// Set the reset pin back high to complete the reset
		self.rst_pin.set_high();
		Timer::after(Duration::from_millis(100)).await;

		// Set the INT pin back to input mode so it can be used for interrupts
		self.int_pin.set_as_input();
	}

	pub async fn read_product_id(&mut self) -> Result<[u8; 4]> {
		let mut product_id = [0u8; 4];
		self.read_bytes(GT911_REG_PRODUCT_ID, &mut product_id)
			.await?;
		Ok(product_id)
	}

	pub async fn wait_for_int(&mut self) {
		self.int_pin.wait_for_falling_edge().await;
	}

	pub async fn read_points(&mut self) -> Result<TouchStatus> {
		// Reading protocol:
		// 1. Wait for the interrupt pin to go low (not implemented here, should be handled by caller)
		// 2. Read the number of touch points from GT911_REG_BUFF_STATUS
		// 3. If num_points > 0, read the touch points data from GT911_REG_POINTS
		// 4. Write 0x00 to GT911_REG_BUFF_STATUS to clear the buffer status

		// Status register:
		// Bit7: 1 = buffer contains data, 0 = buffer empty
		// Bit6: 1 = large area (palm) detected, 0 = normal touch
		// Bit3-0: Number of touch points (0-5)

		// Read buffer status
		let mut status = [0u8; 1];
		self.read_bytes(GT911_REG_BUFF_STATUS, &mut status).await?;
		let palm = (status[0] & 0x40) != 0;

		let num_points = min((status[0] & 0x0F) as usize, MAX_NUM_POINTS);

		// Clear previous points
		let mut points = TouchPoints::new();

		if num_points > 0 {
			let mut points_data = [0u8; MAX_NUM_POINTS * GT911_POINT_REG_SIZE];

			// Read num_points touch points (each point is 8 bytes)
			self.read_bytes(
				GT911_REG_POINTS,
				&mut points_data[..num_points * GT911_POINT_REG_SIZE],
			)
			.await?;

			for i in 0..num_points {
				let point_bytes =
					&points_data[i * GT911_POINT_REG_SIZE..(i + 1) * GT911_POINT_REG_SIZE];
				let point = TouchPoint::from_bytes(point_bytes.try_into().unwrap());
				points.push(point).unwrap();
			}
		}

		// Clear buffer status
		self.write_bytes(GT911_REG_BUFF_STATUS, &[0x00]).await?;

		Ok(TouchStatus { palm, points })
	}

	async fn read_bytes(&mut self, reg_addr: u16, buffer: &mut [u8]) -> Result<()> {
		let reg_addr = reg_addr.to_be_bytes();
		self.i2c
			.write_read(GT911_DEVICE_ADDR, &reg_addr, buffer)
			.await
			.map_err(|e| e.kind().into())
	}

	async fn write_bytes(&mut self, reg_addr: u16, data: &[u8]) -> Result<()> {
		let reg_addr = reg_addr.to_be_bytes();
		let mut ops = [Operation::Write(&reg_addr), Operation::Write(data)];
		self.i2c
			.transaction(GT911_DEVICE_ADDR, &mut ops)
			.await
			.map_err(|e| e.kind().into())
	}
}
