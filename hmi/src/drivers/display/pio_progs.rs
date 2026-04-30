use crate::error::Result;
use embassy_rp::pio::{self, Common, Instance, program::pio_file};

pub(super) fn load_vsync_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_file!("src/drivers/display/rgb.pio", select_program("vsync")).program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

pub(super) fn load_hsync_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_file!("src/drivers/display/rgb.pio", select_program("hsync")).program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

pub(super) fn load_rgb_de_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_file!("src/drivers/display/rgb.pio", select_program("rgb_de")).program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

pub(super) fn load_rgb_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_file!("src/drivers/display/rgb.pio", select_program("rgb")).program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}
