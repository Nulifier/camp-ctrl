#include "styles.hpp"
#include "gui_conf.hpp"

uint16_t styles::W_WIDTH = 0;
lv_style_t styles::wrapper;

void styles::init()
{
	styles::W_WIDTH = lv_font_get_glyph_width(DEFAULT_FONT, 'W', 'W');

	lv_style_init(&styles::wrapper);
	lv_style_set_size(&styles::wrapper, LV_PCT(100), LV_PCT(100));
	lv_style_set_pad_all(&styles::wrapper, 0);
	lv_style_set_margin_all(&styles::wrapper, 0);
	lv_style_set_border_width(&styles::wrapper, 0);
	lv_style_set_bg_opa(&styles::wrapper, LV_OPA_TRANSP);
}