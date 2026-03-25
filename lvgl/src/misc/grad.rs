use crate::misc::color::{Color, Opacity};

#[repr(u32)]
pub enum GradDir {
	None = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_NONE,
	Ver = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_VER,
	Hor = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_HOR,
	Linear = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_LINEAR,
	Radial = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_RADIAL,
	Conical = lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_CONICAL,
}

#[repr(u32)]
pub enum GradExtend {
	Pad = lvgl_sys::lv_grad_extend_t_LV_GRAD_EXTEND_PAD,
	Repeat = lvgl_sys::lv_grad_extend_t_LV_GRAD_EXTEND_REPEAT,
	Reflect = lvgl_sys::lv_grad_extend_t_LV_GRAD_EXTEND_REFLECT,
}

#[allow(dead_code)]
pub struct GradStop(lvgl_sys::lv_grad_stop_t);

impl GradStop {
	// pub(crate) fn as_raw_ptr(&self) -> *const lvgl_sys::lv_grad_stop_t {
	// 	&self.0
	// }

	// pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_grad_stop_t {
	// 	&mut self.0
	// }

	pub fn new(color: Color, opa: Opacity, frac: u8) -> Self {
		Self(lvgl_sys::lv_grad_stop_t {
			color: color.as_raw(),
			opa: opa.into(),
			frac,
		})
	}
}

pub struct GradientDescriptor(lvgl_sys::lv_grad_dsc_t);

impl GradientDescriptor {
	pub(crate) fn as_raw_ptr(&self) -> *const lvgl_sys::lv_grad_dsc_t {
		&self.0
	}

	// pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_grad_dsc_t {
	// 	&mut self.0
	// }
}
