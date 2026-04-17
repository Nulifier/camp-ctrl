pub struct Opacity(u8);

impl Opacity {
	pub(crate) const fn from_u32(value: u32) -> Self {
		Self(value as u8)
	}
}

impl From<Opacity> for lvgl_sys::lv_opa_t {
	fn from(opa: Opacity) -> Self {
		opa.0
	}
}

pub const OPA_TRANSPARENT: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_TRANSP);
pub const OPA_0: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_0);
pub const OPA_10: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_10);
pub const OPA_20: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_20);
pub const OPA_30: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_30);
pub const OPA_40: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_40);
pub const OPA_50: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_50);
pub const OPA_60: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_60);
pub const OPA_70: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_70);
pub const OPA_80: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_80);
pub const OPA_90: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_90);
pub const OPA_100: Opacity = Opacity::from_u32(lvgl_sys::_lv_opacity_level_t_LV_OPA_100);

pub struct Color(lvgl_sys::lv_color_t);

impl Color {
	pub(crate) fn as_raw(&self) -> lvgl_sys::lv_color_t {
		self.0
	}

	// pub(crate) fn as_raw_ptr(&self) -> *const lvgl_sys::lv_color_t {
	// 	&self.0
	// }

	// pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_color_t {
	// 	&mut self.0
	// }

	pub const fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
		Self(lvgl_sys::lv_color_t { red, green, blue })
	}

	pub const fn red(&self) -> u8 {
		self.0.red
	}

	pub const fn green(&self) -> u8 {
		self.0.green
	}

	pub const fn blue(&self) -> u8 {
		self.0.blue
	}

	pub fn set_red(&mut self, red: u8) {
		self.0.red = red;
	}

	pub fn set_green(&mut self, green: u8) {
		self.0.green = green;
	}

	pub fn set_blue(&mut self, blue: u8) {
		self.0.blue = blue;
	}
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Color16(lvgl_sys::lv_color16_t);

impl Color16 {
	pub fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
		Self(lvgl_sys::lv_color16_t {
			_bitfield_align_1: [],
			_bitfield_1: lvgl_sys::lv_color16_t::new_bitfield_1(
				(blue >> 3) as u16,
				(green >> 2) as u16,
				(red >> 3) as u16,
			),
		})
	}

	#[inline(always)]
	pub fn red(&self) -> u8 {
		self.0.red() as u8
	}

	#[inline(always)]
	pub fn green(&self) -> u8 {
		self.0.green() as u8
	}

	#[inline(always)]
	pub fn blue(&self) -> u8 {
		self.0.blue() as u8
	}

	#[inline(always)]
	pub fn set_red(&mut self, red: u8) {
		self.0.set_red(red as u16);
	}

	#[inline(always)]
	pub fn set_green(&mut self, green: u8) {
		self.0.set_green(green as u16);
	}

	#[inline(always)]
	pub fn set_blue(&mut self, blue: u8) {
		self.0.set_blue(blue as u16);
	}
}

#[allow(dead_code)]
pub struct Color32(lvgl_sys::lv_color32_t);

#[allow(dead_code)]
pub struct ColorHsv(lvgl_sys::lv_color_hsv_t);

#[allow(dead_code)]
pub struct Color16a(lvgl_sys::lv_color16a_t);
