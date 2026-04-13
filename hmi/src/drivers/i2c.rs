use crate::board::I2c1Resources;
use defmt::info;
use embassy_rp::{
	bind_interrupts,
	i2c::{Async, Config as I2cConfig, I2c},
	peripherals,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use static_cell::StaticCell;

bind_interrupts!(
	/// Binds the i2c Interrupts.
	struct Irqs {
		// I2C0_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C0>;
		I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
	}
);

pub type I2cDriver = I2c<'static, peripherals::I2C1, Async>;
pub type I2cBus = Mutex<CriticalSectionRawMutex, I2cDriver>;
pub fn init(
	// i2c: Peri<'static, peripherals::I2C1>,
	// scl: Peri<'static, peripherals::PIN_7>,
	// sda: Peri<'static, peripherals::PIN_6>,
	r: I2c1Resources,
) -> &'static I2cBus {
	info!("Initializing I2C driver");
	static BUS: StaticCell<I2cBus> = StaticCell::new();
	BUS.init(Mutex::new(I2c::new_async(
		r.i2c, // I2C1 peripheral
		r.scl, // SCL pin
		r.sda, // SDA pin
		Irqs,
		I2cConfig::default(),
	)))
}
