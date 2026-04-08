#include "tank_vert.hpp"
#include "gui_conf.hpp"
#include "styles.hpp"

void VerticalTankWidget::create(lv_obj_t *parent, const char *name, lv_color_t color)
{
	wrapper = lv_obj_create(parent);
	lv_obj_add_style(wrapper, &styles::wrapper, LV_PART_MAIN);
	// lv_obj_set_size(wrapper, 40, 100);
	// lv_obj_set_size(wrapper, LV_PCT(100), LV_PCT(100));

	// Create the bar
	bar = lv_bar_create(wrapper);
	lv_bar_set_orientation(bar, LV_BAR_ORIENTATION_VERTICAL);
	lv_bar_set_range(bar, 0, 100);
	lv_bar_set_value(bar, 0, LV_ANIM_OFF);
	lv_obj_set_size(bar, LV_PCT(100), LV_PCT(100));
	lv_obj_set_style_bg_color(bar, color, LV_PART_INDICATOR);
	lv_obj_set_style_radius(bar, 5, LV_PART_MAIN);
	lv_obj_set_style_radius(bar, 5, LV_PART_INDICATOR);

	// Create the percentage label
	label_pct = lv_label_create(wrapper);
	lv_label_set_text(label_pct, "0%");
	lv_obj_center(label_pct);
	lv_obj_set_style_text_font(label_pct, LARGE_FONT, LV_PART_MAIN);

	// Create the name label
	label_name = lv_label_create(wrapper);
	lv_label_set_text(label_name, name);
	lv_obj_align(label_name, LV_ALIGN_BOTTOM_MID, 0, -20);
}

void VerticalTankWidget::destroy()
{
	// if (wrapper)
	// {
	// 	// Deleting the wrapper will also delete all child objects (bar, labels, etc.)
	// 	lv_obj_del(wrapper);
	// 	wrapper = nullptr;
	// }
}

void VerticalTankWidget::set_level(int32_t level_pct)
{
	if (wrapper)
	{
		lv_bar_set_value(bar, level_pct, LV_ANIM_ON);
		lv_label_set_text_fmt(label_pct, "%d%%", level_pct);
	}
}
