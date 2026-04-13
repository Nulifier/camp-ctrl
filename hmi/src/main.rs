#![no_std]
#![no_main]

use crate::board::{
	AssignedResources, BacklightResources, DisplayResources, I2c1Resources, PsramResources,
	TouchResources,
};
use crate::services::backlight::{backlight_task, wake_screen};
use crate::services::gui::gui_task;
use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use defmt::{info, unwrap};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Executor;
use embassy_rp::block::ImageDef;
use embassy_rp::multicore::{Stack, spawn_core1};
use embassy_rp::{self as hal};
use embedded_alloc::TlsfHeap as Heap;

pub mod board;
pub mod drivers;
pub mod error;
pub mod gui;
pub mod services;
pub mod touch;
pub mod utils;

// Panic handler
use panic_probe as _;
// Defmt logging
use defmt_rtt as _;
use static_cell::StaticCell;

use crate::drivers::i2c::{self};

// Tell the boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

const HEAP_SIZE: usize = 16 * 1024; // 16 KB
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::task]
async fn dimmer() {
	let mut touch_event_sub = services::touch_task::subscribe().unwrap();

	loop {
		match touch_event_sub.next_message_pure().await {
			services::touch_task::TouchEvent::Touched(points) => {
				let first_point = &points[0];
				info!("Touched at ({}, {})", first_point.x, first_point.y);

				// Scale y value to brightness (0-100)
				let brightness = (first_point.y as u32 * 100 / 480) as u8;

				crate::services::backlight::set_brightness(brightness);
			}
			_ => {}
		}
	}
}

#[cortex_m_rt::entry]
fn main() -> ! {
	// Initialize the heap allocator
	{
		static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
		unsafe {
			HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE);
		}
	}

	let p = hal::init(Default::default());
	let r = split_resources!(p);

	// Initialize the PSRAM allocator
	utils::psram::init_psram_heap(r.psram);

	spawn_core1(
		p.CORE1,
		unsafe { &mut *addr_of_mut!(CORE1_STACK) },
		move || {
			let executor1 = EXECUTOR1.init(Executor::new());
			executor1.run(|_spawner| {
				//////////////////////////////////////////////////////////////
				// Core1: UI and related tasks
				unwrap!(_spawner.spawn(gui_task()));
			})
		},
	);

	let executor0 = EXECUTOR0.init(Executor::new());
	executor0.run(|spawner| {
		//////////////////////////////////////////////////////////////
		// Core0: Drivers and services

		let i2c = i2c::init(r.i2c1);

		unwrap!(spawner.spawn(backlight_task(r.backlight)));

		spawner
			.spawn(services::touch_task::touch_task(
				// r.touch.tp_rst.into(),
				// r.touch.tp_int.into(),
				I2cDevice::new(i2c),
				r.touch,
			))
			.unwrap();

		spawner.spawn(dimmer()).unwrap();
		spawner.spawn(wake_screen()).unwrap();
	});
}

// Program metadata for `picotool info`
// This isn't needed, but its recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
	embassy_rp::binary_info::rp_program_name!(c"hmi"),
	embassy_rp::binary_info::rp_program_description!(c"Camp Ctrl HMI"),
	embassy_rp::binary_info::rp_cargo_version!(),
	embassy_rp::binary_info::rp_program_build_attribute!(),
];
