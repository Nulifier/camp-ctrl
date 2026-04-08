use core::marker::PhantomData;
use core::ptr::NonNull;

#[repr(transparent)]
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

/// Marker trait for widget types that can be cast to
pub trait WidgetClassMarker: Sized {
	/// Returns a pointer to the LVGL class descriptor for this widget type
	fn class_ptr() -> *const lvgl_sys::lv_obj_class_t;
}

/// A reference to an LVGL object with a known widget type
pub struct TypedObjRef<'a, T: WidgetClassMarker> {
	raw: NonNull<lvgl_sys::lv_obj_t>,
	_marker: PhantomData<(&'a (), T)>,
}

impl<'a, T: WidgetClassMarker> TypedObjRef<'a, T> {
	/// Get the underlying raw LVGL object
	pub(crate) fn from_raw(raw: NonNull<lvgl_sys::lv_obj_t>) -> Self {
		Self {
			raw,
			_marker: PhantomData,
		}
	}
}

impl<'a, T: WidgetClassMarker> AsRawObj for TypedObjRef<'a, T> {
	fn as_raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
		self.raw
	}
}

impl<'a, T: WidgetClassMarker> core::ops::Deref for TypedObjRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		// SAFETY: T is repr(transparent) over Obj which is repr(transparent) over
		// NonNull<lv_obj_t>. TypedObjRef is only constructed after lv_obj_check_type
		// confirms the underlying object is of type T.
		unsafe { &*(core::ptr::addr_of!(self.raw) as *const T) }
	}
}

impl Obj {
	pub(crate) fn from_raw(obj: *mut lvgl_sys::lv_obj_t) -> Self {
		Self {
			raw: NonNull::new(obj).expect("Received null pointer from LVGL"),
		}
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
		Self::from_raw(raw)
	}

	/// Try to cast this object to a specific widget type
	///
	/// # Example
	/// ```ignore
	/// let obj: Obj = /* ... */;
	/// if let Some(label) = obj.try_cast::<Label>() {
	///     // obj is a Label
	/// }
	/// ```
	pub fn try_cast<T: WidgetClassMarker>(&self) -> Option<TypedObjRef<'_, T>> {
		if unsafe { lvgl_sys::lv_obj_check_type(self.as_raw_ptr(), T::class_ptr()) } {
			Some(TypedObjRef::from_raw(self.raw))
		} else {
			None
		}
	}

	/// Check if this object is of a specific widget type
	pub fn is<T: WidgetClassMarker>(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_check_type(self.as_raw_ptr(), T::class_ptr()) }
	}
}

impl<'a> ObjRef<'a> {
	pub(crate) fn from_raw(obj: *mut lvgl_sys::lv_obj_t) -> Self {
		Self {
			raw: NonNull::new(obj).expect("Received null pointer from LVGL"),
			_marker: PhantomData,
		}
	}

	/// Try to cast this reference to a specific widget type
	pub fn try_cast<T: WidgetClassMarker>(&self) -> Option<TypedObjRef<'a, T>> {
		if unsafe { lvgl_sys::lv_obj_check_type(self.as_raw_ptr(), T::class_ptr()) } {
			Some(TypedObjRef::from_raw(self.raw))
		} else {
			None
		}
	}

	/// Check if this reference is of a specific widget type
	pub fn is<T: WidgetClassMarker>(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_check_type(self.as_raw_ptr(), T::class_ptr()) }
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
