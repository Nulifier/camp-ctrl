#[repr(u32)]
pub enum FlexAlign {
	Start = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_START,
	End = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_END,
	Center = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_CENTER,
	SpaceEvenly = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_EVENLY,
	SpaceAround = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_AROUND,
	SpaceBetween = lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_BETWEEN,
}

#[repr(u32)]
pub enum FlexFlow {
	Row = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW,
	Column = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN,
	RowWrap = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP,
	RowReverse = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_REVERSE,
	RowWrapReverse = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP_REVERSE,
	ColumnWrap = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP,
	ColumnReverse = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_REVERSE,
	ColumnWrapReverse = lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP_REVERSE,
}
