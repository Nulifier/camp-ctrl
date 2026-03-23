#include "tab_electrical.hpp"
#include "gui_conf.hpp"
#include "styles.hpp"
#include "widgets/metric_card.hpp"
#include <cstdio>

static const int32_t CARD_HEIGHT = 162;

struct
{
	lv_obj_t *root;
	MetricCard card_solar;
	MetricCard card_charger;
	MetricCard card_battery;
	MetricCard card_inverter;
	MetricCard card_dc;
	MetricCard card_ac;
} tab_electrical;

void tab::electrical::build(lv_obj_t *tab)
{
	// Create container
	tab_electrical.root = lv_obj_create(tab);
	lv_obj_add_style(tab_electrical.root, &styles::wrapper, LV_PART_MAIN);
	lv_obj_set_layout(tab_electrical.root, LV_LAYOUT_GRID);
	static const int32_t column_desc[] = {LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};
	static const int32_t row_desc[] = {LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};
	lv_obj_set_grid_dsc_array(tab_electrical.root, column_desc, row_desc);
	lv_obj_set_style_pad_row(tab_electrical.root, 30, LV_PART_MAIN);
	lv_obj_set_style_pad_column(tab_electrical.root, 60, LV_PART_MAIN);
	lv_obj_set_size(tab_electrical.root, LV_PCT(100), LV_PCT(100));
	lv_obj_center(tab_electrical.root);

	lv_area_t coords;

	// Origin
	lv_obj_update_layout(tab_electrical.root);
	lv_obj_get_coords(tab_electrical.root, &coords);
	printf("Container: (%d,%d) -> (%d,%d)\n", coords.x1, coords.y1, coords.x2, coords.y2);

	// Solar card
	tab_electrical.card_solar.create(tab_electrical.root, "Solar");
	lv_obj_set_grid_cell(tab_electrical.card_solar.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 0, 1,
						 LV_GRID_ALIGN_STRETCH, 0, 1);

	tab_electrical.card_solar.get_coords(&coords, true);
	printf("Solar card: (%d,%d) -> (%d,%d)\n", coords.x1, coords.y1, coords.x2, coords.y2);

	tab_electrical.card_solar.add_metric("Voltage", "108.5", MetricUnit::Volts);
	tab_electrical.card_solar.add_metric("Current", "12.3", MetricUnit::Amps);

	// Charger card
	tab_electrical.card_charger.create(tab_electrical.root, "Charger");
	lv_obj_set_grid_cell(tab_electrical.card_charger.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 0, 1,
						 LV_GRID_ALIGN_STRETCH, 1, 1);

	tab_electrical.card_charger.get_coords(&coords, true);
	printf("Charger card: (%d,%d) -> (%d,%d)\n", coords.x1, coords.y1, coords.x2, coords.y2);

	// Battery card
	tab_electrical.card_battery.create(tab_electrical.root, "Battery");
	lv_obj_set_grid_cell(tab_electrical.card_battery.get_wrapper(),
						 LV_GRID_ALIGN_STRETCH, 1, 1,
						 LV_GRID_ALIGN_STRETCH, 0, 1);

	tab_electrical.card_battery.get_coords(&coords, true);
	printf("Battery card: (%d,%d) -> (%d,%d)\n", coords.x1, coords.y1, coords.x2, coords.y2);

	// Container: (820,50) -> (1579,394)
	// Solar card: (820,50) -> (1032,207)
	// Charger card: (820,238) -> (1032,394)
	// Battery card: (1093,50) -> (1306,207)

	// Normalized
	// Solar card: (0,0) -> (212,157)
	// Charger card: (0,188) -> (212,344)
	// Battery card: (273,0) -> (486,157)

	// Create a line
	lv_obj_t *line1 = lv_line_create(tab_electrical.root);
	static lv_point_precise_t line_points1[] = {{212, 157 / 2 - 10}, {273, 157 / 2 - 10}};
	lv_line_set_points(line1, line_points1, 2);
	lv_obj_set_style_line_width(line1, 2, LV_PART_MAIN);
	lv_obj_set_style_line_color(line1, lv_palette_main(LV_PALETTE_GREY), LV_PART_MAIN);
	lv_obj_set_size(line1, LV_PCT(100), LV_PCT(100));

	lv_obj_t *line2 = lv_line_create(tab_electrical.root);
	static lv_point_precise_t line_points2[] = {
		{212, (188 + (344 - 188) / 2)},
		{212 + (273 - 212) / 2, (188 + (344 - 188) / 2)},
		{212 + (273 - 212) / 2, 157 / 2 + 10},
		{273, 157 / 2 + 10}};
	lv_line_set_points(line2, line_points2, 4);
	lv_obj_set_style_line_width(line2, 2, LV_PART_MAIN);
	lv_obj_set_style_line_color(line2, lv_palette_main(LV_PALETTE_GREY), LV_PART_MAIN);
	lv_obj_set_size(line2, LV_PCT(100), LV_PCT(100));
}
