#![no_std]
#![no_main]

use crate::board::{
	AssignedResources, BacklightResources, DisplayCtrlResources, DisplayDataResources,
	DisplayFillResources, DisplayTimingResources, I2c1Resources, PsramResources, TouchResources,
};
use crate::drivers::display::spawn_display_tasks;
use crate::services::backlight::{backlight_task, wake_screen};
use crate::services::gui::gui_task;
use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use defmt::{info, unwrap};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_rp::block::ImageDef;
use embassy_rp::executor::{Executor, InterruptExecutor};
use embassy_rp::interrupt::InterruptExt;
use embassy_rp::multicore::{Stack, spawn_core1};
use embassy_rp::{self as hal, interrupt};
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
static EXECUTOR1_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR1_GUI: StaticCell<Executor> = StaticCell::new();

const HEAP_SIZE: usize = 16 * 1024 * 10; // 16 KB
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

#[interrupt]
unsafe fn SWI_IRQ_0() {
	unsafe { EXECUTOR1_HIGH.on_interrupt() };
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
			// Setup the executor for core 1 to run higher priority tasks
			interrupt::SWI_IRQ_0.set_priority(interrupt::Priority::P2);
			let spawner1_high = EXECUTOR1_HIGH.start(interrupt::SWI_IRQ_0);
			spawn_display_tasks(
				&spawner1_high,
				r.display_ctrl,
				r.display_timing,
				r.display_data,
				r.display_fill,
			);

			// Run the GUI on core 1 at lower priority
			let executor1_gui = EXECUTOR1_GUI.init(Executor::new());
			executor1_gui.run(|spawner1_low| {
				//////////////////////////////////////////////////////////////
				// Core1: UI and related tasks
				spawner1_low.spawn(unwrap!(gui_task()));
			})
		},
	);

	let executor0 = EXECUTOR0.init(Executor::new());
	executor0.run(|spawner| {
		//////////////////////////////////////////////////////////////
		// Core0: Drivers and services

		let i2c = i2c::init(r.i2c1);

		spawner.spawn(unwrap!(backlight_task(r.backlight)));

		spawner.spawn(unwrap!(services::touch_task::touch_task(
			// r.touch.tp_rst.into(),
			// r.touch.tp_int.into(),
			I2cDevice::new(i2c),
			r.touch,
		)));

		spawner.spawn(unwrap!(dimmer()));
		spawner.spawn(unwrap!(wake_screen()));
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
