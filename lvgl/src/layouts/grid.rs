#[repr(u32)]
pub enum GridAlign {
	Start = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_START,
	Center = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_CENTER,
	End = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_END,
	Stretch = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
	SpaceEvenly = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_EVENLY,
	SpaceAround = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_AROUND,
	SpaceBetween = lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_BETWEEN,
}

pub const GRID_CONTENT: i32 = lvgl_sys::LV_GRID_CONTENT as i32;
pub const GRID_TEMPLATE_LAST: i32 = lvgl_sys::LV_GRID_TEMPLATE_LAST as i32;

pub fn grid_fr(x: u8) -> i32 {
	unsafe { lvgl_sys::lv_grid_fr(x) }
}
