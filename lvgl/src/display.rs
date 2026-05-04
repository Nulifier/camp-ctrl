extern crate alloc;

use alloc::boxed::Box;
use core::ptr::NonNull;
use defmt::debug;

use crate::{
	AsRaw, AsRawMut,
	error::{Error, Result},
	misc::{area::Area, color::Color16},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum DisplayRotation {
	Rotation0 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_0,
	Rotation90 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_90,
	Rotation180 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_180,
	Rotation270 = lvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_270,
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum DisplayRenderMode {
	Partial = lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
	Direct = lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_DIRECT,
	Full = lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
}

pub trait LvDisplay: Sized + AsRaw<lvgl_sys::lv_disp_t> + AsRawMut<lvgl_sys::lv_disp_t> {
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

	fn set_default(&self) {
		unsafe { lvgl_sys::lv_display_set_default(self.as_raw() as *mut lvgl_sys::lv_disp_t) };
	}

	fn default() -> Option<Self> {
		let disp = unsafe { lvgl_sys::lv_display_get_default() };
		if disp.is_null() {
			None
		} else {
			Some(unsafe { Self::_from_raw(disp) })
		}
	}

	fn refresh_now(&mut self) {
		unsafe { lvgl_sys::lv_refr_now(self.as_raw_mut()) };
	}

	fn flush_ready(&mut self) {
		unsafe { lvgl_sys::lv_display_flush_ready(self.as_raw_mut()) };
	}

	fn set_resolution(&mut self, hor_res: i32, ver_res: i32) {
		unsafe { lvgl_sys::lv_display_set_resolution(self.as_raw_mut(), hor_res, ver_res) };
	}

	fn set_physical_resolution(&mut self, hor_res: i32, ver_res: i32) {
		unsafe {
			lvgl_sys::lv_display_set_physical_resolution(self.as_raw_mut(), hor_res, ver_res)
		};
	}

	fn set_offset(&mut self, x: i32, y: i32) {
		unsafe { lvgl_sys::lv_display_set_offset(self.as_raw_mut(), x, y) };
	}

	fn set_rotation(&mut self, rotation: DisplayRotation) {
		unsafe { lvgl_sys::lv_display_set_rotation(self.as_raw_mut(), rotation.into()) };
	}

	fn set_matrix_rotation(&mut self, enable: bool) {
		unsafe { lvgl_sys::lv_display_set_matrix_rotation(self.as_raw_mut(), enable) };
	}

	fn set_dpi(&mut self, dpi: i32) {
		unsafe { lvgl_sys::lv_display_set_dpi(self.as_raw_mut(), dpi) };
	}

	fn horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_horizontal_resolution(self.as_raw()) }
	}

	fn vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_vertical_resolution(self.as_raw()) }
	}

	fn original_horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_original_horizontal_resolution(self.as_raw()) }
	}

	fn original_vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_original_vertical_resolution(self.as_raw()) }
	}

	fn physical_horizontal_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_physical_horizontal_resolution(self.as_raw()) }
	}

	fn physical_vertical_resolution(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_physical_vertical_resolution(self.as_raw()) }
	}

	fn offset_x(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_offset_x(self.as_raw()) }
	}

	fn offset_y(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_offset_y(self.as_raw()) }
	}

	fn rotation(&self) -> DisplayRotation {
		unsafe {
			lvgl_sys::lv_display_get_rotation(self.as_raw() as *mut lvgl_sys::lv_disp_t)
				.try_into()
				.expect("Received invalid display rotation from LVGL")
		}
	}

	fn matrix_rotation(&self) -> bool {
		unsafe {
			lvgl_sys::lv_display_get_matrix_rotation(self.as_raw() as *mut lvgl_sys::lv_disp_t)
		}
	}

	fn dpi(&self) -> i32 {
		unsafe { lvgl_sys::lv_display_get_dpi(self.as_raw()) }
	}

	fn set_buffers(
		&mut self,
		buf0: &mut [u8],
		buf1: Option<&mut [u8]>,
		render_mode: DisplayRenderMode,
	) {
		// Check if buffers are the same size (if buf1 is provided)
		if let Some(buf1) = &buf1 {
			if buf0.len() != buf1.len() {
				panic!("Both display buffers must be the same size");
			}
		}

		let size = buf0.len() as u32;

		unsafe {
			lvgl_sys::lv_display_set_buffers(
				self.as_raw_mut(),
				buf0.as_mut_ptr() as *mut core::ffi::c_void,
				buf1.map(|b| b.as_mut_ptr() as *mut core::ffi::c_void)
					.unwrap_or(core::ptr::null_mut()),
				size,
				render_mode.into(),
			);
		}
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
impl AsRaw<lvgl_sys::lv_disp_t> for SdlDisplay {
	#[inline(always)]
	fn as_raw(&self) -> *const lvgl_sys::lv_disp_t {
		self.disp.as_ptr()
	}
}

#[cfg(feature = "sdl")]
impl AsRawMut<lvgl_sys::lv_disp_t> for SdlDisplay {
	#[inline(always)]
	fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_disp_t {
		self.disp.as_ptr()
	}
}

#[cfg(feature = "sdl")]
impl LvDisplay for SdlDisplay {
	#[inline(always)]
	unsafe fn _from_raw(disp: *mut lvgl_sys::lv_disp_t) -> Self {
		Self {
			disp: NonNull::new(disp).expect("Received null pointer from LVGL"),
		}
	}
}

pub type FlushCallback<C> = fn(area: &Area, pixels: &[C]);
pub type FlushWaitCallback = fn();

struct DoubleBufferedDisplayCallbacks<C> {
	len: usize,
	flush_cb: FlushCallback<C>,
	flush_wait_cb: Option<FlushWaitCallback>,
}

pub struct DoubleBufferedDisplay<C = Color16> {
	display: NonNull<lvgl_sys::lv_disp_t>,
	buf0: *mut C,
	buf1: *mut C,
}

impl<C> DoubleBufferedDisplay<C> {
	/// Creates a new double buffered display with the specified width, height, buffers, and flush
	/// callbacks.
	/// # Safety
	/// `buf0` and `buf1` must be valid pointers to buffers that can hold at least `buf_size` pixels
	/// of type `C`. The caller must ensure that these buffers remain valid for the lifetime of the
	/// display, and that they are not modified by anything other than the display's flush callbacks
	/// while the display is active.
	pub unsafe fn new(
		width: usize,
		height: usize,
		buf0: *mut C,
		buf1: *mut C,
		buf_size_pixels: usize,
		flush_cb: FlushCallback<C>,
		flush_wait_cb: Option<FlushWaitCallback>,
	) -> Self {
		let display = unsafe { lvgl_sys::lv_display_create(width as i32, height as i32) };
		let buf_size_bytes = buf_size_pixels
			.checked_mul(core::mem::size_of::<C>())
			.and_then(|n| u32::try_from(n).ok())
			.expect("Display buffer size does not fit in u32 bytes");

		if buf0.is_null() || buf1.is_null() {
			panic!("Both display buffers must be non-null");
		}

		let disp = Self {
			display: NonNull::new(display).expect("Failed to create LVGL display"),
			buf0,
			buf1,
		};

		let callbacks = Box::new(DoubleBufferedDisplayCallbacks {
			len: buf_size_pixels,
			flush_cb,
			flush_wait_cb,
		});

		unsafe {
			lvgl_sys::lv_display_set_buffers(
				display,
				disp.buf0 as *mut core::ffi::c_void,
				disp.buf1 as *mut core::ffi::c_void,
				buf_size_bytes,
				lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
			);

			lvgl_sys::lv_display_set_user_data(display, Box::into_raw(callbacks).cast());
			lvgl_sys::lv_display_set_flush_cb(display, Some(flush_trampoline::<C>));
			lvgl_sys::lv_display_set_flush_wait_cb(display, Some(flush_wait_trampoline::<C>));
		}

		disp
	}
}

unsafe extern "C" fn flush_trampoline<C>(
	disp: *mut lvgl_sys::lv_display_t,
	area: *const lvgl_sys::lv_area_t,
	px_map: *mut u8,
) {
	let callbacks_ptr = unsafe { lvgl_sys::lv_display_get_user_data(disp) }
		as *const DoubleBufferedDisplayCallbacks<C>;

	if callbacks_ptr.is_null() {
		// If no callbacks are set, just signal that the flush is ready and return
		// This should not happen in normal operation, but it's better than crashing if it does
		unsafe { lvgl_sys::lv_display_flush_ready(disp) };
		return;
	}

	let callbacks = unsafe { &*callbacks_ptr };
	let active_ptr = px_map.cast_const().cast::<C>();

	let area = Area::from_raw(area);

	let pixels_size = area.width() as usize * area.height() as usize;
	assert!(
		pixels_size <= callbacks.len,
		"LVGL is drawing more pixels than the buffer can hold"
	);

	let pixels = unsafe { core::slice::from_raw_parts(active_ptr, pixels_size) };

	(callbacks.flush_cb)(&area, pixels);
}

unsafe extern "C" fn flush_wait_trampoline<C>(disp: *mut lvgl_sys::lv_display_t) {
	debug!("Flush wait trampoline called");

	let callbacks_ptr = unsafe { lvgl_sys::lv_display_get_user_data(disp) }
		as *const DoubleBufferedDisplayCallbacks<C>;

	if callbacks_ptr.is_null() {
		return;
	}

	let callbacks = unsafe { &*callbacks_ptr };
	if let Some(wait_cb) = callbacks.flush_wait_cb {
		wait_cb();
	}
}

impl<C> LvDisplay for DoubleBufferedDisplay<C> {
	#[inline(always)]
	unsafe fn _from_raw(_disp: *mut lvgl_sys::lv_disp_t) -> Self {
		unimplemented!("DoubleBufferedDisplay cannot be created from a raw pointer")
	}
}

impl<C> AsRaw<lvgl_sys::lv_disp_t> for DoubleBufferedDisplay<C> {
	#[inline(always)]
	fn as_raw(&self) -> *const lvgl_sys::lv_disp_t {
		self.display.as_ptr()
	}
}

impl<C> AsRawMut<lvgl_sys::lv_disp_t> for DoubleBufferedDisplay<C> {
	#[inline(always)]
	fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_disp_t {
		self.display.as_ptr()
	}
}

impl<C> Drop for DoubleBufferedDisplay<C> {
	fn drop(&mut self) {
		unsafe {
			let user_data = lvgl_sys::lv_display_get_user_data(self.display.as_ptr());
			if !user_data.is_null() {
				drop(Box::from_raw(
					user_data.cast::<DoubleBufferedDisplayCallbacks<C>>(),
				));
				lvgl_sys::lv_display_set_user_data(self.display.as_ptr(), core::ptr::null_mut());
			}

			lvgl_sys::lv_display_set_flush_cb(self.display.as_ptr(), None);
			lvgl_sys::lv_display_set_flush_wait_cb(self.display.as_ptr(), None);
			lvgl_sys::lv_display_delete(self.display.as_ptr());
		}
	}
}
