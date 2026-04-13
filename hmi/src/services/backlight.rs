use crate::{board::BacklightResources, drivers::lcd_bl::BacklightDriver, services::touch_task};
use defmt::{debug, info};
use embassy_futures::select::{Either, select};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Instant, Timer};

const INACTIVITY_TIMEOUT: Duration = Duration::from_secs(10);

static ENABLE_SIG: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static BRIGHTNESS_SIG: Signal<CriticalSectionRawMutex, u8> = Signal::new();

pub fn set_enable(enable: bool) {
	ENABLE_SIG.signal(enable);
}

pub fn set_brightness(brightness: u8) {
	BRIGHTNESS_SIG.signal(brightness);
}

#[embassy_executor::task]
pub async fn backlight_task(_r: BacklightResources) {
	info!("Starting backlight task");
	let mut driver = BacklightDriver::new(_r);

	// Start with backlight enabled at full brightness
	driver.enable(true);
	driver.set_brightness(100);

	loop {
		// Wait for either an enable signal or a brightness signal
		match select(ENABLE_SIG.wait(), BRIGHTNESS_SIG.wait()).await {
			Either::First(enable) => {
				driver.enable(enable);
			}
			Either::Second(brightness) => {
				driver.set_brightness(brightness);
			}
		}
	}
}

#[embassy_executor::task]
pub async fn wake_screen() {
	info!("Starting wake_screen task");

	let mut is_lcd_on = false;
	let mut touch_event_sub = touch_task::subscribe().unwrap();
	let mut last_touch_time = Instant::now();

	// States:
	// - Idle: LCD is off, waiting for touch event
	// - Active: LCD is on, waiting for inactivity timeout or touch event
	// On touch event:
	// - If idle, turn on LCD and go to active
	// - If active, reset inactivity timer
	// On inactivity timeout:
	// - If active, turn off LCD and go to idle

	loop {
		if is_lcd_on {
			// If the LCD is on we wait for a touch event or inactivity timeout
			let timeout = Timer::at(last_touch_time + INACTIVITY_TIMEOUT);
			match select(timeout, touch_event_sub.next_message_pure()).await {
				Either::First(_) => {
					// Inactivity timeout
					info!("Inactivity timeout, turning off LCD");
					set_enable(false);
					is_lcd_on = false;
				}
				Either::Second(_) => {
					// Touch event, reset inactivity timer
					debug!("Touch event received, resetting inactivity timer");
					last_touch_time = Instant::now();
				}
			}
		} else {
			// If the LCD is off we just wait for a touch event
			touch_event_sub.next_message_pure().await;
			info!("Touch event received, turning on LCD");
			set_enable(true);
			is_lcd_on = true;
			last_touch_time = Instant::now();
		}
	}
}
