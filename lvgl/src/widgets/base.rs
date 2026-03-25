extern crate alloc;

use super::obj::{AsRawObj, Obj, ObjRef};
use crate::event::{Event, EventCode, EventHandle};
use crate::misc::area::{Align, Area, Point};
use crate::misc::color::{Color, Opacity};
use crate::misc::grad::{GradDir, GradientDescriptor};
use crate::style::{BorderSide, ImageColorKey, Style, TextDecor};
use bitflags::bitflags;
use core::ptr::NonNull;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const LAYOUT_FLEX: u32 = lvgl_sys::lv_layout_t_LV_LAYOUT_FLEX;
pub const LAYOUT_GRID: u32 = lvgl_sys::lv_layout_t_LV_LAYOUT_GRID;

bitflags! {
	pub struct ObjFlags: lvgl_sys::lv_obj_flag_t {
		const HIDDEN = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN;
		const CLICKABLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CLICKABLE;
		const CLICK_FOCUSABLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CLICK_FOCUSABLE;
		const CHECKABLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CHECKABLE;
		const SCROLLABLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLLABLE;
		const SCROLL_ELASTIC = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ELASTIC;
		const SCROLL_MOMENTUM = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_MOMENTUM;
		const SCROLL_ONE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ONE;
		const SCROLL_CHAIN_HOR = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_CHAIN_HOR;
		const SCROLL_CHAIN_VER = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_CHAIN_VER;
		const SCROLL_CHAIN = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_CHAIN;
		const SCROLL_ON_FOCUS = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ON_FOCUS;
		const SCROLL_WITH_ARROW = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_WITH_ARROW;
		const SNAPPABLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SNAPPABLE;
		const PRESS_LOCK = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_PRESS_LOCK;
		const EVENT_BUBBLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_EVENT_BUBBLE;
		const GESTURE_BUBBLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_GESTURE_BUBBLE;
		const ADV_HITTEST = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_ADV_HITTEST;
		const IGNORE_LAYOUT = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT;
		const FLOATING = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_FLOATING;
		const SEND_DRAW_TASK_EVENTS = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SEND_DRAW_TASK_EVENTS;
		const OVERFLOW_VISIBLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_OVERFLOW_VISIBLE;
		const EVENT_TRICKLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_EVENT_TRICKLE;
		const STATE_TRICKLE = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_STATE_TRICKLE;
		const LAYOUT_1 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_LAYOUT_1;
		const LAYOUT_2 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_LAYOUT_2;
		const FLEX_IN_NEW_TRACK = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_FLEX_IN_NEW_TRACK;
		const WIDGET_1 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_WIDGET_1;
		const WIDGET_2 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_WIDGET_2;
		const USER_1 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_USER_1;
		const USER_2 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_USER_2;
		const USER_3 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_USER_3;
		const USER_4 = lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_USER_4;
	}

	pub struct State : lvgl_sys::lv_state_t {
		const DEFAULT = lvgl_sys::lv_state_t_LV_STATE_DEFAULT;
		const ALT = lvgl_sys::lv_state_t_LV_STATE_ALT;
		const CHECKED = lvgl_sys::lv_state_t_LV_STATE_CHECKED;
		const FOCUSED = lvgl_sys::lv_state_t_LV_STATE_FOCUSED;
		const FOCUS_KEY = lvgl_sys::lv_state_t_LV_STATE_FOCUS_KEY;
		const EDITED = lvgl_sys::lv_state_t_LV_STATE_EDITED;
		const HOVERED = lvgl_sys::lv_state_t_LV_STATE_HOVERED;
		const PRESSED = lvgl_sys::lv_state_t_LV_STATE_PRESSED;
		const SCROLLED = lvgl_sys::lv_state_t_LV_STATE_SCROLLED;
		const DISABLED = lvgl_sys::lv_state_t_LV_STATE_DISABLED;
		const USER_1 = lvgl_sys::lv_state_t_LV_STATE_USER_1;
		const USER_2 = lvgl_sys::lv_state_t_LV_STATE_USER_2;
		const USER_3 = lvgl_sys::lv_state_t_LV_STATE_USER_3;
		const USER_4 = lvgl_sys::lv_state_t_LV_STATE_USER_4;
		const ANY = lvgl_sys::lv_state_t_LV_STATE_ANY;
	}
}

#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum PointTransformFlag {
	None = lvgl_sys::lv_obj_point_transform_flag_t_LV_OBJ_POINT_TRANSFORM_FLAG_NONE,
	Recursive = lvgl_sys::lv_obj_point_transform_flag_t_LV_OBJ_POINT_TRANSFORM_FLAG_RECURSIVE,
	Inverse = lvgl_sys::lv_obj_point_transform_flag_t_LV_OBJ_POINT_TRANSFORM_FLAG_INVERSE,
	InverseRecursive =
		lvgl_sys::lv_obj_point_transform_flag_t_LV_OBJ_POINT_TRANSFORM_FLAG_INVERSE_RECURSIVE,
}

#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Part {
	Main = lvgl_sys::lv_part_t_LV_PART_MAIN,
	Scrollbar = lvgl_sys::lv_part_t_LV_PART_SCROLLBAR,
	Indicator = lvgl_sys::lv_part_t_LV_PART_INDICATOR,
	Knob = lvgl_sys::lv_part_t_LV_PART_KNOB,
	Selected = lvgl_sys::lv_part_t_LV_PART_SELECTED,
	Items = lvgl_sys::lv_part_t_LV_PART_ITEMS,
	Cursor = lvgl_sys::lv_part_t_LV_PART_CURSOR,
	Custom0 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x10000,
	Custom1 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x20000,
	Custom2 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x30000,
	Custom3 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x40000,
	Custom4 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x50000,
	Custom5 = lvgl_sys::lv_part_t_LV_PART_CUSTOM_FIRST + 0x60000,
	Any = lvgl_sys::lv_part_t_LV_PART_ANY,
}

pub struct StyleSelector(lvgl_sys::lv_style_selector_t);

impl StyleSelector {
	pub fn new(part: Part, state: State) -> Self {
		let part_value: u32 = part.into();
		let state_value: u32 = state.bits();
		Self(part_value | state_value)
	}
}

impl From<Part> for StyleSelector {
	fn from(part: Part) -> Self {
		Self(part.into())
	}
}

impl From<State> for StyleSelector {
	fn from(state: State) -> Self {
		Self(state.bits())
	}
}

pub trait Widget: AsRawObj {
	fn has_flag(&self, flag: ObjFlags) -> bool {
		unsafe { lvgl_sys::lv_obj_has_flag(self.as_raw_ptr(), flag.bits()) }
	}

	fn has_flag_any(&self, flag: ObjFlags) -> bool {
		unsafe { lvgl_sys::lv_obj_has_flag_any(self.as_raw_ptr(), flag.bits()) }
	}

	fn add_flag(&mut self, flag: ObjFlags) {
		unsafe { lvgl_sys::lv_obj_add_flag(self.as_raw_ptr(), flag.bits()) };
	}

	fn remove_flag(&mut self, flag: ObjFlags) {
		unsafe { lvgl_sys::lv_obj_remove_flag(self.as_raw_ptr(), flag.bits()) };
	}

	fn set_flag(&mut self, flag: ObjFlags, enabled: bool) {
		unsafe { lvgl_sys::lv_obj_set_flag(self.as_raw_ptr(), flag.bits(), enabled) };
	}

	fn get_state(&self) -> State {
		State::from_bits_truncate(unsafe { lvgl_sys::lv_obj_get_state(self.as_raw_ptr()) })
	}

	fn has_state(&self, state: State) -> bool {
		unsafe { lvgl_sys::lv_obj_has_state(self.as_raw_ptr(), state.bits()) }
	}

	fn add_state(&mut self, state: State) {
		unsafe { lvgl_sys::lv_obj_add_state(self.as_raw_ptr(), state.bits()) };
	}

	fn remove_state(&mut self, state: State) {
		unsafe { lvgl_sys::lv_obj_remove_state(self.as_raw_ptr(), state.bits()) };
	}

	fn set_state(&mut self, state: State, enabled: bool) {
		unsafe { lvgl_sys::lv_obj_set_state(self.as_raw_ptr(), state.bits(), enabled) };
	}

	fn is_radio_button(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_radio_button(self.as_raw_ptr()) }
	}

	fn add_event_cb<F: Fn(&Event) + 'static>(&mut self, filter: EventCode, f: F) -> EventHandle {
		EventHandle::add_to_obj(
			unsafe { NonNull::new_unchecked(self.as_raw_ptr()) },
			filter,
			f,
		)
	}

	fn remove_event_cb(&mut self, handle: EventHandle) {
		handle.remove();
	}

	fn set_pos(&mut self, x: i32, y: i32) {
		unsafe { lvgl_sys::lv_obj_set_pos(self.as_raw_ptr(), x, y) };
	}

	fn set_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_obj_set_x(self.as_raw_ptr(), x) };
	}

	fn set_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_obj_set_y(self.as_raw_ptr(), y) };
	}

	fn set_size(&mut self, width: i32, height: i32) {
		unsafe { lvgl_sys::lv_obj_set_size(self.as_raw_ptr(), width, height) };
	}

	/// Reclaculate the size of the object.
	/// Returns true if the size changed, false if it stayed the same.
	fn refresh_size(&mut self) -> bool {
		unsafe { lvgl_sys::lv_obj_refr_size(self.as_raw_ptr()) }
	}

	fn set_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_obj_set_width(self.as_raw_ptr(), width) };
	}

	fn set_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_obj_set_height(self.as_raw_ptr(), height) };
	}

	fn set_content_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_obj_set_content_width(self.as_raw_ptr(), width) };
	}

	fn set_content_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_obj_set_content_height(self.as_raw_ptr(), height) };
	}

	/// Sets the layout of the object's children.
	/// See LAYOUT_FLEX and LAYOUT_GRID for common layouts.
	fn set_layout(&mut self, layout: u32) {
		unsafe { lvgl_sys::lv_obj_set_layout(self.as_raw_ptr(), layout.into()) };
	}

	fn is_layout_positioned(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_layout_positioned(self.as_raw_ptr()) }
	}

	fn mark_layout_as_dirty(&mut self) {
		unsafe { lvgl_sys::lv_obj_mark_layout_as_dirty(self.as_raw_ptr()) };
	}

	fn update_layout(&mut self) {
		unsafe { lvgl_sys::lv_obj_update_layout(self.as_raw_ptr()) };
	}

	fn set_align(&mut self, align: Align) {
		unsafe { lvgl_sys::lv_obj_set_align(self.as_raw_ptr(), align.into()) };
	}

	fn align(&mut self, align: Align, x_offset: i32, y_offset: i32) {
		unsafe { lvgl_sys::lv_obj_align(self.as_raw_ptr(), align.into(), x_offset, y_offset) };
	}

	fn align_to(&mut self, base: &impl AsRawObj, align: Align, x_offset: i32, y_offset: i32) {
		unsafe {
			lvgl_sys::lv_obj_align_to(
				self.as_raw_ptr(),
				base.as_raw_ptr(),
				align.into(),
				x_offset,
				y_offset,
			)
		};
	}

	fn center(&mut self) {
		unsafe { lvgl_sys::lv_obj_center(self.as_raw_ptr()) };
	}

	// fn set_transform(&mut self, matrix: Matrix);

	fn reset_transform(&mut self) {
		unsafe { lvgl_sys::lv_obj_reset_transform(self.as_raw_ptr()) };
	}

	fn coords(&self) -> Area {
		let mut area = Area::default();
		unsafe { lvgl_sys::lv_obj_get_coords(self.as_raw_ptr(), area.as_raw_mut()) };
		area
	}

	fn x(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_x(self.as_raw_ptr()) }
	}

	fn x2(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_x2(self.as_raw_ptr()) }
	}

	fn y(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_y(self.as_raw_ptr()) }
	}

	fn y2(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_y2(self.as_raw_ptr()) }
	}

	fn x_aligned(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_x_aligned(self.as_raw_ptr()) }
	}

	fn y_aligned(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_y_aligned(self.as_raw_ptr()) }
	}

	fn width(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_width(self.as_raw_ptr()) }
	}

	fn height(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_height(self.as_raw_ptr()) }
	}

	fn content_width(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_content_width(self.as_raw_ptr()) }
	}

	fn content_height(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_content_height(self.as_raw_ptr()) }
	}

	fn content_coords(&self) -> Area {
		let mut area = Area::default();
		unsafe { lvgl_sys::lv_obj_get_content_coords(self.as_raw_ptr(), area.as_raw_mut()) };
		area
	}

	fn self_width(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_self_width(self.as_raw_ptr()) }
	}

	fn self_height(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_self_height(self.as_raw_ptr()) }
	}

	fn style_clamped_width(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_style_clamped_width(self.as_raw_ptr()) }
	}

	fn style_clamped_height(&self) -> i32 {
		unsafe { lvgl_sys::lv_obj_get_style_clamped_height(self.as_raw_ptr()) }
	}

	fn is_style_any_width_content(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_style_any_width_content(self.as_raw_ptr()) }
	}

	fn is_style_any_height_content(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_style_any_height_content(self.as_raw_ptr()) }
	}

	fn is_width_min(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_width_min(self.as_raw_ptr()) }
	}

	fn is_height_min(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_height_min(self.as_raw_ptr()) }
	}

	fn is_width_max(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_width_max(self.as_raw_ptr()) }
	}

	fn is_height_max(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_height_max(self.as_raw_ptr()) }
	}

	fn refresh_self_size(&mut self) -> bool {
		unsafe { lvgl_sys::lv_obj_refresh_self_size(self.as_raw_ptr()) }
	}

	fn transform_point(&self, point: &mut Point, flags: PointTransformFlag) {
		unsafe {
			lvgl_sys::lv_obj_transform_point(self.as_raw_ptr(), point.as_raw_mut(), flags as u32)
		};
	}

	fn is_area_visible(&self, area: &Area) -> bool {
		unsafe { lvgl_sys::lv_obj_area_is_visible(self.as_raw_ptr(), area.as_raw() as *mut _) }
	}

	fn is_visible(&self) -> bool {
		unsafe { lvgl_sys::lv_obj_is_visible(self.as_raw_ptr()) }
	}

	fn add_style(&mut self, style: &Style, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_add_style(self.as_raw_ptr(), style.as_raw_ptr(), selector.0) };
	}

	fn replace_style(&mut self, old_style: &Style, new_style: &Style, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_replace_style(
				self.as_raw_ptr(),
				old_style.as_raw_ptr(),
				new_style.as_raw_ptr(),
				selector.0,
			)
		};
	}

	fn remove_style(&mut self, style: &Style, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_remove_style(self.as_raw_ptr(), style.as_raw_ptr(), selector.0) };
	}

	fn remove_theme(&mut self, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_remove_theme(self.as_raw_ptr(), selector.0) };
	}

	fn remove_style_all(&mut self) {
		unsafe { lvgl_sys::lv_obj_remove_style_all(self.as_raw_ptr()) };
	}

	fn style_set_disabled(&mut self, style: &Style, selector: StyleSelector, disabled: bool) {
		unsafe {
			lvgl_sys::lv_obj_style_set_disabled(
				self.as_raw_ptr(),
				style.as_raw_ptr(),
				selector.0,
				disabled,
			)
		};
	}

	fn style_is_disabled(&self, style: &Style, selector: StyleSelector) -> bool {
		unsafe {
			lvgl_sys::lv_obj_style_get_disabled(self.as_raw_ptr(), style.as_raw_ptr(), selector.0)
		}
	}

	fn fade_in(&mut self, time: u32, delay: u32) {
		unsafe { lvgl_sys::lv_obj_fade_in(self.as_raw_ptr(), time, delay) };
	}

	fn fade_out(&mut self, time: u32, delay: u32) {
		unsafe { lvgl_sys::lv_obj_fade_out(self.as_raw_ptr(), time, delay) };
	}

	fn set_style_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_min_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_min_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_max_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_max_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_height(&mut self, height: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_height(self.as_raw_ptr(), height, selector.0) };
	}

	fn set_style_min_height(&mut self, height: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_min_height(self.as_raw_ptr(), height, selector.0) };
	}

	fn set_style_max_height(&mut self, height: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_max_height(self.as_raw_ptr(), height, selector.0) };
	}

	fn set_style_length(&mut self, length: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_length(self.as_raw_ptr(), length, selector.0) };
	}

	fn set_style_x(&mut self, x: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_x(self.as_raw_ptr(), x, selector.0) };
	}

	fn set_style_y(&mut self, y: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_y(self.as_raw_ptr(), y, selector.0) };
	}

	fn set_style_align(&mut self, align: Align, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_align(self.as_raw_ptr(), align.into(), selector.0) };
	}

	fn set_style_transform_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_transform_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_transform_height(&mut self, height: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_transform_height(self.as_raw_ptr(), height, selector.0)
		};
	}

	fn set_style_translate_x(&mut self, x: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_translate_x(self.as_raw_ptr(), x, selector.0) };
	}

	fn set_style_translate_y(&mut self, y: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_translate_y(self.as_raw_ptr(), y, selector.0) };
	}

	fn set_style_translate_radial(&mut self, value: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_translate_radial(self.as_raw_ptr(), value, selector.0)
		};
	}

	fn set_style_transform_scale_x(&mut self, scale: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_transform_scale_x(self.as_raw_ptr(), scale, selector.0)
		};
	}

	fn set_style_transform_scale_y(&mut self, scale: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_transform_scale_y(self.as_raw_ptr(), scale, selector.0)
		};
	}

	fn set_style_transform_rotation(&mut self, rotation: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_transform_rotation(self.as_raw_ptr(), rotation, selector.0)
		};
	}

	fn set_style_transform_pivot_x(&mut self, x: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_transform_pivot_x(self.as_raw_ptr(), x, selector.0) };
	}

	fn set_style_transform_pivot_y(&mut self, y: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_transform_pivot_y(self.as_raw_ptr(), y, selector.0) };
	}

	fn set_style_transform_skew_x(&mut self, skew: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_transform_skew_x(self.as_raw_ptr(), skew, selector.0) };
	}

	fn set_style_transform_skew_y(&mut self, skew: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_transform_skew_y(self.as_raw_ptr(), skew, selector.0) };
	}

	fn set_style_pad_top(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_top(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_bottom(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_bottom(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_left(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_left(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_right(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_right(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_row(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_row(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_column(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_column(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_pad_radial(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_pad_radial(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_margin_top(&mut self, margin: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_margin_top(self.as_raw_ptr(), margin, selector.0) };
	}

	fn set_style_margin_bottom(&mut self, margin: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_margin_bottom(self.as_raw_ptr(), margin, selector.0) };
	}

	fn set_style_margin_left(&mut self, margin: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_margin_left(self.as_raw_ptr(), margin, selector.0) };
	}

	fn set_style_margin_right(&mut self, margin: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_margin_right(self.as_raw_ptr(), margin, selector.0) };
	}

	fn set_style_bg_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_bg_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_bg_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	fn set_style_bg_grad_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_grad_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_bg_grad_dir(&mut self, dir: GradDir, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_grad_dir(
				self.as_raw_ptr(),
				dir as lvgl_sys::lv_grad_dir_t,
				selector.0,
			)
		};
	}

	fn set_style_bg_main_stop(&mut self, stop: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_bg_main_stop(self.as_raw_ptr(), stop, selector.0) };
	}

	fn set_style_bg_grad_stop(&mut self, stop: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_bg_grad_stop(self.as_raw_ptr(), stop, selector.0) };
	}

	fn set_style_bg_main_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_main_opa(self.as_raw_ptr(), opa.into(), selector.0)
		};
	}

	fn set_style_bg_grad_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_grad_opa(self.as_raw_ptr(), opa.into(), selector.0)
		};
	}

	fn set_style_bg_grad(&mut self, grad: &GradientDescriptor, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_grad(self.as_raw_ptr(), grad.as_raw_ptr(), selector.0)
		}
	}

	// fn set_style_bg_image_src(&mut self, src: *const (), selector: StyleSelector);

	fn set_style_bg_image_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_image_opa(self.as_raw_ptr(), opa.into(), selector.0)
		};
	}

	fn set_style_bg_image_recolor(&mut self, recolor: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_image_recolor(
				self.as_raw_ptr(),
				recolor.as_raw(),
				selector.0,
			)
		};
	}

	fn set_style_bg_image_recolor_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_bg_image_recolor_opa(
				self.as_raw_ptr(),
				opa.into(),
				selector.0,
			)
		};
	}

	fn set_style_bg_image_tiled(&mut self, tiled: bool, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_bg_image_tiled(self.as_raw_ptr(), tiled, selector.0) };
	}

	fn set_style_border_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_border_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_border_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_border_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	fn set_style_border_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_border_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_border_side(&mut self, side: BorderSide, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_border_side(self.as_raw_ptr(), side.bits(), selector.0)
		};
	}

	fn set_style_border_post(&mut self, post: bool, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_border_post(self.as_raw_ptr(), post, selector.0) };
	}

	fn set_style_outline_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_outline_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_outline_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_outline_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_outline_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_outline_opa(self.as_raw_ptr(), opa.into(), selector.0)
		};
	}

	fn set_style_outline_pad(&mut self, pad: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_outline_pad(self.as_raw_ptr(), pad, selector.0) };
	}

	fn set_style_shadow_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_shadow_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_shadow_offset_x(&mut self, x: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_shadow_offset_x(self.as_raw_ptr(), x, selector.0) };
	}

	fn set_style_shadow_offset_y(&mut self, y: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_shadow_offset_y(self.as_raw_ptr(), y, selector.0) };
	}

	fn set_style_shadow_spread(&mut self, spread: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_shadow_spread(self.as_raw_ptr(), spread, selector.0) };
	}

	fn set_style_shadow_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_shadow_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_shadow_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_shadow_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	fn set_style_image_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_image_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	fn set_style_image_recolor(&mut self, recolor: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_image_recolor(
				self.as_raw_ptr(),
				recolor.as_raw(),
				selector.0,
			)
		};
	}

	fn set_style_image_recolor_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_image_recolor_opa(self.as_raw_ptr(), opa.into(), selector.0)
		};
	}

	fn set_style_image_colorkey(&mut self, colorkey: &ImageColorKey, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_image_colorkey(
				self.as_raw_ptr(),
				colorkey.as_raw_ptr(),
				selector.0,
			)
		};
	}

	fn set_style_line_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_line_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_line_dash_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_line_dash_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_line_dash_gap(&mut self, gap: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_line_dash_gap(self.as_raw_ptr(), gap, selector.0) };
	}

	fn set_style_line_rounded(&mut self, rounded: bool, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_line_rounded(self.as_raw_ptr(), rounded, selector.0) };
	}

	fn set_style_line_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_line_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_line_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_line_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	fn set_style_arc_width(&mut self, width: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_arc_width(self.as_raw_ptr(), width, selector.0) };
	}

	fn set_style_arc_rounded(&mut self, rounded: bool, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_arc_rounded(self.as_raw_ptr(), rounded, selector.0) };
	}

	fn set_style_arc_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_arc_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_arc_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_arc_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	// fn set_style_arc_image_src(&mut self, src: *const (), selector: StyleSelector);

	fn set_style_text_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_text_color(self.as_raw_ptr(), color.as_raw(), selector.0)
		};
	}

	fn set_style_text_opa(&mut self, opa: Opacity, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_text_opa(self.as_raw_ptr(), opa.into(), selector.0) };
	}

	// fn set_style_text_font(&mut self, font: &Font, selector: StyleSelector) {
	// 	unsafe {
	// 		lvgl_sys::lv_obj_set_style_text_font(self.as_raw_ptr(), font.as_raw_ptr(), selector.0)
	// 	};
	// }

	fn set_style_text_letter_space(&mut self, space: i32, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_text_letter_space(self.as_raw_ptr(), space, selector.0)
		};
	}

	fn set_style_text_line_space(&mut self, space: i32, selector: StyleSelector) {
		unsafe { lvgl_sys::lv_obj_set_style_text_line_space(self.as_raw_ptr(), space, selector.0) };
	}

	fn set_style_text_decor(&mut self, decor: TextDecor, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_text_decor(self.as_raw_ptr(), decor.bits(), selector.0)
		};
	}

	fn set_style_text_align(&mut self, align: Align, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_text_align(self.as_raw_ptr(), align.into(), selector.0)
		};
	}

	fn set_style_text_outline_stroke_color(&mut self, color: Color, selector: StyleSelector) {
		unsafe {
			lvgl_sys::lv_obj_set_style_text_outline_stroke_color(
				self.as_raw_ptr(),
				color.as_raw(),
				selector.0,
			)
		};
	}
}

impl Widget for Obj {}
impl<'a> Widget for ObjRef<'a> {}
