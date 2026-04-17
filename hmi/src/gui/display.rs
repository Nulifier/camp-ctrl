use crate::utils::psram::PsramBuffer;
use embassy_time::{Duration, Timer};
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use lvgl::{display::DoubleBufferedDisplay, misc::color::Color16};

const DISPLAY_BUF_SIZE: usize = DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize;

type DisplayColor = Color16;
type DisplayBuffer = PsramBuffer<DisplayColor, DISPLAY_BUF_SIZE>;
pub type Display = DoubleBufferedDisplay<DisplayColor>;

// pub async fn flush_display()

pub fn create_rp_display() -> Display {
	let mut buf0: DisplayBuffer = PsramBuffer::new().unwrap();
	let mut buf1: DisplayBuffer = PsramBuffer::new().unwrap();

	let display = unsafe {
		DoubleBufferedDisplay::new(
			DISPLAY_WIDTH as usize,
			DISPLAY_HEIGHT as usize,
			buf0.as_mut_ptr().cast(),
			buf1.as_mut_ptr().cast(),
			DISPLAY_BUF_SIZE,
			|_, _| {}, // Dummy flush callback
			Some(|| {
				let _future = Timer::after(Duration::from_millis(10));

				// future.await;
			}),
		)
	};

	buf0.as_mut().fill(Color16::from_rgb8(0, 0, 0));

	display
}
