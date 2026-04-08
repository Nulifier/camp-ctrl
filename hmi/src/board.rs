use embassy_rp::{self as hal, Peri, gpio, peripherals};

pub struct Board {
	pub i2c: I2cPins,
	pub display: DisplayPins,
	pub touch: TouchPins,
}

pub struct I2cPins {
	pub sda: Peri<'static, peripherals::PIN_6>,
	pub scl: Peri<'static, peripherals::PIN_7>,
	pub i2c: Peri<'static, peripherals::I2C1>,
}

pub struct DisplayPins {
	pub lcd_rst: Peri<'static, peripherals::PIN_41>,
	pub lcd_de: Peri<'static, peripherals::PIN_20>,
	pub lcd_vsync: Peri<'static, peripherals::PIN_21>,
	pub lcd_hsync: Peri<'static, peripherals::PIN_22>,
	pub lcd_pclk: Peri<'static, peripherals::PIN_23>,

	pub lcd_b3: Peri<'static, peripherals::PIN_24>,
	pub lcd_b4: Peri<'static, peripherals::PIN_25>,
	pub lcd_b5: Peri<'static, peripherals::PIN_26>,
	pub lcd_b6: Peri<'static, peripherals::PIN_27>,
	pub lcd_b7: Peri<'static, peripherals::PIN_28>,

	pub lcd_g2: Peri<'static, peripherals::PIN_29>,
	pub lcd_g3: Peri<'static, peripherals::PIN_30>,
	pub lcd_g4: Peri<'static, peripherals::PIN_31>,
	pub lcd_g5: Peri<'static, peripherals::PIN_32>,
	pub lcd_g6: Peri<'static, peripherals::PIN_33>,
	pub lcd_g7: Peri<'static, peripherals::PIN_34>,

	pub lcd_r3: Peri<'static, gpio::AnyPin>,
	pub lcd_r4: Peri<'static, gpio::AnyPin>,
	pub lcd_r5: Peri<'static, gpio::AnyPin>,
	pub lcd_r6: Peri<'static, gpio::AnyPin>,
	pub lcd_r7: Peri<'static, gpio::AnyPin>,

	/// LCD backlight enable pin
	pub lcd_en: Peri<'static, gpio::AnyPin>,
	/// LCD backlight PWM dimming pin
	pub lcd_bl: Peri<'static, gpio::AnyPin>,
}

pub struct TouchPins {
	pub tp_rst: Peri<'static, peripherals::PIN_19>,
	pub tp_int: Peri<'static, peripherals::PIN_18>,
}

pub struct UsbPins {
	pub usb_dp: Peri<'static, gpio::AnyPin>,
	pub usb_dm: Peri<'static, gpio::AnyPin>,
}

pub fn init() -> Board {
	let p = hal::init(Default::default());

	Board {
		i2c: I2cPins {
			sda: p.PIN_6,
			scl: p.PIN_7,
			i2c: p.I2C1,
		},
		display: DisplayPins {
			lcd_rst: p.PIN_41.into(),
			lcd_de: p.PIN_20.into(),
			lcd_vsync: p.PIN_21.into(),
			lcd_hsync: p.PIN_22.into(),
			lcd_pclk: p.PIN_23.into(),

			lcd_b3: p.PIN_24,
			lcd_b4: p.PIN_25,
			lcd_b5: p.PIN_26.into(),
			lcd_b6: p.PIN_27.into(),
			lcd_b7: p.PIN_28.into(),

			lcd_g2: p.PIN_29.into(),
			lcd_g3: p.PIN_30.into(),
			lcd_g4: p.PIN_31.into(),
			lcd_g5: p.PIN_32.into(),
			lcd_g6: p.PIN_33.into(),
			lcd_g7: p.PIN_34.into(),

			lcd_r3: p.PIN_35.into(),
			lcd_r4: p.PIN_36.into(),
			lcd_r5: p.PIN_37.into(),
			lcd_r6: p.PIN_38.into(),
			lcd_r7: p.PIN_39.into(),

			lcd_en: p.PIN_44.into(),
			lcd_bl: p.PIN_45.into(),
		},
		touch: TouchPins {
			tp_rst: p.PIN_19.into(),
			tp_int: p.PIN_18.into(),
		},
	}
}
