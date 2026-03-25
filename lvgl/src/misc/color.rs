pub struct Opacity(u8);

impl Opacity {
	pub const fn from_u32(value: u32) -> Self {
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

	pub fn red(&self) -> u8 {
		self.0.red
	}

	pub fn green(&self) -> u8 {
		self.0.green
	}

	pub fn blue(&self) -> u8 {
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

#[allow(dead_code)]
pub struct Color16(lvgl_sys::lv_color16_t);

#[allow(dead_code)]
pub struct Color32(lvgl_sys::lv_color32_t);

#[allow(dead_code)]
pub struct ColorHsv(lvgl_sys::lv_color_hsv_t);

#[allow(dead_code)]
pub struct Color16a(lvgl_sys::lv_color16a_t);
