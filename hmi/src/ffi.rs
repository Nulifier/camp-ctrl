#[repr(C)]
pub struct UiSnapshot {
	pub tank_level_pct: i32,
	pub solar_watts: i32,
	pub charging: bool,
}

unsafe extern "C" {
	pub fn gui_init() -> i32;
}
