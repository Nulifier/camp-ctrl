use lvgl::widgets::obj::Obj;

pub struct GuiTabSettings {
	#[allow(dead_code)]
	container: Obj,
}

impl GuiTabSettings {
	pub fn new(parent: &Obj) -> Self {
		let container = Obj::new(parent);
		Self { container }
	}
}
