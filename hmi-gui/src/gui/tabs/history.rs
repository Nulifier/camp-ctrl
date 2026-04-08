use lvgl::widgets::obj::Obj;

pub struct GuiTabHistory {
	#[allow(dead_code)]
	container: Obj,
}

impl GuiTabHistory {
	pub fn new(parent: &Obj) -> Self {
		let container = Obj::new(parent);
		Self { container }
	}
}
