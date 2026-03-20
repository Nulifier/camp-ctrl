mod ffi;

use ffi::UiSnapshot;

#[unsafe(no_mangle)]
pub extern "C" fn rust_on_reset_button() {
	println!("Reset button pressed!");
}

fn main() {
	unsafe { ffi::gui_init() };
}
