use core::ffi::CStr;
use lvgl::{
	font::{FONT_MONTSERRAT_20, symbols::SYMBOL_AUDIO},
	layouts::grid::{GRID_CONTENT, GRID_TEMPLATE_LAST, GridAlign, grid_fr},
	widgets::{
		base::{LAYOUT_GRID, Part, Widget},
		label::Label,
		obj::Obj,
	},
};

pub enum MetricUnit {
	None,
	Percent,
	Amps,
	Volts,
	Watts,
}

pub struct MetricCard {
	#[allow(dead_code)]
	card: Obj,
	#[allow(dead_code)]
	title_label: Label,
	#[allow(dead_code)]
	icon_label: Label,
	#[allow(dead_code)]
	metric_labels: [Option<Label>; 3],
	#[allow(dead_code)]
	unit_labels: [Option<Label>; 3],
}

impl MetricCard {
	pub fn new(parent: &Obj, title: &CStr) -> Self {
		static COL_DESC: [i32; 4] = [GRID_CONTENT, grid_fr(1), 4, GRID_TEMPLATE_LAST];
		static ROW_DESC: [i32; 4] = [grid_fr(1), grid_fr(1), grid_fr(1), GRID_TEMPLATE_LAST];

		let mut card = Obj::new(parent);
		card.set_layout(LAYOUT_GRID)
			.set_grid_desc_array(&COL_DESC, &ROW_DESC)
			.expect("Invalid grid descriptor");

		let mut title_label = Label::new(&card);
		title_label
			.set_text(title)
			.set_style_text_font(&FONT_MONTSERRAT_20, Part::Main.into())
			.set_grid_cell(GridAlign::Start, 0, 1, GridAlign::Start, 0, 1);

		let mut icon_label = Label::new(&card);
		icon_label
			.set_text_static(SYMBOL_AUDIO)
			.set_style_text_font(&FONT_MONTSERRAT_20, Part::Main.into());

		Self {
			card,
			title_label,
			icon_label,
			metric_labels: [None, None, None],
			unit_labels: [None, None, None],
		}
	}
}
