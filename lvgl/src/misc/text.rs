use bitflags::bitflags;

bitflags! {
	pub struct TextFlag: lvgl_sys::lv_text_flag_t {
		const NONE = lvgl_sys::lv_text_flag_t_LV_TEXT_FLAG_NONE;
		const EXPAND = lvgl_sys::lv_text_flag_t_LV_TEXT_FLAG_EXPAND;
		const FIT = lvgl_sys::lv_text_flag_t_LV_TEXT_FLAG_FIT;
		const BREAK_ALL = lvgl_sys::lv_text_flag_t_LV_TEXT_FLAG_BREAK_ALL;
		const RECOLOR = lvgl_sys::lv_text_flag_t_LV_TEXT_FLAG_RECOLOR;
	}
}

#[repr(u32)]
pub enum TextAlign {
	Auto = lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_AUTO,
	Left = lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_LEFT,
	Center = lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_CENTER,
	Right = lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_RIGHT,
}
