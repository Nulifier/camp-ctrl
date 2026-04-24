use embassy_rp::dma;

use crate::board::DisplayFillResources;

pub(crate) struct FillEngine {
	// pub dma_channel: dma::Channel<'static>,
}

impl FillEngine {
	pub fn new(r: DisplayFillResources) -> Self {
		FillEngine {
			// dma_channel: dma::Channel::new(r.dma_fill, crate::board::Irqs),
		}
	}
}
