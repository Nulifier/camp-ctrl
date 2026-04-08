use core::ffi::CStr;

pub const SYMBOL_BULLET: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BULLET) };
pub const SYMBOL_AUDIO: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_AUDIO) };
pub const SYMBOL_LIST: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_LIST) };
pub const SYMBOL_OK: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_OK) };
pub const SYMBOL_CLOSE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_CLOSE) };
pub const SYMBOL_POWER: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_POWER) };
pub const SYMBOL_SETTINGS: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_SETTINGS) };
pub const SYMBOL_HOME: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_HOME) };
pub const SYMBOL_DOWNLOAD: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_DOWNLOAD) };
pub const SYMBOL_DRIVE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_DRIVE) };
pub const SYMBOL_REFRESH: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_REFRESH) };
pub const SYMBOL_MUTE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_MUTE) };
pub const SYMBOL_VOLUME_MID: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_VOLUME_MID) };
pub const SYMBOL_VOLUME_MAX: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_VOLUME_MAX) };
pub const SYMBOL_IMAGE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_IMAGE) };
pub const SYMBOL_TINT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_TINT) };
pub const SYMBOL_PREV: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_PREV) };
pub const SYMBOL_PLAY: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_PLAY) };
pub const SYMBOL_PAUSE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_PAUSE) };
pub const SYMBOL_STOP: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_STOP) };
pub const SYMBOL_NEXT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_NEXT) };
pub const SYMBOL_EJECT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_EJECT) };
pub const SYMBOL_LEFT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_LEFT) };
pub const SYMBOL_RIGHT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_RIGHT) };
pub const SYMBOL_PLUS: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_PLUS) };
pub const SYMBOL_MINUS: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_MINUS) };
pub const SYMBOL_EYE_OPEN: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_EYE_OPEN) };
pub const SYMBOL_EYE_CLOSE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_EYE_CLOSE) };
pub const SYMBOL_WARNING: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_WARNING) };
pub const SYMBOL_SHUFFLE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_SHUFFLE) };
pub const SYMBOL_UP: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_UP) };
pub const SYMBOL_DOWN: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_DOWN) };
pub const SYMBOL_LOOP: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_LOOP) };
pub const SYMBOL_DIRECTORY: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_DIRECTORY) };
pub const SYMBOL_UPLOAD: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_UPLOAD) };
pub const SYMBOL_CALL: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_CALL) };
pub const SYMBOL_CUT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_CUT) };
pub const SYMBOL_COPY: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_COPY) };
pub const SYMBOL_SAVE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_SAVE) };
pub const SYMBOL_BARS: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BARS) };
pub const SYMBOL_ENVELOPE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_ENVELOPE) };
pub const SYMBOL_CHARGE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_CHARGE) };
pub const SYMBOL_PASTE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_PASTE) };
pub const SYMBOL_BELL: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BELL) };
pub const SYMBOL_KEYBOARD: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_KEYBOARD) };
pub const SYMBOL_GPS: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_GPS) };
pub const SYMBOL_FILE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_FILE) };
pub const SYMBOL_WIFI: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_WIFI) };
pub const SYMBOL_BATTERY_FULL: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BATTERY_FULL) };
pub const SYMBOL_BATTERY_3: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BATTERY_3) };
pub const SYMBOL_BATTERY_2: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BATTERY_2) };
pub const SYMBOL_BATTERY_1: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BATTERY_1) };
pub const SYMBOL_BATTERY_EMPTY: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BATTERY_EMPTY) };
pub const SYMBOL_USB: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_USB) };
pub const SYMBOL_BLUETOOTH: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BLUETOOTH) };
pub const SYMBOL_TRASH: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_TRASH) };
pub const SYMBOL_EDIT: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_EDIT) };
pub const SYMBOL_BACKSPACE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_BACKSPACE) };
pub const SYMBOL_SD_CARD: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_SD_CARD) };
pub const SYMBOL_NEW_LINE: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_NEW_LINE) };
pub const SYMBOL_DUMMY: &'static CStr =
	unsafe { CStr::from_bytes_with_nul_unchecked(lvgl_sys::LV_SYMBOL_DUMMY) };
