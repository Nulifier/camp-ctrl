#[cfg(feature = "sdl")]
use core::ptr::NonNull;

#[cfg(feature = "sdl")]
use crate::error::{Error, Result};

pub trait InputDevice {
	fn _input_device(&self) -> *mut lvgl_sys::lv_indev_t;
}

#[cfg(feature = "sdl")]
pub struct SdlMouse {
	indev: NonNull<lvgl_sys::lv_indev_t>,
}

#[cfg(feature = "sdl")]
impl SdlMouse {
	pub fn new() -> Result<Self> {
		let indev = unsafe { lvgl_sys::lv_sdl_mouse_create() };
		let indev = NonNull::new(indev).ok_or(Error::InputDeviceCreateFailed)?;
		Ok(Self { indev })
	}
}

#[cfg(feature = "sdl")]
impl InputDevice for SdlMouse {
	#[inline(always)]
	fn _input_device(&self) -> *mut lvgl_sys::lv_indev_t {
		self.indev.as_ptr()
	}
}

#[cfg(feature = "sdl")]
pub struct SdlKeyboard {
	indev: NonNull<lvgl_sys::lv_indev_t>,
}

#[cfg(feature = "sdl")]
impl SdlKeyboard {
	pub fn new() -> Result<Self> {
		let indev = unsafe { lvgl_sys::lv_sdl_keyboard_create() };
		let indev = NonNull::new(indev).ok_or(Error::InputDeviceCreateFailed)?;
		Ok(Self { indev })
	}
}

#[cfg(feature = "sdl")]
impl InputDevice for SdlKeyboard {
	#[inline(always)]
	fn _input_device(&self) -> *mut lvgl_sys::lv_indev_t {
		self.indev.as_ptr()
	}
}
