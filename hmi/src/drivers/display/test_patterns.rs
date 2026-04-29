use hmi_gui::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn primary_color_bars(x: usize) -> lvgl::misc::color::Color16 {
	match x {
		0..=99 => lvgl::misc::color::Color16::from_rgb8(255, 0, 0), // Red
		100..=199 => lvgl::misc::color::Color16::from_rgb8(255, 255, 0), // Yellow
		200..=299 => lvgl::misc::color::Color16::from_rgb8(0, 255, 0), // Green
		300..=399 => lvgl::misc::color::Color16::from_rgb8(0, 255, 255), // Cyan
		400..=499 => lvgl::misc::color::Color16::from_rgb8(0, 0, 255), // Blue
		500..=599 => lvgl::misc::color::Color16::from_rgb8(255, 0, 255), // Magenta
		600..=699 => lvgl::misc::color::Color16::from_rgb8(255, 255, 255), // White
		700..=799 => lvgl::misc::color::Color16::from_rgb8(255, 165, 0), // Orange
		_ => lvgl::misc::color::Color16::from_rgb8(0, 0, 0),        // Black
	}
}

fn grayscale_bars(x: usize) -> lvgl::misc::color::Color16 {
	let intensity = (x as u32 * 255 / (DISPLAY_WIDTH as u32 - 1)) as u8;
	lvgl::misc::color::Color16::from_rgb8(intensity, intensity, intensity)
}

fn color16_to_u16(color: lvgl::misc::color::Color16) -> u16 {
	unsafe { core::mem::transmute(color) }
}

#[allow(dead_code)]
pub(super) fn fill_chunk_with_test_pattern(buffer: &mut [u16]) {
	// Assert that the buffer is a multiple of the line size
	assert!(buffer.len() % DISPLAY_WIDTH as usize == 0);

	let num_lines = buffer.len() / DISPLAY_WIDTH as usize;

	// For each line in the buffer
	for line in 0..num_lines {
		for x in 0..DISPLAY_WIDTH as usize {
			let color = primary_color_bars(x);

			buffer[line * DISPLAY_WIDTH as usize + x] = color16_to_u16(color);
		}
	}
}

#[allow(dead_code)]
pub(super) fn fill_frame_buffers_with_test_pattern(buffer: &mut [u16]) {
	// Assert that the buffer is the size of the entire frame
	assert!(buffer.len() == DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize);

	for line in 0..DISPLAY_HEIGHT as usize {
		for x in 0..DISPLAY_WIDTH as usize {
			let color = match line {
				0..=299 => primary_color_bars(x),
				300..=399 => grayscale_bars(x),
				400..=479 => {
					// Checkerboard pattern
					if (line / 20 + x / 20) % 2 == 0 {
						lvgl::misc::color::Color16::from_rgb8(255, 255, 255) // White
					} else {
						lvgl::misc::color::Color16::from_rgb8(0, 0, 0) // Black
					}
				}
				_ => lvgl::misc::color::Color16::from_rgb8(0, 0, 0), // Black
			};

			buffer[line * DISPLAY_WIDTH as usize + x] = color16_to_u16(color);
		}
	}
}
