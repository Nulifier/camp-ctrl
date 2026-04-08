use core::{marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

use crate::{AsRaw, AsRawMut};

pub mod symbols;

#[repr(u32)]
pub enum FontGlyphFormat {
	None = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_NONE,
	A1 = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_A1,
	A2 = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_A2,
	A3 = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_A3,
	A4 = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_A4,
	A8 = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_A8,
	Image = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_IMAGE,
	Vector = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_VECTOR,
	Svg = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_SVG,
	Custom = lvgl_sys::lv_font_glyph_format_t_LV_FONT_GLYPH_FORMAT_CUSTOM,
}

#[repr(u32)]
pub enum FontSubpixel {
	None = lvgl_sys::lv_font_subpx_t_LV_FONT_SUBPX_NONE,
	Hor = lvgl_sys::lv_font_subpx_t_LV_FONT_SUBPX_HOR,
	Ver = lvgl_sys::lv_font_subpx_t_LV_FONT_SUBPX_VER,
	Both = lvgl_sys::lv_font_subpx_t_LV_FONT_SUBPX_BOTH,
}

#[repr(u32)]
pub enum FontKerning {
	Normal = lvgl_sys::lv_font_kerning_t_LV_FONT_KERNING_NORMAL,
	None = lvgl_sys::lv_font_kerning_t_LV_FONT_KERNING_NONE,
}

impl From<FontKerning> for u32 {
	fn from(kerning: FontKerning) -> Self {
		kerning as u32
	}
}

pub struct FontGlyphDesc(lvgl_sys::lv_font_glyph_dsc_t);

impl FontGlyphDesc {
	pub fn new() -> Self {
		Self(unsafe { MaybeUninit::zeroed().assume_init() })
	}

	pub fn font(&self) -> ConstFontRef<'_> {
		ConstFontRef::from_raw(self.0.resolved_font)
	}

	pub unsafe fn release_draw_data(&mut self) {
		unsafe { lvgl_sys::lv_font_glyph_release_draw_data(self.as_raw_mut()) };
	}
}

impl AsRawMut<lvgl_sys::lv_font_glyph_dsc_t> for FontGlyphDesc {
	fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_font_glyph_dsc_t {
		&mut self.0 as *mut lvgl_sys::lv_font_glyph_dsc_t
	}
}

pub trait Font: AsRaw<lvgl_sys::lv_font_t> {
	// SAFETY: The caller must ensure that the draw data for glyph_desc is released after.
	unsafe fn glyph_desc(
		&self,
		glyph_desc: &mut FontGlyphDesc,
		letter: u32,
		next_letter: u32,
	) -> bool {
		unsafe {
			lvgl_sys::lv_font_get_glyph_dsc(
				self.as_raw(),
				glyph_desc.as_raw_mut(),
				letter,
				next_letter,
			)
		}
	}

	fn glyph_width(&self, letter: u32, next_letter: u32) -> u16 {
		unsafe { lvgl_sys::lv_font_get_glyph_width(self.as_raw(), letter, next_letter) }
	}

	fn line_height(&self) -> i32 {
		unsafe { lvgl_sys::lv_font_get_line_height(self.as_raw()) }
	}

	fn has_static_bitmap(&self) -> bool {
		unsafe { lvgl_sys::lv_font_has_static_bitmap(self.as_raw()) }
	}
}

pub trait FontMut: Font + AsRawMut<lvgl_sys::lv_font_t> {
	fn set_kerning(&mut self, kerning: FontKerning) {
		unsafe { lvgl_sys::lv_font_set_kerning(self.as_raw_mut(), kerning.into()) };
	}
}

pub struct FontRef<'a> {
	raw: NonNull<lvgl_sys::lv_font_t>,
	_marker: PhantomData<&'a lvgl_sys::lv_font_t>,
}

impl FontRef<'_> {
	#[allow(dead_code)]
	pub(crate) const fn from_raw(raw: *mut lvgl_sys::lv_font_t) -> Self {
		Self {
			raw: NonNull::new(raw).expect("Received null pointer from LVGL"),
			_marker: PhantomData,
		}
	}
}

impl AsRaw<lvgl_sys::lv_font_t> for FontRef<'_> {
	fn as_raw(&self) -> *const lvgl_sys::lv_font_t {
		self.raw.as_ptr()
	}
}

impl AsRawMut<lvgl_sys::lv_font_t> for FontRef<'_> {
	fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_font_t {
		self.raw.as_ptr()
	}
}

impl Font for FontRef<'_> {}
impl FontMut for FontRef<'_> {}

pub struct ConstFontRef<'a> {
	raw: *const lvgl_sys::lv_font_t, // Guaranteed to be non-null by constructor
	_marker: PhantomData<&'a lvgl_sys::lv_font_t>,
}

impl<'a> ConstFontRef<'a> {
	pub(crate) const fn from_raw(raw: *const lvgl_sys::lv_font_t) -> Self {
		if raw.is_null() {
			panic!("Received null pointer from LVGL");
		}
		Self {
			raw,
			_marker: PhantomData,
		}
	}
}

impl AsRaw<lvgl_sys::lv_font_t> for ConstFontRef<'_> {
	fn as_raw(&self) -> *const lvgl_sys::lv_font_t {
		self.raw
	}
}

impl Font for ConstFontRef<'_> {}

impl Default for ConstFontRef<'_> {
	fn default() -> Self {
		let raw = unsafe { lvgl_sys::lv_font_get_default() };
		if raw.is_null() {
			panic!("LVGL returned null pointer for default font");
		}
		Self {
			raw,
			_marker: PhantomData,
		}
	}
}

pub const FONT_MONTSERRAT_14: ConstFontRef<'static> =
	unsafe { ConstFontRef::from_raw(&lvgl_sys::lv_font_montserrat_14) };

pub const FONT_MONTSERRAT_20: ConstFontRef<'static> =
	unsafe { ConstFontRef::from_raw(&lvgl_sys::lv_font_montserrat_20) };
