use crate::utils::psram::PsramBuffer;
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use lvgl::misc::color::Color16;

const DISPLAY_BUF_SIZE: usize = DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize;

pub struct RpDisplay {
	buf0: PsramBuffer<Color16, DISPLAY_BUF_SIZE>,
	buf1: PsramBuffer<Color16, DISPLAY_BUF_SIZE>,
}

impl RpDisplay {
	pub fn new() -> Self {
		RpDisplay {
			//buf0: [0; DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize],
			buf0: PsramBuffer::new().unwrap(),
			buf1: PsramBuffer::new().unwrap(),
		}
	}
}
