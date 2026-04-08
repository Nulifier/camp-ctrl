#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{AnyPin, Input, Output};
use embassy_rp::{self as hal, Peri};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Instant, Timer};

pub mod board;
pub mod drivers;
pub mod error;
pub mod tasks;
pub mod touch;

// Panic handler
use panic_probe as _;
// Defmt logging
use defmt_rtt as _;

use crate::drivers::i2c::{self};
use crate::drivers::touch::TouchDriver;

// Tell the boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::task]
async fn wake_screen(
	pin_tp_rst: Peri<'static, AnyPin>,
	pin_int: Peri<'static, AnyPin>,
	pin_lcd_bl: Peri<'static, AnyPin>,
	i2c_dev: I2cDevice<'static, CriticalSectionRawMutex, i2c::I2cDriver>,
) {
	// info!("Starting wake_screen task");

	// let mut touch_driver = TouchDriver::new(i2c_dev, pin_tp_rst, pin_int);

	// info!("Touch driver initialized");

	// // Turn on the touch sensor reset pin (active low)
	// let mut out_tp_rst = Output::new(pin_tp_rst, embassy_rp::gpio::Level::Low);
	// out_tp_rst.set_high();

	// // Wait for 500ms to allow the touch sensor to reset
	// Timer::after(Duration::from_millis(500)).await;

	// // When pin_int goes high, wake the LCD up by setting pin_lcd_bl high for 5 seconds, then low again.
	// let mut in_int = Input::new(pin_int, embassy_rp::gpio::Pull::None);
	// let mut out_lcd_bl = Output::new(pin_lcd_bl, embassy_rp::gpio::Level::Low);

	// // Read GT911 product ID for debugging
	// match touch_driver.read_product_id().await {
	// 	Ok(product_id) => {
	// 		info!(
	// 			"GT911 Product ID: {}{}{}{}",
	// 			product_id[0] as char,
	// 			product_id[1] as char,
	// 			product_id[2] as char,
	// 			product_id[3] as char
	// 		);
	// 	}
	// 	Err(_) => {
	// 		info!("Failed to read GT911 Product ID");
	// 	}
	// }

	// let mut last_touch_time = Instant::now();

	loop {
		// in_int.wait_for_falling_edge().await;
		// // info!("Pin INT went high, reading touch info");

		// touch_driver.read_points().await.unwrap();

		// // info!("There are {} touch points", touch_driver.point_count());

		// if touch_driver.point_count() > 0 {
		// 	info!("There are {} touch points", touch_driver.point_count());

		// 	let points = touch_driver.points();
		// 	for (i, point) in points.iter().enumerate() {
		// 		info!(
		// 			"Touch point {}: ID={}, x={}, y={}, size={}",
		// 			i, point.id, point.x, point.y, point.size
		// 		);
		// 	}

		// 	info!("Turning LCD backlight on");
		// 	out_lcd_bl.set_high();
		// 	last_touch_time = Instant::now();
		// } else {
		// 	if out_lcd_bl.get_output_level() == embassy_rp::gpio::Level::High {
		// 		// Check if its been 2 seconds since the last touch, and if so, turn off the backlight
		// 		if Instant::now() - last_touch_time > Duration::from_secs(2) {
		// 			info!("No touch points, turning LCD backlight off");
		// 			out_lcd_bl.set_low();
		// 		}
		// 	}
		// }
	}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = board::init();

	info!("Hello, world!");

	let _i2c = i2c::init(p.i2c.i2c, p.i2c.scl, p.i2c.sda);

	// let i2c = i2c::I2cDriver::new(
	// 	p.i2c.scl, // placeholder, real impl would pass in the I2C peripheral and pins
	// 	Peri::take().unwrap(),
	// 	Peri::take().unwrap(),
	// );

	// spawner
	// 	.spawn(wake_screen(
	// 		p.touch.tp_rst.into(),   // Touch sensor reset pin
	// 		p.touch.tp_int.into(),   // LCD interrupt pin
	// 		p.display.lcd_bl.into(), // LCD backlight control pin
	// 		i2c_dev_touch,
	// 	))
	// 	.unwrap();

	spawner
		.spawn(tasks::touch_task::touch_task(
			p.touch.tp_rst.into(),
			p.touch.tp_int.into(),
			I2cDevice::new(_i2c),
		))
		.unwrap();

	// Turn on GPIO45 (the LCD backlight)
	// let mut lcd_bl = Output::new(p.PIN_45, embassy_rp::gpio::Level::Low);
	// lcd_bl.set_low();

	loop {
		Timer::after_millis(100).await;
	}
}

// Program metadata for `picotool info`
// This isn't needed, but its recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
	embassy_rp::binary_info::rp_program_name!(c"hmi"),
	embassy_rp::binary_info::rp_program_description!(c"Camp Ctrl HMI"),
	embassy_rp::binary_info::rp_cargo_version!(),
	embassy_rp::binary_info::rp_program_build_attribute!(),
];
