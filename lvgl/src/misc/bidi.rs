#[repr(u32)]
pub enum BaseDir {
	Ltr = lvgl_sys::lv_base_dir_t_LV_BASE_DIR_LTR,
	Rtl = lvgl_sys::lv_base_dir_t_LV_BASE_DIR_RTL,
	Auto = lvgl_sys::lv_base_dir_t_LV_BASE_DIR_AUTO,
	Neutral = lvgl_sys::lv_base_dir_t_LV_BASE_DIR_NEUTRAL,
	Weak = lvgl_sys::lv_base_dir_t_LV_BASE_DIR_WEAK,
}
