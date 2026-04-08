use crate::{
	DISPLAY_HEIGHT,
	gui::{HEADER_HEIGHT, TAB_NAMES},
};
use lvgl::{
	event::EventCode,
	font::FONT_MONTSERRAT_20,
	misc::area::as_percent,
	widgets::{
		base::{Part, Widget},
		obj::AsRawObj,
		tab_view::TabView,
	},
};

pub mod electrical;
pub mod history;
pub mod overview;
pub mod settings;
pub mod tanks;

pub struct GuiTabs {
	#[allow(dead_code)]
	container: TabView,

	#[allow(dead_code)]
	overview: overview::GuiTabOverview,
	#[allow(dead_code)]
	electrical: electrical::GuiTabElectrical,
	#[allow(dead_code)]
	tanks: tanks::GuiTabTanks,
	#[allow(dead_code)]
	history: history::GuiTabHistory,
	#[allow(dead_code)]
	settings: settings::GuiTabSettings,
}

impl GuiTabs {
	pub fn new<F>(parent: &impl AsRawObj, on_tab_changed: F) -> Self
	where
		F: Fn(u32) + 'static,
	{
		let mut container = TabView::new(parent);
		container
			.set_pos(0, HEADER_HEIGHT)
			.set_size(as_percent(100), DISPLAY_HEIGHT - HEADER_HEIGHT)
			.set_tab_bar_position(lvgl::misc::area::Dir::Bottom);

		// Set tab text font to a larger size
		container
			.get_tab_bar()
			.set_style_text_font(&FONT_MONTSERRAT_20, Part::Main.into());

		// Register event handler for tab changes.
		container.add_event_cb(EventCode::ValueChanged, move |event| {
			if event.code() != EventCode::ValueChanged {
				return;
			}

			let active_index = event
				.target_obj()
				.try_cast::<TabView>()
				.map(|tv| tv.get_active_tab())
				.unwrap_or(0);

			on_tab_changed(active_index);
		});

		let overview = overview::GuiTabOverview::new(&container.add_tab(TAB_NAMES[0]));
		let electrical = electrical::GuiTabElectrical::new(&container.add_tab(TAB_NAMES[1]));
		let tanks = tanks::GuiTabTanks::new(&container.add_tab(TAB_NAMES[2]));
		let history = history::GuiTabHistory::new(&container.add_tab(TAB_NAMES[3]));
		let settings = settings::GuiTabSettings::new(&container.add_tab(TAB_NAMES[4]));

		container
			.set_active_tab(1, false)
			.expect("Failed to set active tab");

		Self {
			container,
			overview,
			electrical,
			tanks,
			history,
			settings,
		}
	}
}
