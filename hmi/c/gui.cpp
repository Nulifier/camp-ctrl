#include "gui.hpp"
#include "lvgl.h"
#include "gui_conf.hpp"
#include "tab_overview.hpp"
#include "tab_electrical.hpp"
#include "tab_tanks.hpp"
#include "styles.hpp"

#define BACKEND_SDL

#ifdef BACKEND_SDL
#include "drivers/sdl/lv_sdl_window.h"
#include "drivers/sdl/lv_sdl_mouse.h"
#endif
#include <stdio.h>

static lv_display_t *display;

static struct
{
	lv_obj_t *container;
	lv_obj_t *label_time;
	lv_obj_t *label_tab;
	lv_obj_t *icon_container;
	lv_obj_t *icon_warning;
	lv_obj_t *icon_battery;
} header;

static lv_obj_t *tab_view;

static struct
{
	lv_obj_t *overview;
	lv_obj_t *electrical;
	lv_obj_t *tanks;
	lv_obj_t *history;
	lv_obj_t *settings;
} tabs;

void gui_build();
void gui_build_header(lv_obj_t *scr);
void gui_build_tabs(lv_obj_t *scr);
void gui_build_tab_history(lv_obj_t *tab);
void gui_build_tab_settings(lv_obj_t *tab);

// Event handlers
void on_tab_changed(lv_event_t *e)
{
	lv_event_code_t code = lv_event_get_code(e);
	if (code == LV_EVENT_VALUE_CHANGED)
	{
		lv_obj_t *tabview = static_cast<lv_obj_t *>(lv_event_get_target(e));
		uint32_t tab_idx = lv_tabview_get_tab_act(tabview);
		const char *tab_name = TAB_NAMES[tab_idx];
		lv_label_set_text(header.label_tab, tab_name);
	}
}

void gui_build()
{
	lv_obj_t *scr = lv_scr_act();

	gui_build_header(scr);
	gui_build_tabs(scr);
}

void gui_build_header(lv_obj_t *scr)
{
	// Create status header
	header.container = lv_obj_create(scr);
	lv_obj_set_size(header.container, lv_pct(100), HEADER_HEIGHT);
	lv_obj_set_align(header.container, LV_ALIGN_TOP_MID);
	lv_obj_set_style_pad_all(header.container, 5, LV_PART_MAIN);						 // No padding
	lv_obj_set_style_radius(header.container, 0, LV_PART_MAIN);							 // Square corners
	lv_obj_set_style_border_side(header.container, LV_BORDER_SIDE_BOTTOM, LV_PART_MAIN); // Border on bottom only

	// Current time on the left
	header.label_time = lv_label_create(header.container);
	lv_label_set_text(header.label_time, "12:00 AM");
	lv_obj_align(header.label_time, LV_ALIGN_LEFT_MID, 0, 0);

	// Tab name in the middle
	header.label_tab = lv_label_create(header.container);
	lv_label_set_text(header.label_tab, TAB_NAMES[0]);
	lv_obj_set_width(header.label_tab, lv_pct(100));
	lv_obj_set_style_text_align(header.label_tab, LV_TEXT_ALIGN_CENTER, LV_PART_MAIN);
	lv_obj_align(header.label_tab, LV_ALIGN_CENTER, 0, 0);
	lv_obj_move_foreground(header.label_tab);

	// Icons on the right
	header.icon_container = lv_obj_create(header.container);
	lv_obj_remove_style_all(header.icon_container);
	lv_obj_set_style_pad_gap(header.icon_container, 6, LV_PART_MAIN);
	lv_obj_set_flex_flow(header.icon_container, LV_FLEX_FLOW_ROW);
	lv_obj_set_flex_align(header.icon_container, LV_FLEX_ALIGN_END, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER);
	lv_obj_align(header.icon_container, LV_ALIGN_RIGHT_MID, 0, 0);
	lv_obj_set_size(header.icon_container, LV_SIZE_CONTENT, LV_SIZE_CONTENT);
	lv_obj_clear_flag(header.icon_container, LV_OBJ_FLAG_SCROLLABLE);

	header.icon_warning = lv_label_create(header.icon_container);
	lv_label_set_text(header.icon_warning, LV_SYMBOL_WARNING);
	lv_obj_set_style_text_color(header.icon_warning, lv_palette_main(LV_PALETTE_AMBER), LV_PART_MAIN);

	header.icon_battery = lv_label_create(header.icon_container);
	lv_label_set_text(header.icon_battery, LV_SYMBOL_BATTERY_FULL);
}

void gui_build_tabs(lv_obj_t *scr)
{
	tab_view = lv_tabview_create(scr);
	lv_obj_set_pos(tab_view, 0, HEADER_HEIGHT);
	lv_obj_set_size(tab_view, lv_pct(100), DISPLAY_HEIGHT - HEADER_HEIGHT);
	lv_tabview_set_tab_bar_position(tab_view, LV_DIR_BOTTOM);

	// Set tab text font to a larger size
	lv_obj_t *tab_btns = lv_tabview_get_tab_bar(tab_view);
	lv_obj_set_style_text_font(tab_btns, LARGE_FONT, LV_PART_MAIN);

	// Register event handler for tab changes
	lv_obj_add_event(tab_view, on_tab_changed, LV_EVENT_VALUE_CHANGED, NULL);

	tabs.overview = lv_tabview_add_tab(tab_view, TAB_NAMES[0]);
	gui_build_tab_overview(tabs.overview);

	tabs.electrical = lv_tabview_add_tab(tab_view, TAB_NAMES[1]);
	tab::electrical::build(tabs.electrical);

	tabs.tanks = lv_tabview_add_tab(tab_view, TAB_NAMES[2]);
	gui_build_tab_tanks(tabs.tanks);

	tabs.history = lv_tabview_add_tab(tab_view, TAB_NAMES[3]);
	// gui_build_tab_history(tabs.history);

	tabs.settings = lv_tabview_add_tab(tab_view, TAB_NAMES[4]);
	// gui_build_tab_settings(tabs.settings);

	lv_tabview_set_active(tab_view, 1, LV_ANIM_OFF);
}

int gui_init(void)
{
	lv_init();
	styles::init();

#ifdef BACKEND_SDL
	// Create display
	display = lv_sdl_window_create(DISPLAY_WIDTH, DISPLAY_HEIGHT);
	if (!display)
	{
		fprintf(stderr, "Failed to create SDL window\n");
		return -1;
	}

	// Create input devices
	lv_sdl_mouse_create();
#endif

	lv_display_set_default(display);

	gui_build();

	update_tank_level(TANK_FRESH, 75);
	update_tank_level(TANK_GRAY, 50);
	update_tank_level(TANK_BLACK, 25);
	update_tank_level(TANK_PROPANE, 90);

	return 0;
}

void gui_tick_inc(uint32_t ms)
{
	lv_tick_inc(ms);
}

void gui_task_handler(void)
{
	lv_task_handler();
}

void gui_apply_snapshot(const UiSnapshot *snapshot)
{
	// char buf[32];
	// snprintf(buf, sizeof(buf), "%d W", snapshot->solar_watts);
	// lv_label_set_text(label_power, buf);

	// lv_bar_set_value(bar_tank, snapshot->tank_level_pct, LV_ANIM_OFF);

	// if (snapshot->charging) {
	// 	lv_obj_add_state(btn_reset, LV_STATE_DISABLED);
	// } else {
	// 	lv_obj_clear_state(btn_reset, LV_STATE_DISABLED);
	// }
}
