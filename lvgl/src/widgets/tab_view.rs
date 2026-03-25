use crate::{
	error::Result,
	misc::area::Dir,
	widgets::{
		base::Widget,
		obj::{AsRawObj, Obj, ObjRef},
	},
};
use core::{ffi::CStr, ptr::NonNull};

pub struct TabView {
	obj: Obj,
}

impl TabView {
	pub fn new(parent: &impl AsRawObj) -> Self {
		let raw = unsafe { lvgl_sys::lv_tabview_create(parent.as_raw_ptr()) };
		Self {
			obj: Obj::from_raw(raw).expect("Failed to create tab view"),
		}
	}

	pub fn add_tab(&mut self, name: &CStr) -> Obj {
		let raw = unsafe { lvgl_sys::lv_tabview_add_tab(self.obj.as_raw_ptr(), name.as_ptr()) };
		Obj::from_raw(raw).expect("Failed to add tab")
	}

	pub fn set_tab_text(&mut self, index: u32, text: &CStr) -> Result<()> {
		if index >= self.get_tab_count() {
			return Err(crate::error::Error::IndexOutOfBounds);
		}
		unsafe { lvgl_sys::lv_tabview_set_tab_text(self.obj.as_raw_ptr(), index, text.as_ptr()) };
		Ok(())
	}

	pub fn set_active_tab(&mut self, index: u32, animate: bool) -> Result<()> {
		if index >= self.get_tab_count() {
			return Err(crate::error::Error::IndexOutOfBounds);
		}
		unsafe { lvgl_sys::lv_tabview_set_active(self.obj.as_raw_ptr(), index, animate) };
		Ok(())
	}

	pub fn set_tab_bar_position(&mut self, dir: Dir) {
		unsafe { lvgl_sys::lv_tabview_set_tab_bar_position(self.obj.as_raw_ptr(), dir.into()) };
	}

	pub fn set_tab_bar_size(&mut self, size: i32) {
		unsafe { lvgl_sys::lv_tabview_set_tab_bar_size(self.obj.as_raw_ptr(), size) };
	}

	pub fn get_tab_count(&self) -> u32 {
		unsafe { lvgl_sys::lv_tabview_get_tab_count(self.obj.as_raw_ptr()) }
	}

	pub fn get_active_tab(&self) -> u32 {
		unsafe { lvgl_sys::lv_tabview_get_tab_active(self.obj.as_raw_ptr()) }
	}

	pub fn get_tab_button(&self, index: u32) -> Option<ObjRef<'_>> {
		if index >= self.get_tab_count() {
			return None;
		}
		let raw =
			unsafe { lvgl_sys::lv_tabview_get_tab_button(self.obj.as_raw_ptr(), index as i32) };
		ObjRef::from_raw(raw)
	}

	pub fn get_content(&self) -> Option<ObjRef<'_>> {
		let raw = unsafe { lvgl_sys::lv_tabview_get_content(self.obj.as_raw_ptr()) };
		ObjRef::from_raw(raw)
	}

	pub fn get_tab_bar(&self) -> Option<ObjRef<'_>> {
		let raw = unsafe { lvgl_sys::lv_tabview_get_tab_bar(self.obj.as_raw_ptr()) };
		ObjRef::from_raw(raw)
	}

	pub fn tab_bar_position(&self) -> Dir {
		let pos = unsafe { lvgl_sys::lv_tabview_get_tab_bar_position(self.obj.as_raw_ptr()) };
		pos.try_into().expect("Unknown tab bar position value")
	}
}

impl AsRawObj for TabView {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
		self.obj.as_raw()
	}
}

impl Widget for TabView {}
