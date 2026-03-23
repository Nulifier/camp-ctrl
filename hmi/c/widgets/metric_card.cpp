#include "metric_card.hpp"
#include "gui_conf.hpp"
#include "styles.hpp"

int32_t COLUMN_DESC[] = {LV_GRID_CONTENT, LV_GRID_FR(1), 5, LV_GRID_TEMPLATE_LAST};
const int32_t ROW_DESC[] = {LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};

char *get_unit_string(MetricUnit unit)
{
	switch (unit)
	{
	case MetricUnit::Percent:
		return "%";
	case MetricUnit::Amps:
		return "A";
	case MetricUnit::Volts:
		return "V";
	case MetricUnit::Watts:
		return "W";
	case MetricUnit::None:
	default:
		return " ";
	}
}

void MetricCard::create(lv_obj_t *parent, const char *title)
{
	// Set the unit column width based on the 'W' character width
	COLUMN_DESC[2] = styles::W_WIDTH;

	card = lv_obj_create(parent);
	lv_obj_set_layout(card, LV_LAYOUT_GRID);
	lv_obj_set_grid_dsc_array(card, COLUMN_DESC, ROW_DESC);

	label_title = lv_label_create(card);
	lv_label_set_text(label_title, title);
	lv_obj_set_style_text_font(label_title, LARGE_FONT, LV_PART_MAIN);
	// lv_obj_set_style_text_opa(label_title, LV_OPA_80, LV_PART_MAIN);
	lv_obj_set_grid_cell(label_title,
						 LV_GRID_ALIGN_START, 0, 1,
						 LV_GRID_ALIGN_START, 0, 1);

	label_icon = lv_label_create(card);
	lv_label_set_text(label_icon, LV_SYMBOL_AUDIO);
	lv_obj_set_style_text_font(label_icon, LARGE_FONT, LV_PART_MAIN);
	lv_obj_set_grid_cell(label_icon,
						 LV_GRID_ALIGN_START, 0, 1,
						 LV_GRID_ALIGN_END, 1, 2);

	lv_obj_update_layout(card);
	lv_area_t coords;
	lv_obj_get_coords(card, &coords);

	// Metrics will be added later with add_metric()
}

void MetricCard::destroy()
{
	if (card)
	{
		lv_obj_delete(card);
		card = nullptr;
		label_title = nullptr;
		label_icon = nullptr;
		for (size_t i = 0; i < MAX_METRICS; i++)
		{
			label_metrics[i] = nullptr;
			label_units[i] = nullptr;
		}
	}
}

void MetricCard::get_coords(lv_area_t *area, bool before_draw) const
{
	if (card)
	{
		if (before_draw)
		{
			// Get the coordinates of the card before it's drawn (i.e., before layout is applied)
			lv_obj_update_layout(card);
		}

		lv_obj_get_coords(card, area);
	}
	else
	{
		area->x1 = area->y1 = area->x2 = area->y2 = 0;
	}
}

size_t MetricCard::add_metric(const char *name, const char *value, MetricUnit unit)
{
	// Find the first empty slot
	for (size_t i = 0; i < MAX_METRICS; i++)
	{
		if (label_metrics[i] == nullptr)
		{
			// Create metric label
			label_metrics[i] = lv_label_create(card);
			lv_label_set_text(label_metrics[i], value);
			lv_obj_set_grid_cell(label_metrics[i],
								 LV_GRID_ALIGN_END, 1, 1,
								 LV_GRID_ALIGN_CENTER, i, 1);

			// Create unit label if needed
			if (unit != MetricUnit::None)
			{
				label_units[i] = lv_label_create(card);
				lv_label_set_text(label_units[i], get_unit_string(unit));
				lv_obj_set_grid_cell(label_units[i],
									 LV_GRID_ALIGN_START, 2, 1,
									 LV_GRID_ALIGN_CENTER, i, 1);
			}

			return i; // Return the index of the added metric
		}
	}

	return -1; // No space for more metrics
}