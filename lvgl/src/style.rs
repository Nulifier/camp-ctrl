extern crate alloc;

use crate::{
	layouts::{
		flex::{FlexAlign, FlexFlow},
		grid::{GRID_TEMPLATE_LAST, GridAlign},
	},
	misc::{
		area::Align,
		bidi::BaseDir,
		color::{Color, Opacity},
		grad::{GradDir, GradientDescriptor},
		text::TextAlign,
	},
};
use bitflags::bitflags;
use core::mem::MaybeUninit;

#[repr(u32)]
pub enum BlendMode {
	Normal = lvgl_sys::lv_blend_mode_t_LV_BLEND_MODE_NORMAL,
	Additive = lvgl_sys::lv_blend_mode_t_LV_BLEND_MODE_ADDITIVE,
	Subtractive = lvgl_sys::lv_blend_mode_t_LV_BLEND_MODE_SUBTRACTIVE,
	Multiply = lvgl_sys::lv_blend_mode_t_LV_BLEND_MODE_MULTIPLY,
	Difference = lvgl_sys::lv_blend_mode_t_LV_BLEND_MODE_DIFFERENCE,
}

bitflags! {
	pub struct TextDecor: u32 {
		const NONE = lvgl_sys::lv_text_decor_t_LV_TEXT_DECOR_NONE;
		const UNDERLINE = lvgl_sys::lv_text_decor_t_LV_TEXT_DECOR_UNDERLINE;
		const STRIKETHROUGH = lvgl_sys::lv_text_decor_t_LV_TEXT_DECOR_STRIKETHROUGH;
	}

	pub struct BorderSide: u32 {
		const NONE = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_NONE;
		const BOTTOM = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_BOTTOM;
		const TOP = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_TOP;
		const LEFT = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_LEFT;
		const RIGHT = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_RIGHT;
		const FULL = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_FULL;
		const INTERNAL = lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_INTERNAL;
	}
}

#[repr(u32)]
pub enum BlurQuality {
	Auto = lvgl_sys::lv_blur_quality_t_LV_BLUR_QUALITY_AUTO,
	Speed = lvgl_sys::lv_blur_quality_t_LV_BLUR_QUALITY_SPEED,
	Precision = lvgl_sys::lv_blur_quality_t_LV_BLUR_QUALITY_PRECISION,
}

pub struct ImageColorKey(lvgl_sys::lv_image_colorkey_t);

impl ImageColorKey {
	pub(crate) fn as_raw_ptr(&self) -> *const lvgl_sys::lv_image_colorkey_t {
		&self.0
	}

	pub fn new(low: Color, high: Color) -> Self {
		Self(lvgl_sys::lv_image_colorkey_t {
			low: low.as_raw(),
			high: high.as_raw(),
		})
	}
}

pub struct Style(lvgl_sys::lv_style_t);

impl Style {
	pub(crate) fn as_raw_ptr(&self) -> *const lvgl_sys::lv_style_t {
		&self.0
	}

	unsafe fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_style_t {
		&mut self.0
	}

	pub fn new() -> Self {
		let mut style = MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
		unsafe { lvgl_sys::lv_style_init(style.as_mut_ptr()) };
		Self(unsafe { style.assume_init() })
	}

	pub fn reset(&mut self) {
		unsafe { lvgl_sys::lv_style_reset(self.as_raw_mut()) };
	}

	pub fn merge<'a>(&mut self, other: &Style) {
		unsafe { lvgl_sys::lv_style_merge(self.as_raw_mut(), other.as_raw_ptr()) };
	}

	pub fn is_empty(&self) -> bool {
		unsafe { lvgl_sys::lv_style_is_empty(self.as_raw_ptr()) }
	}

	pub fn set_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_width(self.as_raw_mut(), width) };
	}

	pub fn set_min_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_min_width(self.as_raw_mut(), width) };
	}

	pub fn set_max_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_max_width(self.as_raw_mut(), width) };
	}

	pub fn set_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_style_set_height(self.as_raw_mut(), height) };
	}

	pub fn set_min_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_style_set_min_height(self.as_raw_mut(), height) };
	}

	pub fn set_max_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_style_set_max_height(self.as_raw_mut(), height) };
	}

	pub fn set_length(&mut self, length: i32) {
		unsafe { lvgl_sys::lv_style_set_length(self.as_raw_mut(), length) };
	}

	pub fn set_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_x(self.as_raw_mut(), x) };
	}

	pub fn set_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_style_set_y(self.as_raw_mut(), y) };
	}

	pub fn set_align(&mut self, align: Align) {
		unsafe { lvgl_sys::lv_style_set_align(self.as_raw_mut(), align.into()) };
	}

	pub fn set_transform_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_width(self.as_raw_mut(), width) };
	}

	pub fn set_transform_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_height(self.as_raw_mut(), height) };
	}

	pub fn set_translate_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_translate_x(self.as_raw_mut(), x) };
	}

	pub fn set_translate_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_style_set_translate_y(self.as_raw_mut(), y) };
	}

	pub fn set_translate_radial(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_translate_radial(self.as_raw_mut(), value) };
	}

	pub fn set_transform_scale_x(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_scale_x(self.as_raw_mut(), value) };
	}

	pub fn set_transform_scale_y(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_scale_y(self.as_raw_mut(), value) };
	}

	pub fn set_transform_rotation(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_rotation(self.as_raw_mut(), value) };
	}

	pub fn set_transform_pivot_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_pivot_x(self.as_raw_mut(), x) };
	}

	pub fn set_transform_pivot_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_pivot_y(self.as_raw_mut(), y) };
	}

	pub fn set_transform_skew_x(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_skew_x(self.as_raw_mut(), value) };
	}

	pub fn set_transform_skew_y(&mut self, value: i32) {
		unsafe { lvgl_sys::lv_style_set_transform_skew_y(self.as_raw_mut(), value) };
	}

	pub fn set_pad_top(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_top(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_bottom(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_bottom(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_left(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_left(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_right(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_right(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_row(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_row(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_column(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_column(self.as_raw_mut(), pad) };
	}

	pub fn set_pad_radial(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_pad_radial(self.as_raw_mut(), pad) };
	}

	pub fn set_margin_top(&mut self, margin: i32) {
		unsafe { lvgl_sys::lv_style_set_margin_top(self.as_raw_mut(), margin) };
	}

	pub fn set_margin_bottom(&mut self, margin: i32) {
		unsafe { lvgl_sys::lv_style_set_margin_bottom(self.as_raw_mut(), margin) };
	}

	pub fn set_margin_left(&mut self, margin: i32) {
		unsafe { lvgl_sys::lv_style_set_margin_left(self.as_raw_mut(), margin) };
	}

	pub fn set_margin_right(&mut self, margin: i32) {
		unsafe { lvgl_sys::lv_style_set_margin_right(self.as_raw_mut(), margin) };
	}

	pub fn set_bg_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_bg_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_bg_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_bg_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_bg_grad_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_bg_grad_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_bg_grad_dir(&mut self, dir: GradDir) {
		unsafe {
			lvgl_sys::lv_style_set_bg_grad_dir(self.as_raw_mut(), dir as lvgl_sys::lv_grad_dir_t)
		};
	}

	pub fn set_bg_main_stop(&mut self, stop: i32) {
		unsafe { lvgl_sys::lv_style_set_bg_main_stop(self.as_raw_mut(), stop) };
	}

	pub fn set_bg_main_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_bg_main_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_bg_grad_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_bg_grad_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_bg_grad(&mut self, grad: &GradientDescriptor) {
		unsafe { lvgl_sys::lv_style_set_bg_grad(self.as_raw_mut(), grad.as_raw_ptr()) };
	}

	// pub fn set_bg_image_src(&mut self, src: *const ());

	pub fn set_bg_image_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_bg_image_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_bg_image_recolor(&mut self, recolor: Color) {
		unsafe { lvgl_sys::lv_style_set_bg_image_recolor(self.as_raw_mut(), recolor.as_raw()) };
	}

	pub fn set_bg_image_recolor_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_bg_image_recolor_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_bg_image_tiled(&mut self, tiled: bool) {
		unsafe { lvgl_sys::lv_style_set_bg_image_tiled(self.as_raw_mut(), tiled) };
	}

	pub fn set_border_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_border_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_border_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_border_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_border_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_border_width(self.as_raw_mut(), width) };
	}

	pub fn set_border_side(&mut self, side: BorderSide) {
		unsafe { lvgl_sys::lv_style_set_border_side(self.as_raw_mut(), side.bits()) };
	}

	pub fn set_border_post(&mut self, post: bool) {
		unsafe { lvgl_sys::lv_style_set_border_post(self.as_raw_mut(), post) };
	}

	pub fn set_outline_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_outline_width(self.as_raw_mut(), width) };
	}

	pub fn set_outline_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_outline_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_outline_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_outline_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_outline_pad(&mut self, pad: i32) {
		unsafe { lvgl_sys::lv_style_set_outline_pad(self.as_raw_mut(), pad) };
	}

	pub fn set_shadow_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_shadow_width(self.as_raw_mut(), width) };
	}

	pub fn set_shadow_offset_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_shadow_offset_x(self.as_raw_mut(), x) };
	}

	pub fn set_shadow_offset_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_style_set_shadow_offset_y(self.as_raw_mut(), y) };
	}

	pub fn set_shadow_spread(&mut self, spread: i32) {
		unsafe { lvgl_sys::lv_style_set_shadow_spread(self.as_raw_mut(), spread) };
	}

	pub fn set_shadow_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_shadow_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_shadow_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_shadow_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_image_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_image_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_image_recolor(&mut self, recolor: Color) {
		unsafe { lvgl_sys::lv_style_set_image_recolor(self.as_raw_mut(), recolor.as_raw()) };
	}

	pub fn set_image_recolor_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_image_recolor_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_image_colorkey(&mut self, colorkey: &ImageColorKey) {
		unsafe { lvgl_sys::lv_style_set_image_colorkey(self.as_raw_mut(), colorkey.as_raw_ptr()) };
	}

	pub fn set_line_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_line_width(self.as_raw_mut(), width) };
	}

	pub fn set_line_dash_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_line_dash_width(self.as_raw_mut(), width) };
	}

	pub fn set_line_dash_gap(&mut self, gap: i32) {
		unsafe { lvgl_sys::lv_style_set_line_dash_gap(self.as_raw_mut(), gap) };
	}

	pub fn set_line_rounded(&mut self, rounded: bool) {
		unsafe { lvgl_sys::lv_style_set_line_rounded(self.as_raw_mut(), rounded) };
	}

	pub fn set_line_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_line_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_line_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_line_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_arc_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_arc_width(self.as_raw_mut(), width) };
	}

	pub fn set_arc_rounded(&mut self, rounded: bool) {
		unsafe { lvgl_sys::lv_style_set_arc_rounded(self.as_raw_mut(), rounded) };
	}

	pub fn set_arc_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_arc_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_arc_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_arc_opa(self.as_raw_mut(), opa.into()) };
	}

	// pub fn set_arc_image_src(&mut self, src: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_arc_image_src(self.as_raw_mut(), src) };
	// }

	pub fn set_text_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_text_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_text_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_text_opa(self.as_raw_mut(), opa.into()) };
	}

	// pub fn set_text_font(&mut self, font: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_text_font(self.as_raw_mut(), font) };
	// }

	pub fn set_text_letter_space(&mut self, space: i32) {
		unsafe { lvgl_sys::lv_style_set_text_letter_space(self.as_raw_mut(), space) };
	}

	pub fn set_text_line_space(&mut self, space: i32) {
		unsafe { lvgl_sys::lv_style_set_text_line_space(self.as_raw_mut(), space) };
	}

	pub fn set_text_decor(&mut self, decor: TextDecor) {
		unsafe { lvgl_sys::lv_style_set_text_decor(self.as_raw_mut(), decor.bits()) };
	}

	pub fn set_text_align(&mut self, align: TextAlign) {
		unsafe {
			lvgl_sys::lv_style_set_text_align(self.as_raw_mut(), align as lvgl_sys::lv_text_align_t)
		};
	}

	pub fn set_text_outline_stroke_color(&mut self, color: Color) {
		unsafe {
			lvgl_sys::lv_style_set_text_outline_stroke_color(self.as_raw_mut(), color.as_raw())
		};
	}

	pub fn set_text_outline_stroke_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_style_set_text_outline_stroke_width(self.as_raw_mut(), width) };
	}

	pub fn set_text_outline_stroke_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_text_outline_stroke_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_blur_radius(&mut self, radius: i32) {
		unsafe { lvgl_sys::lv_style_set_blur_radius(self.as_raw_mut(), radius) };
	}

	pub fn set_blur_backdrop(&mut self, backdrop: bool) {
		unsafe { lvgl_sys::lv_style_set_blur_backdrop(self.as_raw_mut(), backdrop) };
	}

	pub fn set_blur_quality(&mut self, quality: BlurQuality) {
		unsafe {
			lvgl_sys::lv_style_set_blur_quality(
				self.as_raw_mut(),
				quality as lvgl_sys::lv_blur_quality_t,
			)
		};
	}

	pub fn set_drop_shadow_radius(&mut self, radius: i32) {
		unsafe { lvgl_sys::lv_style_set_drop_shadow_radius(self.as_raw_mut(), radius) };
	}

	pub fn set_drop_shadow_offset_x(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_drop_shadow_offset_x(self.as_raw_mut(), x) };
	}

	pub fn set_drop_shadow_offset_y(&mut self, y: i32) {
		unsafe { lvgl_sys::lv_style_set_drop_shadow_offset_y(self.as_raw_mut(), y) };
	}

	pub fn set_drop_shadow_color(&mut self, color: Color) {
		unsafe { lvgl_sys::lv_style_set_drop_shadow_color(self.as_raw_mut(), color.as_raw()) };
	}

	pub fn set_drop_shadow_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_drop_shadow_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_drop_shadow_quality(&mut self, quality: BlurQuality) {
		unsafe {
			lvgl_sys::lv_style_set_drop_shadow_quality(
				self.as_raw_mut(),
				quality as lvgl_sys::lv_blur_quality_t,
			)
		};
	}

	pub fn set_radius(&mut self, radius: i32) {
		unsafe { lvgl_sys::lv_style_set_radius(self.as_raw_mut(), radius) };
	}

	pub fn set_radial_offset(&mut self, x: i32) {
		unsafe { lvgl_sys::lv_style_set_radial_offset(self.as_raw_mut(), x) };
	}

	pub fn set_clip_corner(&mut self, clip: bool) {
		unsafe { lvgl_sys::lv_style_set_clip_corner(self.as_raw_mut(), clip) };
	}

	pub fn set_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_opa_layered(&mut self, value: Opacity) {
		unsafe { lvgl_sys::lv_style_set_opa_layered(self.as_raw_mut(), value.into()) };
	}

	// pub fn set_color_filter_descriptor(&mut self, descr: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_color_filter_descriptor(self.as_raw_mut(), descr) };
	// }

	pub fn set_color_filter_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_color_filter_opa(self.as_raw_mut(), opa.into()) };
	}

	pub fn set_recolor(&mut self, recolor: Color) {
		unsafe { lvgl_sys::lv_style_set_recolor(self.as_raw_mut(), recolor.as_raw()) };
	}

	pub fn set_recolor_opa(&mut self, opa: Opacity) {
		unsafe { lvgl_sys::lv_style_set_recolor_opa(self.as_raw_mut(), opa.into()) };
	}

	// pub fn set_anim(&mut self, anim: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_anim(self.as_raw_mut(), anim) };
	// }

	pub fn set_anim_duration(&mut self, duration_ms: u32) {
		unsafe { lvgl_sys::lv_style_set_anim_duration(self.as_raw_mut(), duration_ms) };
	}

	// pub fn set_transition(&mut self, transition: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_transition(self.as_raw_mut(), transition) };
	// }

	pub fn set_blend_mode(&mut self, mode: BlendMode) {
		unsafe {
			lvgl_sys::lv_style_set_blend_mode(self.as_raw_mut(), mode as lvgl_sys::lv_blend_mode_t)
		};
	}

	pub fn set_layout(&mut self, layout: u32) {
		unsafe { lvgl_sys::lv_style_set_layout(self.as_raw_mut(), layout as u16) };
	}

	pub fn set_base_dir(&mut self, base_dir: BaseDir) {
		unsafe {
			lvgl_sys::lv_style_set_base_dir(self.as_raw_mut(), base_dir as lvgl_sys::lv_base_dir_t)
		};
	}

	// pub fn set_bitmap_mask_src(&mut self, src: *const ()) {
	// 	unsafe { lvgl_sys::lv_style_set_bitmap_mask_src(self.as_raw_mut(), src) };
	// }

	pub fn set_rotary_sensitivity(&mut self, sensitivity: u32) {
		unsafe { lvgl_sys::lv_style_set_rotary_sensitivity(self.as_raw_mut(), sensitivity) };
	}

	pub fn set_flex_flow(&mut self, flow: FlexFlow) {
		unsafe {
			lvgl_sys::lv_style_set_flex_flow(self.as_raw_mut(), flow as lvgl_sys::lv_flex_flow_t)
		};
	}

	pub fn set_flex_main_place(&mut self, place: FlexAlign) {
		unsafe {
			lvgl_sys::lv_style_set_flex_main_place(
				self.as_raw_mut(),
				place as lvgl_sys::lv_flex_align_t,
			)
		};
	}

	pub fn set_flex_cross_place(&mut self, place: FlexAlign) {
		unsafe {
			lvgl_sys::lv_style_set_flex_cross_place(
				self.as_raw_mut(),
				place as lvgl_sys::lv_flex_align_t,
			)
		};
	}

	pub fn set_flex_track_place(&mut self, place: FlexAlign) {
		unsafe {
			lvgl_sys::lv_style_set_flex_track_place(
				self.as_raw_mut(),
				place as lvgl_sys::lv_flex_align_t,
			)
		};
	}

	pub fn set_flex_grow(&mut self, grow: u8) {
		unsafe { lvgl_sys::lv_style_set_flex_grow(self.as_raw_mut(), grow) };
	}

	pub fn set_grid_column_descriptors(&mut self, descriptors: &[i32]) {
		// Check if the descriptors array ends with GRID_TEMPLATE_LAST, and if not allocate a vec to
		// hold the descriptors and append GRID_TEMPLATE_LAST.
		let desc_array = if descriptors.last() == Some(&GRID_TEMPLATE_LAST) {
			None
		} else {
			let mut vec = alloc::vec::Vec::from(descriptors);
			vec.push(GRID_TEMPLATE_LAST);
			Some(vec)
		};

		unsafe {
			lvgl_sys::lv_style_set_grid_column_dsc_array(
				self.as_raw_mut(),
				desc_array
					.as_ref()
					.map_or(descriptors.as_ptr(), |v| v.as_ptr()),
			)
		};
	}

	pub fn set_grid_column_align(&mut self, align: GridAlign) {
		unsafe {
			lvgl_sys::lv_style_set_grid_column_align(
				self.as_raw_mut(),
				align as lvgl_sys::lv_grid_align_t,
			)
		};
	}

	pub fn set_grid_row_descriptors(&mut self, descriptors: &[i32]) {
		// Check if the descriptors array ends with GRID_TEMPLATE_LAST, and if not allocate a vec to
		// hold the descriptors and append GRID_TEMPLATE_LAST.
		let desc_array = if descriptors.last() == Some(&GRID_TEMPLATE_LAST) {
			None
		} else {
			let mut vec = alloc::vec::Vec::from(descriptors);
			vec.push(GRID_TEMPLATE_LAST);
			Some(vec)
		};

		unsafe {
			lvgl_sys::lv_style_set_grid_row_dsc_array(
				self.as_raw_mut(),
				desc_array
					.as_ref()
					.map_or(descriptors.as_ptr(), |v| v.as_ptr()),
			)
		};
	}

	pub fn set_grid_row_align(&mut self, align: GridAlign) {
		unsafe {
			lvgl_sys::lv_style_set_grid_row_align(
				self.as_raw_mut(),
				align as lvgl_sys::lv_grid_align_t,
			)
		};
	}

	pub fn set_grid_cell_column_pos(&mut self, column: i32) {
		unsafe { lvgl_sys::lv_style_set_grid_cell_column_pos(self.as_raw_mut(), column) };
	}

	pub fn set_grid_cell_x_align(&mut self, align: GridAlign) {
		unsafe {
			lvgl_sys::lv_style_set_grid_cell_x_align(
				self.as_raw_mut(),
				align as lvgl_sys::lv_grid_align_t,
			)
		};
	}

	pub fn set_grid_cell_column_span(&mut self, span: i32) {
		unsafe { lvgl_sys::lv_style_set_grid_cell_column_span(self.as_raw_mut(), span) };
	}

	pub fn set_grid_cell_row_pos(&mut self, row: i32) {
		unsafe { lvgl_sys::lv_style_set_grid_cell_row_pos(self.as_raw_mut(), row) };
	}

	pub fn set_grid_cell_y_align(&mut self, align: GridAlign) {
		unsafe {
			lvgl_sys::lv_style_set_grid_cell_y_align(
				self.as_raw_mut(),
				align as lvgl_sys::lv_grid_align_t,
			)
		};
	}

	pub fn set_grid_cell_row_span(&mut self, span: i32) {
		unsafe { lvgl_sys::lv_style_set_grid_cell_row_span(self.as_raw_mut(), span) };
	}
}

impl Clone for Style {
	fn clone(&self) -> Self {
		let mut new_style = Self::new();
		unsafe { lvgl_sys::lv_style_copy(new_style.as_raw_mut(), self.as_raw_ptr()) };
		new_style
	}
}
