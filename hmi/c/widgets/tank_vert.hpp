#pragma once

#include <lvgl.h>

class VerticalTankWidget
{
public:
	void create(lv_obj_t *parent, const char *name, lv_color_t color);
	void destroy();

	inline lv_obj_t *get_wrapper() const { return wrapper; }

	void set_level(int32_t level_pct);

private:
	lv_obj_t *wrapper = nullptr;
	lv_obj_t *bar = nullptr;
	lv_obj_t *label_pct = nullptr;
	lv_obj_t *label_name = nullptr;
};
