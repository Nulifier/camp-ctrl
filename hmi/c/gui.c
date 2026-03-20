#include "gui.h"
#include "lvgl.h"

static lv_obj_t* label_power;
static lv_obj_t* bar_tank;
static lv_obj_t* btn_reset;

static void on_reset_clicked(lv_event_t* e) {
	(void)e;
	rust_on_reset_button();
}

void gui_init(void) {
	lv_init();

	lv_obj_t *scr = lv_scr_act();

	label_power = lv_label_create(scr);
	lv_label_set_text(label_power, "Power: 0 W");
	lv_obj_align(label_power, LV_ALIGN_TOP_MID, 0, 10);

	bar_tank = lv_bar_create(scr);
	lv_obj_set_size(bar_tank, 180, 20);
	lv_obj_align(bar_tank, LV_ALIGN_CENTER, 0, 0);
	lv_bar_set_range(bar_tank, 0, 100);

	btn_reset = lv_btn_create(scr);
	lv_obj_align(btn_reset, LV_ALIGN_BOTTOM_MID, 0, -10);
	lv_obj_add_event_cb(btn_reset, on_reset_clicked, LV_EVENT_CLICKED, NULL);

	lv_obj_t *btn_label = lv_label_create(btn_reset);
	lv_label_set_text(btn_label, "Reset");
	lv_obj_center(btn_label);
}

void gui_tick_inc(uint32_t ms) {
	lv_tick_inc(ms);
}

void gui_task_handler(void) {
	lv_task_handler();
}

void gui_apply_snapshot(const UiSnapshot *snapshot) {
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
