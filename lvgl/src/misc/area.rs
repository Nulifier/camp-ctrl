use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Align {
	Default = lvgl_sys::lv_align_t_LV_ALIGN_DEFAULT,
	TopLeft = lvgl_sys::lv_align_t_LV_ALIGN_TOP_LEFT,
	TopMid = lvgl_sys::lv_align_t_LV_ALIGN_TOP_MID,
	TopRight = lvgl_sys::lv_align_t_LV_ALIGN_TOP_RIGHT,
	BottomLeft = lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_LEFT,
	BottomMid = lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_MID,
	BottomRight = lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_RIGHT,
	LeftMid = lvgl_sys::lv_align_t_LV_ALIGN_LEFT_MID,
	RightMid = lvgl_sys::lv_align_t_LV_ALIGN_RIGHT_MID,
	Center = lvgl_sys::lv_align_t_LV_ALIGN_CENTER,

	OutTopLeft = lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_LEFT,
	OutTopMid = lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_MID,
	OutTopRight = lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_RIGHT,
	OutBottomLeft = lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_LEFT,
	OutBottomMid = lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_MID,
	OutBottomRight = lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_RIGHT,
	OutLeftTop = lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_TOP,
	OutLeftMid = lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_MID,
	OutLeftBottom = lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_BOTTOM,
	OutRightTop = lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_TOP,
	OutRightMid = lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_MID,
	OutRightBottom = lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_BOTTOM,
}

#[derive(Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Dir {
	None = lvgl_sys::lv_dir_t_LV_DIR_NONE,
	Left = lvgl_sys::lv_dir_t_LV_DIR_LEFT,
	Right = lvgl_sys::lv_dir_t_LV_DIR_RIGHT,
	Top = lvgl_sys::lv_dir_t_LV_DIR_TOP,
	Bottom = lvgl_sys::lv_dir_t_LV_DIR_BOTTOM,
	Hor = lvgl_sys::lv_dir_t_LV_DIR_HOR,
	Ver = lvgl_sys::lv_dir_t_LV_DIR_VER,
	All = lvgl_sys::lv_dir_t_LV_DIR_ALL,
}

#[derive(Copy, Clone)]
pub struct Point(lvgl_sys::lv_point_t);

impl Point {
	// pub(crate) fn from_raw(raw: *const lvgl_sys::lv_point_t) -> Self {
	// 	Self(unsafe { *raw })
	// }

	#[inline(always)]
	pub(crate) fn as_raw(&self) -> *const lvgl_sys::lv_point_t {
		&self.0 as *const lvgl_sys::lv_point_t
	}

	#[inline(always)]
	pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_point_t {
		&mut self.0 as *mut lvgl_sys::lv_point_t
	}

	#[inline(always)]
	pub fn x(&self) -> i32 {
		self.0.x
	}

	#[inline(always)]
	pub fn y(&self) -> i32 {
		self.0.y
	}

	pub fn new(x: i32, y: i32) -> Self {
		Self(lvgl_sys::lv_point_t { x, y })
	}

	pub fn set(&mut self, x: i32, y: i32) {
		self.0.x = x;
		self.0.y = y;
	}
}

impl From<PointPrecise> for Point {
	fn from(point: PointPrecise) -> Self {
		Self(unsafe { lvgl_sys::lv_point_from_precise(point.as_raw()) })
	}
}

#[derive(Copy, Clone)]
pub struct PointPrecise(lvgl_sys::lv_point_precise_t);

impl PointPrecise {
	// pub(crate) fn from_raw(raw: *const lvgl_sys::lv_point_precise_t) -> Self {
	// 	Self(unsafe { *raw })
	// }

	#[inline(always)]
	pub(crate) fn as_raw(&self) -> *const lvgl_sys::lv_point_precise_t {
		&self.0 as *const lvgl_sys::lv_point_precise_t
	}

	// #[inline(always)]
	// pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_point_precise_t {
	// 	&mut self.0 as *mut lvgl_sys::lv_point_precise_t
	// }

	#[inline(always)]
	pub fn x(&self) -> i32 {
		self.0.x
	}

	#[inline(always)]
	pub fn y(&self) -> i32 {
		self.0.y
	}

	pub fn new(x: i32, y: i32) -> Self {
		Self(lvgl_sys::lv_point_precise_t { x, y })
	}

	pub fn set(&mut self, x: i32, y: i32) {
		self.0.x = x;
		self.0.y = y;
	}
}

impl From<Point> for PointPrecise {
	fn from(point: Point) -> Self {
		Self(unsafe { lvgl_sys::lv_point_to_precise(point.as_raw()) })
	}
}

#[derive(Copy, Clone)]
pub struct Area(lvgl_sys::lv_area_t);

impl Area {
	pub(crate) fn from_raw(raw: *const lvgl_sys::lv_area_t) -> Self {
		Self(unsafe { *raw })
	}

	#[inline(always)]
	pub(crate) fn as_raw(&self) -> *const lvgl_sys::lv_area_t {
		&self.0 as *const lvgl_sys::lv_area_t
	}

	#[inline(always)]
	pub(crate) fn as_raw_mut(&mut self) -> *mut lvgl_sys::lv_area_t {
		&mut self.0 as *mut lvgl_sys::lv_area_t
	}

	#[inline(always)]
	pub fn x1(&self) -> i32 {
		self.0.x1
	}

	#[inline(always)]
	pub fn y1(&self) -> i32 {
		self.0.y1
	}

	#[inline(always)]
	pub fn x2(&self) -> i32 {
		self.0.x2
	}

	#[inline(always)]
	pub fn y2(&self) -> i32 {
		self.0.y2
	}

	pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
		Self(lvgl_sys::lv_area_t { x1, y1, x2, y2 })
	}

	pub fn set(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
		self.0.x1 = x1;
		self.0.y1 = y1;
		self.0.x2 = x2;
		self.0.y2 = y2;
	}

	pub fn width(&self) -> i32 {
		unsafe { lvgl_sys::lv_area_get_width(self.as_raw()) }
	}

	pub fn height(&self) -> i32 {
		unsafe { lvgl_sys::lv_area_get_height(self.as_raw()) }
	}

	pub fn set_width(&mut self, width: i32) {
		unsafe { lvgl_sys::lv_area_set_width(self.as_raw_mut(), width) }
	}

	pub fn set_height(&mut self, height: i32) {
		unsafe { lvgl_sys::lv_area_set_height(self.as_raw_mut(), height) }
	}

	/// Returns the size of the area (width * height).
	pub fn size(&self) -> u32 {
		unsafe { lvgl_sys::lv_area_get_size(self.as_raw()) }
	}

	pub fn increase(&mut self, inc_x: i32, inc_y: i32) {
		unsafe { lvgl_sys::lv_area_increase(self.as_raw_mut(), inc_x, inc_y) }
	}

	pub fn move_by(&mut self, delta_x: i32, delta_y: i32) {
		unsafe { lvgl_sys::lv_area_move(self.as_raw_mut(), delta_x, delta_y) }
	}

	pub fn align(&mut self, base: &Area, align: Align, offset_x: i32, offset_y: i32) {
		unsafe {
			lvgl_sys::lv_area_align(
				self.as_raw_mut(),
				base.as_raw().cast_mut(),
				align.into(),
				offset_x,
				offset_y,
			)
		}
	}
}

impl Default for Area {
	fn default() -> Self {
		Self(lvgl_sys::lv_area_t {
			x1: 0,
			y1: 0,
			x2: 0,
			y2: 0,
		})
	}
}

/// Converts a percentage value (0-1000) to the internal representation used by LVGL.
pub fn as_percent(value: i32) -> i32 {
	unsafe { lvgl_sys::lv_pct(value) }
}

/// Converts a percentage value (0-1000) to pixels based on the given base value.
pub fn percent_as_pixel(value: i32, base: i32) -> i32 {
	unsafe { lvgl_sys::lv_pct_to_px(value, base) }
}
