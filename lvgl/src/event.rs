extern crate alloc;

use crate::widgets::obj::ObjRef;
use crate::{misc::area::Area, widgets::base::State};
use alloc::boxed::Box;
use core::ffi::CStr;
use core::marker::PhantomData;
use core::ptr::NonNull;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub struct Event<'a> {
	raw: *mut lvgl_sys::lv_event_t,
	_marker: PhantomData<&'a lvgl_sys::lv_event_t>,
}

impl<'a> Event<'a> {
	pub fn target_obj(&self) -> ObjRef<'a> {
		ObjRef::from_raw(unsafe { lvgl_sys::lv_event_get_target_obj(self.raw) })
			.expect("Received null event target from LVGL")
	}

	pub fn current_target_obj(&self) -> ObjRef<'a> {
		ObjRef::from_raw(unsafe { lvgl_sys::lv_event_get_current_target_obj(self.raw) })
			.expect("Received null event current target from LVGL")
	}

	pub fn code(&self) -> EventCode {
		EventCode::try_from(unsafe { lvgl_sys::lv_event_get_code(self.raw) })
			.expect("Invalid event code received from LVGL")
	}

	pub fn stop_bubbling(&mut self) {
		unsafe { lvgl_sys::lv_event_stop_bubbling(self.raw) };
	}

	pub fn stop_trickling(&mut self) {
		unsafe { lvgl_sys::lv_event_stop_trickling(self.raw) };
	}

	pub fn stop_processing(&mut self) {
		unsafe { lvgl_sys::lv_event_stop_processing(self.raw) };
	}

	// pub fn indev(&self) -> Option<InutDevice>;

	// pub fn layer(&self) -> Option<Layer>;

	pub fn old_size(&self) -> Option<Area> {
		let area = unsafe { lvgl_sys::lv_event_get_old_size(self.raw) };
		if area.is_null() {
			None
		} else {
			Some(Area::from_raw(area))
		}
	}

	pub fn key(&self) -> Option<u32> {
		let key = unsafe { lvgl_sys::lv_event_get_key(self.raw) };
		if key != 0 { Some(key) } else { None }
	}

	pub fn rotary_diff(&self) -> Option<i32> {
		let diff = unsafe { lvgl_sys::lv_event_get_rotary_diff(self.raw) };
		if diff != 0 { Some(diff) } else { None }
	}

	// pub fn scroll_anim(&self) -> Option<Animation>;

	// pub fn set_ext_draw_size(&mut self, size: i16) {
	// 	unsafe { lvgl_sys::lv_event_set_ext_draw_size(self.raw, size) };
	// }

	// pub fn self_size_info(&mut self) -> Option<&mut Point>;

	// pub fn hit_test_info(&mut self) -> Option<&mut HitTestInfo>;

	pub fn cover_area(&self) -> Option<Area> {
		let area = unsafe { lvgl_sys::lv_event_get_cover_area(self.raw) };
		if area.is_null() {
			None
		} else {
			Some(Area::from_raw(area))
		}
	}

	// pub fn set_cover_result(&mut self, result: CoverResult);

	// pub fn get_draw_task(&mut self) -> Option<&mut DrawTask>;

	pub fn prev_state(&self) -> State {
		State::from_bits_truncate(unsafe { lvgl_sys::lv_event_get_prev_state(self.raw) })
	}
}

#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum EventCode {
	All = lvgl_sys::lv_event_code_t_LV_EVENT_ALL,

	/* Input device events */
	Pressed = lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
	PressLost = lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
	ShortClicked = lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
	SingleClicked = lvgl_sys::lv_event_code_t_LV_EVENT_SINGLE_CLICKED,
	DoubleClicked = lvgl_sys::lv_event_code_t_LV_EVENT_DOUBLE_CLICKED,
	TripleClicked = lvgl_sys::lv_event_code_t_LV_EVENT_TRIPLE_CLICKED,
	LongPressed = lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
	LongPressedRepeat = lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT,
	Clicked = lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
	Released = lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
	ScrollBegin = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL_BEGIN,
	ScrollThrowBegin = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL_THROW_BEGIN,
	ScrollEnd = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL_END,
	Scroll = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL,
	Gesture = lvgl_sys::lv_event_code_t_LV_EVENT_GESTURE,
	Key = lvgl_sys::lv_event_code_t_LV_EVENT_KEY,
	Rotary = lvgl_sys::lv_event_code_t_LV_EVENT_ROTARY,
	Focused = lvgl_sys::lv_event_code_t_LV_EVENT_FOCUSED,
	Defocused = lvgl_sys::lv_event_code_t_LV_EVENT_DEFOCUSED,
	Leave = lvgl_sys::lv_event_code_t_LV_EVENT_LEAVE,
	HitTest = lvgl_sys::lv_event_code_t_LV_EVENT_HIT_TEST,
	IndevReset = lvgl_sys::lv_event_code_t_LV_EVENT_INDEV_RESET,
	HoverOver = lvgl_sys::lv_event_code_t_LV_EVENT_HOVER_OVER,

	/* Drawing events */
	CoverCheck = lvgl_sys::lv_event_code_t_LV_EVENT_COVER_CHECK,
	RefrExtDrawSize = lvgl_sys::lv_event_code_t_LV_EVENT_REFR_EXT_DRAW_SIZE,
	DrawMainBegin = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN,
	DrawMain = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN,
	DrawMainEnd = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END,
	DrawPostBegin = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN,
	DrawPost = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST,
	DrawPostEnd = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END,
	DrawTaskAdded = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_TASK_ADDED,

	/* Special events */
	ValueChanged = lvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED,
	Insert = lvgl_sys::lv_event_code_t_LV_EVENT_INSERT,
	Refresh = lvgl_sys::lv_event_code_t_LV_EVENT_REFRESH,
	Ready = lvgl_sys::lv_event_code_t_LV_EVENT_READY,
	Cancel = lvgl_sys::lv_event_code_t_LV_EVENT_CANCEL,
	StateChanged = lvgl_sys::lv_event_code_t_LV_EVENT_STATE_CHANGED,

	/* Other events */
	Created = lvgl_sys::lv_event_code_t_LV_EVENT_CREATE,
	Deleted = lvgl_sys::lv_event_code_t_LV_EVENT_DELETE,
	ChildChanged = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CHANGED,
	ChildCreated = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CREATED,
	ChildDeleted = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_DELETED,
	ScreenUnloadStart = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOAD_START,
	ScreenLoadStart = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOAD_START,
	ScreenLoaded = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOADED,
	ScreenUnloaded = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOADED,
	SizeChanged = lvgl_sys::lv_event_code_t_LV_EVENT_SIZE_CHANGED,
	StyleChanged = lvgl_sys::lv_event_code_t_LV_EVENT_STYLE_CHANGED,
	LayoutChanged = lvgl_sys::lv_event_code_t_LV_EVENT_LAYOUT_CHANGED,
	GetSelfSize = lvgl_sys::lv_event_code_t_LV_EVENT_GET_SELF_SIZE,

	/* Events of optional LVGL components */
	InvalidateArea = lvgl_sys::lv_event_code_t_LV_EVENT_INVALIDATE_AREA,
	ResolutionChanged = lvgl_sys::lv_event_code_t_LV_EVENT_RESOLUTION_CHANGED,
	ColorFormatChanged = lvgl_sys::lv_event_code_t_LV_EVENT_COLOR_FORMAT_CHANGED,
	RefrRequest = lvgl_sys::lv_event_code_t_LV_EVENT_REFR_REQUEST,
	RefrStart = lvgl_sys::lv_event_code_t_LV_EVENT_REFR_START,
	RefrReady = lvgl_sys::lv_event_code_t_LV_EVENT_REFR_READY,
	RenderStart = lvgl_sys::lv_event_code_t_LV_EVENT_RENDER_START,
	RenderReady = lvgl_sys::lv_event_code_t_LV_EVENT_RENDER_READY,
	FlushStart = lvgl_sys::lv_event_code_t_LV_EVENT_FLUSH_START,
	FlushFinish = lvgl_sys::lv_event_code_t_LV_EVENT_FLUSH_FINISH,
	FlushWaitStart = lvgl_sys::lv_event_code_t_LV_EVENT_FLUSH_WAIT_START,
	FlushWaitFinish = lvgl_sys::lv_event_code_t_LV_EVENT_FLUSH_WAIT_FINISH,
	UpdateLayoutCompleted = lvgl_sys::lv_event_code_t_LV_EVENT_UPDATE_LAYOUT_COMPLETED,

	Vsync = lvgl_sys::lv_event_code_t_LV_EVENT_VSYNC,
	VsyncRequest = lvgl_sys::lv_event_code_t_LV_EVENT_VSYNC_REQUEST,
}

impl EventCode {
	pub fn name(&self) -> &'static str {
		unsafe { CStr::from_ptr(lvgl_sys::lv_event_code_get_name((*self).into())) }
			.to_str()
			.unwrap_or("Invalid event code")
	}
}

pub struct EventHandle {
	obj: *mut lvgl_sys::lv_obj_t,
	call_dsc: *mut lvgl_sys::lv_event_dsc_t,
	drop_dsc: *mut lvgl_sys::lv_event_dsc_t,
	data: *mut (),
	free_fn: unsafe fn(*mut ()),
}

impl EventHandle {
	pub(crate) fn add_to_obj<F: Fn(&Event) + 'static>(
		obj: NonNull<lvgl_sys::lv_obj_t>,
		filter: EventCode,
		callback: F,
	) -> Self {
		let boxed_callback = Box::new(callback);
		let data_ptr = Box::into_raw(boxed_callback);

		unsafe {
			let call_dsc = lvgl_sys::lv_obj_add_event_cb(
				obj.as_ptr(),
				Some(call_trampoline::<F>),
				filter.into(),
				data_ptr.cast(),
			);
			let drop_dsc = lvgl_sys::lv_obj_add_event_cb(
				obj.as_ptr(),
				Some(drop_trampoline::<F>),
				lvgl_sys::lv_event_code_t_LV_EVENT_DELETE,
				data_ptr.cast(),
			);
			Self {
				obj: obj.as_ptr(),
				call_dsc,
				drop_dsc,
				data: data_ptr.cast(),
				free_fn: free_box::<F>,
			}
		}
	}

	pub fn remove(self) {
		unsafe {
			lvgl_sys::lv_obj_remove_event_dsc(self.obj, self.call_dsc);
			lvgl_sys::lv_obj_remove_event_dsc(self.obj, self.drop_dsc);
			(self.free_fn)(self.data);
		}
	}
}

unsafe fn free_box<F>(ptr: *mut ()) {
	unsafe { drop(Box::from_raw(ptr as *mut F)) };
}

unsafe extern "C" fn call_trampoline<F: Fn(&Event)>(e: *mut lvgl_sys::lv_event_t) {
	let ptr = unsafe { lvgl_sys::lv_event_get_user_data(e) } as *const F;
	if ptr.is_null() {
		return;
	}
	let event = Event {
		raw: e,
		_marker: PhantomData,
	};
	unsafe { (*ptr)(&event) };
}

unsafe extern "C" fn drop_trampoline<F: Fn(&Event)>(e: *mut lvgl_sys::lv_event_t) {
	let ptr = unsafe { lvgl_sys::lv_event_get_user_data(e) } as *mut F;
	if !ptr.is_null() {
		unsafe { drop(Box::from_raw(ptr)) };
	}
}
