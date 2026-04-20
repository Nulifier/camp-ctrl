use embassy_rp::{Peri, peripherals};

use crate::board::DisplayFillResources;

pub(crate) struct FillEngine {
	pub dma_channel: Peri<'static, peripherals::DMA_CH1>,
}

impl FillEngine {
	pub fn new(r: DisplayFillResources) -> Self {
		FillEngine {
			dma_channel: r.dma_fill,
		}
	}
}
