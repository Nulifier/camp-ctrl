use assign_resources::assign_resources;
use embassy_rp::{Peri, peripherals};

assign_resources! {
	psram: PsramResources {
			qmi_cs1: QMI_CS1,
			cs: PIN_0
	}
	i2c1: I2c1Resources {
		sda: PIN_6,
		scl: PIN_7,
		i2c: I2C1,
	}
	display_ctrl: DisplayCtrlResources {
		lcd_rst: PIN_41,
	},
	display_timing: DisplayTimingResources {
		// lcd_de: PIN_20,
		lcd_vsync: PIN_21,
		lcd_hsync: PIN_22,
		lcd_pclk: PIN_23,

		// lcd_b3: PIN_24,
		// lcd_b4: PIN_25,
		// lcd_b5: PIN_26,
		// lcd_b6: PIN_27,
		// lcd_b7: PIN_28,

		// lcd_g2: PIN_29,
		// lcd_g3: PIN_30,
		// lcd_g4: PIN_31,
		// lcd_g5: PIN_32,
		// lcd_g6: PIN_33,
		// lcd_g7: PIN_34,

		// lcd_r3: PIN_35,
		// lcd_r4: PIN_36,
		// lcd_r5: PIN_37,
		// lcd_r6: PIN_38,
		// lcd_r7: PIN_39,

		pio_timing: PIO0,
		// pio_rgb: PIO1,
	}
	display_data: DisplayDataResources {
		lcd_de: PIN_20,

		lcd_b3: PIN_24,
		lcd_b4: PIN_25,
		lcd_b5: PIN_26,
		lcd_b6: PIN_27,
		lcd_b7: PIN_28,

		lcd_g2: PIN_29,
		lcd_g3: PIN_30,
		lcd_g4: PIN_31,
		lcd_g5: PIN_32,
		lcd_g6: PIN_33,
		lcd_g7: PIN_34,

		lcd_r3: PIN_35,
		lcd_r4: PIN_36,
		lcd_r5: PIN_37,
		lcd_r6: PIN_38,
		lcd_r7: PIN_39,

		pio_rgb: PIO1,
	}
	backlight: BacklightResources {
		lcd_en: PIN_45,
		lcd_bl: PIN_44,
		lcd_pwm: PWM_SLICE10,
	}
	touch: TouchResources {
		tp_rst: PIN_19,
		tp_int: PIN_18,
	}
}
