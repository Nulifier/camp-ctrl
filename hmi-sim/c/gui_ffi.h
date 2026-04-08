#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C"
{
#endif

	typedef struct
	{
		int32_t tank_level_pct;
		int32_t solar_watts;
		bool charging;
	} UiSnapshot;

	/* GUI functions, called from Rust */
	int gui_init(void);
	void gui_tick_inc(uint32_t ms);
	void gui_task_handler(void);
	void gui_apply_snapshot(const UiSnapshot *snapshot);

	/* Event handlers, implemented in Rust */
	void rust_on_reset_button(void);

#ifdef __cplusplus
}
#endif
