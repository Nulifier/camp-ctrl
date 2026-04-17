use crate::error::Result;
use embassy_rp::pio::{self, Common, Instance, program::pio_asm};

// Used resources for PIOs:
// VSYNC:
// - irq 2 - Waits for hsync to signal the end of a line
// HSYNC:
// - irq 2 - Sent from hsync program to signal a new line one cycle before the HSYNC pulse goes low.
// RGB_DE:
// - irq 1 - Signals RGB program to start a new active line
// - irq 0 - Waits for the RGB program to finish sending a line of pixel data
// RGB:
// - irq 1 - Waits for RGB_DE to signal active-line start
// - irq 0 - Sent from the RGB program to signal that it has finished sending a line of pixel data

pub fn load_vsync_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_asm!(
		r#"
			.side_set 1 opt            ; LCD_VSYNC_PIN(bit0)
			.define public T1 31       ; FrontPorch
			.define public T2 31       ; PulseWidth
			.define public T3 31       ; BackPorch
			pull block side 1          ; Pull the height to osr
			.wrap_target
			; Front porch
			set x, T1 side 1
			FrontPorch:
			wait 1 irq 2 side 1        ; Wait for hsync to signal the end of the line
			jmp x-- FrontPorch side 1
			; SyncPulse
			set x, T2 side 0           ; Set VSYNC low
			SyncPulse:
			wait 1 irq 2 side 0        ; Wait for hsync to signal the end of the line
			jmp x-- SyncPulse side 0
			; Back porch
			set x, T3 side 1           ; Set VSYNC high
			BackPorch:
			wait 1 irq 2 side 1        ; Wait for hsync to signal the end of the line
			jmp x-- BackPorch side 1
			mov x, osr side 1
			ActiveFor:
			wait 1 irq 2 side 1        ; Wait for hsync to signal the end of the line
			jmp x--, ActiveFor side 1
			.wrap
		"#
	)
	.program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

pub fn load_hsync_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_asm!(
		r#"
		.side_set 2 opt                ; LCD_PCLK_PIN(bit1) LCD_HSYNC_PIN(bit0)
		.define public T1 31           ; FrontPorch
		.define public T2 31           ; PulseWidth
		.define public T3 31           ; BackPorch
		pull block                     ; Pull the width to osr
		mov y, osr                     ; Save the width to y
		.wrap_target
		; Front porch
		set x, T1 side 0b11
		FrontPorch:
		  nop side 0b11
		  jmp x-- FrontPorch side 0b11
		; SyncPulse
		irq set 2 side 0b01
		set x, T2 side 0b10            ; Set HSYNC low
		SyncPulse:
		  nop side 0b00
		  jmp x-- SyncPulse side 0b10
		; Back porch
		set x, T3 side 0b01
		BackPorch:
		  nop side 0b11
		  jmp x-- BackPorch side 0b01
		mov x, y side 0b11             ; Restore width to x
		ActiveFor:
		  nop side 0b01
		  jmp x--, ActiveFor side 0b11
		.wrap
		"#
	)
	.program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

pub fn load_rgb_de_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	// let program = pio_asm!(
	// 	r#".side_set 1 opt             ; LCD_DE_PIN(bit0)
	// 	.define public T1 31           ; vsync BackPorch
	// 	.define public T2 31           ; hsync BackPorch
	// 	pull block side 0              ; Load the width to osr
	// 	.wrap_target
	// 	mov y osr side 0               ; Save width to y
	// 	set x, T1 side 0
	// 	wait 0 pin 5 side 0            ; Wait for vsync to go low
	// 	wait 1 pin 5 side 0            ; Wait for vsync to go high
	// 	vBackPorch:
	// 	  wait 0 pin 6 side 0          ; Wait for hsync to go low
	// 	  wait 1 pin 6 side 0          ; Wait for hsync to go high
	// 	  jmp x--, vBackPorch side 0
	// 	ActiveFor:
	// 	  wait 0 pin 6 side 0          ; Wait for hsync to go low
	// 	  wait 1 pin 6 side 0          ; Wait for hsync to go high
	// 	  set x, T2 side 0
	// 	  hBackPorch:
	// 	    wait 0 pin 7 side 0        ; Wait for pclk to go low
	// 	    wait 1 pin 7 side 0        ; Wait for pclk to go high
	// 	    jmp x--, hBackPorch side 0
	// 	  wait 1 pin 7 side 0          ; Wait for pclk to go high
	// 	  wait 1 irq 0 side 1          ; Wait for program rgb to finish sending a line of data
	// 	  jmp y--, ActiveFor side 0
	// 	.wrap
	// 	"#,
	// )
	// .program;
	let program = pio_asm!(
		r#".side_set 1 opt             ; LCD_DE_PIN(bit0)
		.define public T1 31           ; vsync BackPorch
		.define public T2 31           ; hsync BackPorch
		pull block side 0              ; Load the width to osr
		.wrap_target
		mov y osr side 0               ; Save width to y
		set x, T1 side 0
		wait 0 pin 5 side 0            ; Wait for vsync to go low
		wait 1 pin 5 side 0            ; Wait for vsync to go high
		vBackPorch:
		  wait 0 pin 6 side 0          ; Wait for hsync to go low
		  wait 1 pin 6 side 0          ; Wait for hsync to go high
		  jmp x--, vBackPorch side 0
		ActiveFor:
		  wait 0 pin 6 side 0          ; Wait for hsync to go low
		  wait 1 pin 6 side 0          ; Wait for hsync to go high
		  set x, T2 side 0
		  hBackPorch:
		    wait 0 pin 7 side 0        ; Wait for pclk to go low
		    wait 1 pin 7 side 0        ; Wait for pclk to go high
		    jmp x--, hBackPorch side 0
		  wait 1 pin 7 side 0          ; Wait for pclk to go high
		  irq set 1 side 1             ; Raise DE and signal RGB start-of-line
		  wait 1 irq 0 side 1          ; Wait for RGB program to finish the line
		  jmp y--, ActiveFor side 0
		.wrap
		"#,
	)
	.program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}

#[allow(dead_code)]
pub fn load_rgb_program<PIO: Instance>(
	common: &mut Common<'static, PIO>,
) -> Result<pio::LoadedProgram<'static, PIO>> {
	let program = pio_asm!(
		r#"
		pull block                     ; Pull width into osr
		mov y, osr                     ; Save width to y
		.wrap_target

		mov x,y side 0                 ; Save width to x
		wait 1 irq 1                   ; Wait until DE program starts active line
		ColorOut:
		  pull block                   ; One RGB565 pixel in low 16 bits
		  wait 0 pin 7                 ; pclk == 0
		  out pins, 16                 ; Send rgb data
		  wait 1 pin 7                 ; pclk == 1
		  jmp x--, ColorOut
		irq set 0                      ; Tell program rgb_de that a row of data has been flushed
		.wrap
		"#
	)
	.program;

	common
		.try_load_program(&program)
		.map_err(|_| crate::error::Error::PioProgramLoadFailed)
}
