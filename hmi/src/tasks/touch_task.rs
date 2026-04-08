use crate::{
	drivers::{i2c, touch},
	error::Result,
};
use defmt::{info, unwrap, warn};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_rp::{Peri, gpio::AnyPin};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub};

pub use crate::drivers::touch::MAX_NUM_POINTS as MAX_TOUCH_POINTS;
const EVENT_QUEUE_SIZE: usize = 8;
const MAX_SUBSCRIBERS: usize = 4;
const MAX_PUBLISHERS: usize = 1;

pub type TouchPoints = heapless::Vec<touch::TouchPoint, MAX_TOUCH_POINTS>;

#[derive(Debug, Clone)]
pub enum TouchEvent {
	Touched(TouchPoints),
	Released,
	PalmDetected,
	PalmCleared,
}

type TouchEventChannel = pubsub::PubSubChannel<
	CriticalSectionRawMutex,
	TouchEvent,
	EVENT_QUEUE_SIZE,
	MAX_SUBSCRIBERS,
	MAX_PUBLISHERS,
>;
static TOUCH_EVENTS: TouchEventChannel = TouchEventChannel::new();

type TouchEventSubscriber = pubsub::Subscriber<
	'static,
	CriticalSectionRawMutex,
	TouchEvent,
	EVENT_QUEUE_SIZE,
	MAX_SUBSCRIBERS,
	MAX_PUBLISHERS,
>;
pub fn subscribe() -> Result<TouchEventSubscriber> {
	TOUCH_EVENTS.subscriber().map_err(|e| e.into())
}

#[embassy_executor::task]
pub async fn touch_task(
	pin_tp_rst: Peri<'static, AnyPin>,
	pin_int: Peri<'static, AnyPin>,
	i2c_dev: I2cDevice<'static, CriticalSectionRawMutex, i2c::I2cDriver>,
) -> ! {
	// Initialize the touch driver
	info!("Initializing touch driver");
	let mut touch_driver = crate::drivers::touch::TouchDriver::new(i2c_dev, pin_tp_rst, pin_int);
	unwrap!(touch_driver.initialize().await);
	info!("Touch driver initialized successfully");

	let event_pub = unwrap!(
		TOUCH_EVENTS.publisher(),
		"Failed to create touch event publisher"
	);

	let mut last_touch_count = 0;
	let mut last_palm_state = false;

	loop {
		// Wait for the interrupt pin to go low, indicating a touch event
		touch_driver.wait_for_int().await;

		// Read the touch points from the driver
		match touch_driver.read_points().await {
			Ok(status) => {
				let old_touch_count = last_touch_count;
				last_touch_count = status.points.len();

				// Send palm events
				if status.palm && !last_palm_state {
					info!("Palm detected");
					event_pub.publish_immediate(TouchEvent::PalmDetected);
				} else if !status.palm && last_palm_state {
					info!("Palm cleared");
					event_pub.publish_immediate(TouchEvent::PalmCleared);
				}
				last_palm_state = status.palm;

				// Send touch events
				if status.points.is_empty() && old_touch_count > 0 {
					info!("Touch released");
					event_pub.publish_immediate(TouchEvent::Released);
				} else if !status.points.is_empty() {
					info!("Touch pressed with {} points", status.points.len());
					event_pub.publish_immediate(TouchEvent::Touched(status.points));
				}
			}
			Err(e) => {
				warn!("Failed to read touch points: {}", e);
			}
		}
	}
}
