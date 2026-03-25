use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct Obj {
	raw: NonNull<lvgl_sys::lv_obj_t>,
}

pub struct ObjRef<'a> {
	raw: NonNull<lvgl_sys::lv_obj_t>,
	_marker: PhantomData<&'a ()>,
}

pub trait AsRawObj {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t>;

	fn as_raw_ptr(&self) -> *mut lvgl_sys::lv_obj_t {
		self.as_raw().as_ptr()
	}
}

impl Obj {
	pub(crate) fn from_raw(obj: *mut lvgl_sys::lv_obj_t) -> Option<Self> {
		Some(Self {
			raw: NonNull::new(obj).expect("Received null pointer from LVGL"),
		})
	}

	pub fn as_ref(&self) -> ObjRef<'_> {
		ObjRef {
			raw: self.raw,
			_marker: PhantomData,
		}
	}

	pub fn delete(self) {
		unsafe { lvgl_sys::lv_obj_delete(self.raw.as_ptr()) };
		core::mem::forget(self);
	}

	pub fn new(parent: &impl AsRawObj) -> Self {
		let raw = unsafe { lvgl_sys::lv_obj_create(parent.as_raw_ptr()) };
		Self::from_raw(raw).expect("Failed to create object")
	}
}

impl<'a> ObjRef<'a> {
	pub(crate) fn from_raw(obj: *mut lvgl_sys::lv_obj_t) -> Option<Self> {
		Some(Self {
			raw: NonNull::new(obj).expect("Received null pointer from LVGL"),
			_marker: PhantomData,
		})
	}
}

impl AsRawObj for Obj {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
		self.raw
	}
}

impl<'a> AsRawObj for ObjRef<'a> {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
		self.raw
	}
}
