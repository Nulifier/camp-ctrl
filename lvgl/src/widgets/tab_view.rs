use crate::{
	error::Result,
	misc::area::Dir,
	widgets::{
		base::Widget,
		obj::{AsRawObj, Obj, ObjRef, WidgetClassMarker},
	},
};
use core::{ffi::CStr, ptr::NonNull};

#[repr(transparent)]
pub struct TabView {
	obj: Obj,
}

impl TabView {
	pub fn new(parent: &impl AsRawObj) -> Self {
		let raw = unsafe { lvgl_sys::lv_tabview_create(parent.as_raw_ptr()) };
		Self {
			obj: Obj::from_raw(raw),
		}
	}

	pub fn add_tab(&mut self, name: &CStr) -> Obj {
		let raw = unsafe { lvgl_sys::lv_tabview_add_tab(self.obj.as_raw_ptr(), name.as_ptr()) };
		Obj::from_raw(raw)
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

	pub fn set_tab_bar_position(&mut self, dir: Dir) -> &mut Self {
		unsafe { lvgl_sys::lv_tabview_set_tab_bar_position(self.obj.as_raw_ptr(), dir.into()) };
		self
	}

	pub fn set_tab_bar_size(&mut self, size: i32) -> &mut Self {
		unsafe { lvgl_sys::lv_tabview_set_tab_bar_size(self.obj.as_raw_ptr(), size) };
		self
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
		Some(ObjRef::from_raw(raw))
	}

	pub fn get_content(&self) -> ObjRef<'_> {
		let raw = unsafe { lvgl_sys::lv_tabview_get_content(self.obj.as_raw_ptr()) };
		ObjRef::from_raw(raw)
	}

	pub fn get_tab_bar(&self) -> ObjRef<'_> {
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

impl<'a> Widget<'a> for TabView {}

impl WidgetClassMarker for TabView {
	fn class_ptr() -> *const lvgl_sys::lv_obj_class_t {
		unsafe { &lvgl_sys::lv_tabview_class as *const _ }
	}
}
