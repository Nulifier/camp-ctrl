use lvgl::widgets::obj::Obj;

pub struct GuiTabOverview {
	#[allow(dead_code)]
	container: Obj,
}

impl GuiTabOverview {
	pub fn new(parent: &Obj) -> Self {
		let container = Obj::new(parent);
		Self { container }
	}
}
