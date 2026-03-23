#[repr(C)]
pub struct UiSnapshot {
	pub tank_level_pct: i32,
	pub solar_watts: i32,
	pub charging: bool,
}

unsafe extern "C" {
	pub fn gui_init() -> i32;
	pub fn gui_tick_inc(ms: u32);
	pub fn gui_task_handler();
	pub fn gui_apply_snapshot(snapshot: *const UiSnapshot);
}
