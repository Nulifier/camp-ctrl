use crate::drivers::display::{FRAME_PIXELS, get_frame_buffer_ptrs, request_frame_buffer_swap};
use defmt::info;
use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use lvgl::{display::DoubleBufferedDisplay, misc::area::Area};

pub type Display = DoubleBufferedDisplay<u16>;

fn flush_display(_area: &Area, _pixels: &[u16]) {
	info!("Flushing display");
	request_frame_buffer_swap();
}

fn flush_wait_cb() {
	info!("Waiting for display flush to complete");
}

pub async fn create_rp_display() -> Display {
	let frame_buffers = get_frame_buffer_ptrs().await;

	let display = unsafe {
		DoubleBufferedDisplay::new(
			DISPLAY_WIDTH as usize,
			DISPLAY_HEIGHT as usize,
			frame_buffers.fb1, // Buffers are swapped so we draw to the back buffer
			frame_buffers.fb0,
			FRAME_PIXELS,
			flush_display,
			Some(flush_wait_cb),
		)
	};

	display
}
