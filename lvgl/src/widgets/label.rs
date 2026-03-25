use crate::widgets::base::Widget;
use crate::widgets::obj::{AsRawObj, Obj};
use core::ffi::CStr;
use core::ptr::NonNull;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u32)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum LabelLongMode {
	Wrap = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_WRAP,
	Dots = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_DOTS,
	Scroll = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL,
	ScrollCircular = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL_CIRCULAR,
	Clip = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_CLIP,
}

pub struct Label {
	obj: Obj,
}

impl Label {
	pub fn new(parent: &impl AsRawObj) -> Self {
		let raw = unsafe { lvgl_sys::lv_label_create(parent.as_raw_ptr()) };
		Self {
			obj: Obj::from_raw(raw).expect("Failed to create label"),
		}
	}

	pub fn get_text(&self) -> &CStr {
		unsafe { CStr::from_ptr(lvgl_sys::lv_label_get_text(self.obj.as_raw_ptr())) }
	}

	pub fn set_text(&mut self, text: &CStr) {
		unsafe { lvgl_sys::lv_label_set_text(self.obj.as_raw_ptr(), text.as_ptr()) };
	}

	pub fn set_text_static(&mut self, text: &'static CStr) {
		unsafe { lvgl_sys::lv_label_set_text_static(self.obj.as_raw_ptr(), text.as_ptr()) };
	}

	pub fn get_long_mode(&self) -> LabelLongMode {
		let mode = unsafe { lvgl_sys::lv_label_get_long_mode(self.obj.as_raw_ptr()) };
		LabelLongMode::try_from(mode).expect("Unknown long mode value")
	}

	pub fn set_long_mode(&mut self, mode: LabelLongMode) {
		unsafe { lvgl_sys::lv_label_set_long_mode(self.obj.as_raw_ptr(), mode.into()) };
	}

	pub fn set_text_selection_start(&mut self, index: u32) {
		unsafe { lvgl_sys::lv_label_set_text_selection_start(self.obj.as_raw_ptr(), index) };
	}

	pub fn set_text_selection_end(&mut self, index: u32) {
		unsafe { lvgl_sys::lv_label_set_text_selection_end(self.obj.as_raw_ptr(), index) };
	}

	pub fn set_recolor(&mut self, recolor: bool) {
		unsafe { lvgl_sys::lv_label_set_recolor(self.obj.as_raw_ptr(), recolor) };
	}
}

impl AsRawObj for Label {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
		self.obj.as_raw()
	}
}

impl Widget for Label {}
