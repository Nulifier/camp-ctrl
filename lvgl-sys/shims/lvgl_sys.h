#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

#include <lvgl.h>

#ifdef LV_USE_SDL
#include "drivers/sdl/lv_sdl_window.h"
#include "drivers/sdl/lv_sdl_mouse.h"
#endif

#ifdef __cplusplus
}
#endif
