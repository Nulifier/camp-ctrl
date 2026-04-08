#pragma once

#include <lvgl.h>

enum Tank
{
	TANK_FRESH,
	TANK_GRAY,
	TANK_BLACK,
	TANK_PROPANE,
	TANK_COUNT
};

void gui_build_tab_tanks(lv_obj_t *tab);

void update_tank_level(enum Tank tank, int level_pct);
