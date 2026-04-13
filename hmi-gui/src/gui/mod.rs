use crate::future::RtcTime;
use crate::gui::header::GuiHeader;
use crate::gui::tabs::GuiTabs;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::CStr;

pub mod components;
pub mod header;
pub mod tabs;

pub const HEADER_HEIGHT: i32 = 30;

pub const TAB_NAMES: [&'static CStr; 5] = [
	c"Overview",
	c"Electrical",
	c"Tanks",
	c"History",
	c"Settings",
];

pub struct Gui {
	#[allow(dead_code)]
	header: Rc<RefCell<GuiHeader>>,

	#[allow(dead_code)]
	tabs: GuiTabs,
}

impl Gui {
	pub fn new() -> Self {
		// Get active screen
		let screen = lvgl::active_screen().expect("Failed to get active screen");
		let header = Rc::new(RefCell::new(GuiHeader::new(&screen)));

		let header_for_tabs = header.clone();
		let tabs = GuiTabs::new(&screen, move |active_index| {
			let tab_name = TAB_NAMES
				.get(active_index as usize)
				.copied()
				.unwrap_or(c"Unknown");
			header_for_tabs.borrow_mut().set_tab_name(tab_name);
		});

		Self { header, tabs }
	}

	pub fn set_time(&mut self, time: &RtcTime) {
		self.header.borrow_mut().set_time(time);
	}
}
