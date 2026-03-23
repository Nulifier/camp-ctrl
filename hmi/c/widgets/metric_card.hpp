#pragma once

#include <lvgl.h>

static const size_t MAX_METRICS = 3; // Also need to update the row_desc in MetricCard::create() if this is changed

enum class MetricUnit
{
	None,
	Percent,
	Amps,
	Volts,
	Watts,
};

class MetricCard
{
public:
	void create(lv_obj_t *parent, const char *title);
	void destroy();

	inline lv_obj_t *get_wrapper() const { return card; }

	void get_coords(lv_area_t *area, bool before_draw = false) const;

	size_t add_metric(const char *name, const char *value, MetricUnit unit = MetricUnit::None);
	void update_metric(size_t idx, const char *value, MetricUnit unit = MetricUnit::None);

private:
	lv_obj_t *card = nullptr;
	lv_obj_t *label_title = nullptr;
	lv_obj_t *label_icon = nullptr;

	lv_obj_t *label_metrics[MAX_METRICS] = {nullptr, nullptr, nullptr};
	lv_obj_t *label_units[MAX_METRICS] = {nullptr, nullptr, nullptr};
};
