use lvgl::widgets::obj::Obj;

pub struct GuiTabTanks {
	#[allow(dead_code)]
	container: Obj,
}

impl GuiTabTanks {
	pub fn new(parent: &Obj) -> Self {
		let container = Obj::new(parent);
		Self { container }
	}
}
