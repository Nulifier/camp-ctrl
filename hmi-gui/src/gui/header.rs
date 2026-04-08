use super::{HEADER_HEIGHT, TAB_NAMES};
use crate::{future::RtcTime, misc::fmt::SliceWriter};
use core::fmt::Write;
use lvgl::{
	font::symbols::{SYMBOL_BATTERY_FULL, SYMBOL_WARNING},
	layouts::flex::{FlexAlign, FlexFlow},
	misc::{
		area::{Align, SIZE_CONTENT, as_percent},
		palette::{PaletteColor, material_color},
		text::TextAlign,
	},
	widgets::{
		base::{ObjFlags, Part, Widget},
		label::Label,
		obj::{AsRawObj, Obj},
	},
};

pub struct GuiHeader {
	#[allow(dead_code)]
	container: Obj,

	time_label: Label,

	#[allow(dead_code)]
	tab_label: Label,

	#[allow(dead_code)]
	icon_container: Obj,

	#[allow(dead_code)]
	icon_warning: Label,

	#[allow(dead_code)]
	icon_battery: Label,
}

impl GuiHeader {
	pub fn new(parent: &impl AsRawObj) -> Self {
		// Create status header
		let mut container = Obj::new(parent);
		container
			.set_size(as_percent(100), HEADER_HEIGHT)
			.set_align(Align::TopMid)
			.set_style_pad_all(5, Part::Main.into())
			.set_style_radius(0, Part::Main.into())
			.set_style_border_side(lvgl::style::BorderSide::BOTTOM, Part::Main.into());

		// Current time on the left
		let mut time_label = Label::new(&container);
		time_label
			.set_text_static(c"12:00 AM")
			.align(Align::LeftMid, 0, 0);

		// Tab name in the middle
		let mut tab_label = Label::new(&container);
		tab_label
			.set_text_static(TAB_NAMES[0])
			.set_width(as_percent(100))
			.set_style_text_align(TextAlign::Center, Part::Main.into())
			.align(Align::Center, 0, 0);
		// Skipped moving this to the foreground as it isn't natively supported in v9

		// Icons on the right
		let mut icon_container = Obj::new(&container);
		icon_container
			.remove_style_all()
			.set_style_pad_gap(6, Part::Main.into())
			.set_flex_flow(FlexFlow::Row)
			.set_flex_align(FlexAlign::End, FlexAlign::Center, FlexAlign::Center)
			.align(Align::RightMid, 0, 0)
			.set_size(SIZE_CONTENT, SIZE_CONTENT)
			.remove_flag(ObjFlags::SCROLLABLE);

		let mut icon_warning = Label::new(&icon_container);
		icon_warning
			.set_text_static(SYMBOL_WARNING)
			.set_style_text_color(material_color(PaletteColor::Amber), Part::Main.into());

		let mut icon_battery = Label::new(&icon_container);
		icon_battery.set_text_static(SYMBOL_BATTERY_FULL);

		Self {
			container,
			time_label,
			tab_label,
			icon_container,
			icon_warning,
			icon_battery,
		}
	}

	pub fn set_time(&mut self, time: &RtcTime) {
		// Create buffer large enough for "12:00 AM\0"
		let mut buffer = [0u8; 9];
		let mut writer = SliceWriter::new(&mut buffer);
		write!(writer, "{}", time.display("%H:%M %p")).expect("Failed to format time");
		self.time_label.set_text(writer.as_cstr());
	}

	pub fn set_tab_name(&mut self, tab_name: &'static core::ffi::CStr) {
		self.tab_label.set_text_static(tab_name);
	}
}
