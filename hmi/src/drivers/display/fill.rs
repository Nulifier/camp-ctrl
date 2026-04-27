use crate::board::DisplayFillResources;

#[allow(dead_code)]
pub(crate) struct FillEngine {
	// pub dma_channel: dma::Channel<'static>,
}

impl FillEngine {
	#[allow(dead_code)]
	pub fn new(_r: DisplayFillResources) -> Self {
		FillEngine {
			// dma_channel: dma::Channel::new(r.dma_fill, crate::board::Irqs),
		}
	}
}
