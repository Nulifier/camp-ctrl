#include "tab_tanks.hpp"
#include "gui_conf.hpp"
// #include <stdio.h>
#include "widgets/tank_vert.hpp"

static const lv_color_t COLOR_FRESH_TANK = COLOR_LIGHT_BLUE;
static const lv_color_t COLOR_GRAY_TANK = COLOR_GREY;
static const lv_color_t COLOR_BLACK_TANK = COLOR_PURPLE;
static const lv_color_t COLOR_PROPANE_TANK = COLOR_KELLY;

// Style for a wrapper that has no padding, or border
static lv_style_t style_wrapper;

struct
{
	lv_obj_t *root;
	lv_obj_t *buttons;

	VerticalTankWidget fresh_water;
	VerticalTankWidget gray_water;
	VerticalTankWidget black_water;
#ifdef ENABLE_PROPANE_TANK
	VerticalTankWidget propane;
#endif
} tab_tank;

void gui_build_tab_tanks(lv_obj_t *tab)
{
	lv_memzero(&tab_tank, sizeof(tab_tank));

	// Build wrapper style
	lv_style_init(&style_wrapper);
	lv_style_set_size(&style_wrapper, LV_PCT(100), LV_PCT(100));
	lv_style_set_pad_all(&style_wrapper, 0);
	lv_style_set_margin_all(&style_wrapper, 0);
	lv_style_set_border_width(&style_wrapper, 0);
	lv_style_set_bg_opa(&style_wrapper, LV_OPA_TRANSP);

	tab_tank.root = lv_obj_create(tab);
	lv_obj_add_style(tab_tank.root, &style_wrapper, LV_PART_MAIN);
	lv_obj_set_layout(tab_tank.root, LV_LAYOUT_GRID);
	static const int32_t column_desc[] = {LV_GRID_CONTENT, LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};
	static const int32_t row_desc[] = {LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};
	lv_obj_set_grid_dsc_array(tab_tank.root, column_desc, row_desc);

	tab_tank.buttons = lv_buttonmatrix_create(tab_tank.root);
	static const char *btn_map[] = {"Pump", NULL};
	lv_buttonmatrix_set_map(tab_tank.buttons, btn_map);
	static const lv_buttonmatrix_ctrl_t btn_ctrl_map[] = {LV_BUTTONMATRIX_CTRL_CHECKABLE};
	lv_buttonmatrix_set_ctrl_map(tab_tank.buttons, btn_ctrl_map);
	lv_obj_set_grid_cell(tab_tank.buttons,
						 LV_GRID_ALIGN_STRETCH, 0, 1,
						 LV_GRID_ALIGN_STRETCH, 0, 1);

	tab_tank.fresh_water.create(tab_tank.root, "Fresh", COLOR_FRESH_TANK);
	lv_obj_set_grid_cell(tab_tank.fresh_water.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 1, 1,
						 LV_GRID_ALIGN_CENTER, 0, 1);

	tab_tank.gray_water.create(tab_tank.root, "Gray", COLOR_GRAY_TANK);
	lv_obj_set_grid_cell(tab_tank.gray_water.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 2, 1,
						 LV_GRID_ALIGN_CENTER, 0, 1);

	tab_tank.black_water.create(tab_tank.root, "Black", COLOR_BLACK_TANK);
	lv_obj_set_grid_cell(tab_tank.black_water.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 3, 1,
						 LV_GRID_ALIGN_CENTER, 0, 1);

#ifdef ENABLE_PROPANE_TANK
	tab_tank.propane.create(tab_tank.root, "Propane", COLOR_PROPANE_TANK);
	lv_obj_set_grid_cell(tab_tank.propane.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 4, 1,
						 LV_GRID_ALIGN_CENTER, 0, 1);
#endif
}

void update_tank_level(enum Tank tank, int level_pct)
{
	switch (tank)
	{
	case TANK_FRESH:
		tab_tank.fresh_water.set_level(level_pct);
		break;
	case TANK_GRAY:
		tab_tank.gray_water.set_level(level_pct);
		break;
	case TANK_BLACK:
		tab_tank.black_water.set_level(level_pct);
		break;
#ifdef ENABLE_PROPANE_TANK
	case TANK_PROPANE:
		tab_tank.propane.set_level(level_pct);
		break;
#endif
	default:
		return;
	}
}
