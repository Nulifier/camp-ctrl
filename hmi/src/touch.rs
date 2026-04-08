// use embassy_rp::Peri;
// use embassy_rp::gpio::AnyPin;
// use crate::i2c_driver::{I2cDriver, I2cError};
// use defmt::warn;

// #[derive(Clone, Debug)]
// pub struct TouchPoint {
//     pub id: u8,
//     pub x: u16,
//     pub y: u16,
//     pub pressed: bool,
// }

// #[derive(Clone, Debug)]
// pub enum TouchEvent {
//     Down(TouchPoint),
//     Move(TouchPoint),
//     Up(u8),
// }

// #[derive(Debug)]
// pub enum TouchError {
//     I2c(I2cError),
//     Parse,
//     Empty,
// }

// pub struct TouchService {
//     driver: I2cDriver,
//     irq: Peri<'static, AnyPin>,
//     rst: Peri<'static, AnyPin>,
// }

// impl TouchService {
//     pub fn new(driver: I2cDriver, irq: Peri<'static, AnyPin>, rst: Peri<'static, AnyPin>) -> Self {
//         TouchService { driver, irq, rst }
//     }

//     pub async fn reset(&mut self) -> Result<(), TouchError> {
//         // stub: toggle the reset pin with delays in real impl
//         warn!("TouchService::reset stub");
//         Ok(())
//     }

//     pub async fn init_controller(&mut self) -> Result<(), TouchError> {
//         warn!("TouchService::init_controller stub");
//         Ok(())
//     }

//     pub async fn read_raw_report(&mut self) -> Result<(), TouchError> {
//         warn!("TouchService::read_raw_report stub");
//         Ok(())
//     }

//     /// Pure parsing function — unit-testable. Returns Ok(()) for success in scaffold.
//     pub fn decode_report(_bytes: &[u8]) -> Result<(), TouchError> {
//         // stub: real parsing depends on controller
//         Err(TouchError::Empty)
//     }

//     pub async fn read_points(&mut self) -> Result<(), TouchError> {
//         self.read_raw_report().await?;
//         match Self::decode_report(&[]) {
//             Ok(()) => Ok(()),
//             Err(e) => Err(e),
//         }
//     }
// }
