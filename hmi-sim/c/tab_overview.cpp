#include "tab_overview.hpp"
#include "gui_conf.hpp"

lv_obj_t *make_card(lv_obj_t *parent, const char *title)
{
	// Create card
	lv_obj_t *card = lv_obj_create(parent);
	lv_obj_set_style_radius(card, 12, LV_PART_MAIN);
	lv_obj_set_style_pad_all(card, 12, LV_PART_MAIN);
	lv_obj_set_style_border_width(card, 2, LV_PART_MAIN);
	lv_obj_set_style_bg_opa(card, LV_OPA_COVER, LV_PART_MAIN);

	// Card internals use flex
	lv_obj_set_layout(card, LV_LAYOUT_FLEX);
	lv_obj_set_flex_flow(card, LV_FLEX_FLOW_COLUMN);
	lv_obj_set_flex_align(card, LV_FLEX_ALIGN_START, LV_FLEX_ALIGN_START, LV_FLEX_ALIGN_START);

	lv_obj_t *label = lv_label_create(card);
	lv_label_set_text(label, title);
	lv_obj_set_style_text_font(label, LARGE_FONT, LV_PART_MAIN);
	lv_obj_set_style_text_opa(label, LV_OPA_80, LV_PART_MAIN);

	return card;
}

void gui_build_tab_overview(lv_obj_t *tab)
{
	lv_obj_t *container = lv_obj_create(tab);
	lv_obj_set_size(container, lv_pct(100), lv_pct(100));
	lv_obj_set_style_pad_all(container, 10, LV_PART_MAIN);
	lv_obj_set_style_pad_row(container, 10, LV_PART_MAIN);
	lv_obj_set_style_pad_column(container, 10, LV_PART_MAIN);
	lv_obj_set_scrollbar_mode(container, LV_SCROLLBAR_MODE_OFF);

	static const int32_t column_desc[] = {LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};
	static const int32_t row_desc[] = {LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};

	lv_obj_set_layout(container, LV_LAYOUT_GRID);
	lv_obj_set_grid_dsc_array(container, column_desc, row_desc);

	// Battery card
	lv_obj_t *card_battery = make_card(container, "Battery");
	lv_obj_set_grid_cell(card_battery,
						 LV_GRID_ALIGN_STRETCH, 0, 1,
						 LV_GRID_ALIGN_STRETCH, 0, 1);

	// Tanks card
	lv_obj_t *card_tanks = make_card(container, "Tanks");
	lv_obj_set_grid_cell(card_tanks,
						 LV_GRID_ALIGN_STRETCH, 1, 1,
						 LV_GRID_ALIGN_STRETCH, 0, 1);

	// Electrical card
	lv_obj_t *card_electrical = make_card(container, "Electrical");
	lv_obj_set_grid_cell(card_electrical,
						 LV_GRID_ALIGN_STRETCH, 0, 1,
						 LV_GRID_ALIGN_STRETCH, 1, 1);
}
