#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <lvgl.h>

#define ENABLE_PROPANE_TANK

static const int32_t DISPLAY_WIDTH = 800;
static const int32_t DISPLAY_HEIGHT = 480;
static const int32_t HEADER_HEIGHT = 30;

static const char *TAB_NAMES[] = {
	LV_SYMBOL_IMAGE " Overview",
	LV_SYMBOL_CHARGE " Electrical",
	LV_SYMBOL_TRASH " Tanks",
	LV_SYMBOL_SD_CARD " History",
	LV_SYMBOL_SETTINGS " Settings"};

static const lv_font_t *DEFAULT_FONT = &lv_font_montserrat_14;
static const lv_font_t *LARGE_FONT = &lv_font_montserrat_20;

static const lv_color_t COLOR_RED = LV_COLOR_MAKE(0xF4, 0x43, 0x36);
static const lv_color_t COLOR_PINK = LV_COLOR_MAKE(0xE9, 0x1E, 0x63);
static const lv_color_t COLOR_PURPLE = LV_COLOR_MAKE(0x9C, 0x27, 0xB0);
static const lv_color_t COLOR_DEEP_PURPLE = LV_COLOR_MAKE(0x67, 0x3A, 0xB7);
static const lv_color_t COLOR_INDIGO = LV_COLOR_MAKE(0x3F, 0x51, 0xB5);
static const lv_color_t COLOR_BLUE = LV_COLOR_MAKE(0x21, 0x96, 0xF3);
static const lv_color_t COLOR_LIGHT_BLUE = LV_COLOR_MAKE(0x03, 0xA9, 0xF4);
static const lv_color_t COLOR_CYAN = LV_COLOR_MAKE(0x00, 0xBC, 0xD4);
static const lv_color_t COLOR_TEAL = LV_COLOR_MAKE(0x00, 0x96, 0x88);
static const lv_color_t COLOR_GREEN = LV_COLOR_MAKE(0x4C, 0xAF, 0x50);
static const lv_color_t COLOR_LIGHT_GREEN = LV_COLOR_MAKE(0x8B, 0xC3, 0x4A);
static const lv_color_t COLOR_LIME = LV_COLOR_MAKE(0xCD, 0xDC, 0x39);
static const lv_color_t COLOR_YELLOW = LV_COLOR_MAKE(0xFF, 0xEB, 0x3B);
static const lv_color_t COLOR_AMBER = LV_COLOR_MAKE(0xFF, 0xC1, 0x07);
static const lv_color_t COLOR_ORANGE = LV_COLOR_MAKE(0xFF, 0x98, 0x00);
static const lv_color_t COLOR_DEEP_ORANGE = LV_COLOR_MAKE(0xFF, 0x57, 0x22);
static const lv_color_t COLOR_BROWN = LV_COLOR_MAKE(0x79, 0x55, 0x48);
static const lv_color_t COLOR_BLUE_GREY = LV_COLOR_MAKE(0x60, 0x7D, 0x8B);
static const lv_color_t COLOR_GREY = LV_COLOR_MAKE(0x9E, 0x9E, 0x9E);
static const lv_color_t COLOR_KELLY = LV_COLOR_MAKE(0xF2, 0x5D, 0x00);
