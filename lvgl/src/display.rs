use core::ptr::NonNull;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::{Error, Result};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum DisplayRotation {
	Rotation0 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_0,
	Rotation90 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_90,
	Rotation180 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_180,
	Rotation270 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_270,
}

pub trait LvDisplay: Sized {
	fn _display(&self) -> NonNull<lvgl_sys::lv_disp_t>;

	/// # Safety
	/// `disp` must be a valid pointer to an LVGL display object.
	unsafe fn _from_raw(disp: *mut lvgl_sys::lv_disp_t) -> Self;

	/// Creates a new display with the specified horizontal and vertical resolution.
	/// This should only be used if you need to create a display manually. In most cases, you should
	/// use `active_screen` to get the default display.
	fn _create(hor_res: i32, ver_res: i32) -> Result<Self> {
		let disp = unsafe { lvgl_sys::lv_display_create(hor_res, ver_res) };
		if disp.is_null() {
			Err(Error::DisplayCreateFailed)
		} else {
			// SAFETY: We just checked that `disp` is not null, so it's safe to use it.
			unsafe { Ok(Self::_from_raw(disp)) }
		}
	}

	// TODO This modifies global state so something should be mutable
	fn set_default(&self) {
		unsafe { lvgl_sys::lv_display_set_default(self._display().as_ptr()) };
	}

	fn default() -> Option<Self> {
		let disp = unsafe { lvgl_sys::lv_display_get_default() };
		if disp.is_null() {
			None
		} else {
			Some(unsafe { Self::_from_raw(disp) })
		}
	}

	/// Get the next display.
	fn next(&self) -> Option<Self> {
		let disp = unsafe { lvgl_sys::lv_display_get_next(self._display().as_ptr()) };
		if disp.is_null() {
			None
		} else {
			Some(unsafe { Self::_from_raw(disp) })
		}
	}

	fn set_resolution(&mut self, hor_res: i32, ver_res: i32) {
		unsafe { lvgl_sys::lv_display_set_resolution(self._display().as_ptr(), hor_res, ver_res) };
	}

	fn set_physical_resolution(&mut self, hor_res: i32, ver_res: i32) {
		unsafe {
			lvgl_sys::lv_display_set_physical_resolution(self._display().as_ptr(), hor_res, ver_res)
		};
	}

	fn set_offset(&mut self, x: i32, y: i32) {
		unsafe { lvgl_sys::lv_display_set_offset(self._display().as_ptr(), x, y) };
	}

	fn set_rotation(&mut self, rotation: DisplayRotation) {
		unsafe { lvgl_sys::lv_display_set_rotation(self._display().as_ptr(), rotation.into()) };
	}

	fn set_matrix_rotation(&mut self, enable: bool) {
		unsafe { lvgl_sys::lv_display_set_matrix_rotation(self._display().as_ptr(), enable) };
	}

	fn set_dpi(&mut self, dpi: i32) {
		unsafe { lvgl_sys::lv_display_set_dpi(self._display().as_ptr(), dpi) };
	}

	fn horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_horizontal_resolution(self._display().as_ptr()) }
	}

	fn vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_vertical_resolution(self._display().as_ptr()) }
	}

	fn original_horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_original_horizontal_resolution(self._display().as_ptr()) }
	}

	fn original_vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_original_vertical_resolution(self._display().as_ptr()) }
	}

	fn physical_horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_physical_horizontal_resolution(self._display().as_ptr()) }
	}

	fn physical_vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_physical_vertical_resolution(self._display().as_ptr()) }
	}

	fn offset_x(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_offset_x(self._display().as_ptr()) }
	}

	fn offset_y(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_offset_y(self._display().as_ptr()) }
	}

	fn rotation(&self) -> DisplayRotation {
		unsafe {
			lvgl_sys::lv_display_get_rotation(self._display().as_ptr())
				.try_into()
				.expect("Received invalid display rotation from LVGL")
		}
	}

	fn matrix_rotation(&self) -> bool {
		unsafe { lvgl_sys::lv_display_get_matrix_rotation(self._display().as_ptr()) }
	}

	fn dpi(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_dpi(self._display().as_ptr()) }
	}
}

#[cfg(feature = "sdl")]
pub struct SdlDisplay {
	disp: NonNull<lvgl_sys::lv_disp_t>,
}

#[cfg(feature = "sdl")]
impl SdlDisplay {
	pub fn new(width: i32, height: i32) -> Result<Self> {
		if width <= 0 || height <= 0 {
			return Err(Error::InvalidDisplaySize);
		}

		let disp = unsafe { lvgl_sys::lv_sdl_window_create(width, height) };
		let disp = NonNull::new(disp).ok_or(Error::DisplayCreateFailed)?;
		Ok(Self { disp })
	}
}

#[cfg(feature = "sdl")]
impl LvDisplay for SdlDisplay {
	#[inline(always)]
	fn _display(&self) -> NonNull<lvgl_sys::lv_disp_t> {
		self.disp
	}

	#[inline(always)]
	unsafe fn _from_raw(disp: *mut lvgl_sys::lv_disp_t) -> Self {
		Self {
			disp: NonNull::new(disp).expect("Received null pointer from LVGL"),
		}
	}
}
