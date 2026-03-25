use lvgl::{
	misc::area::{Align, as_percent},
	widgets::{
		base::Widget,
		obj::{AsRawObj, Obj},
	},
};

pub const DISPLAY_WIDTH: i32 = 800;
pub const DISPLAY_HEIGHT: i32 = 480;
pub const HEADER_HEIGHT: i32 = 30;

pub struct Gui {
	#[allow(dead_code)]
	header: GuiHeader,
}

impl Gui {
	pub fn new() -> Self {
		// Get active screen
		let screen = lvgl::active_screen().expect("Failed to get active screen");

		Self {
			header: GuiHeader::new(&screen),
		}
	}
}

pub struct GuiHeader {
	#[allow(dead_code)]
	container: Obj,
}

impl GuiHeader {
	pub fn new(parent: &impl AsRawObj) -> Self {
		let mut container = Obj::new(parent);
		container.set_size(as_percent(100), HEADER_HEIGHT);
		container.set_align(Align::TopMid);

		Self { container }
	}
}
