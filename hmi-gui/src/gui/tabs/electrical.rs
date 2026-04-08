use lvgl::widgets::obj::Obj;

pub struct GuiTabElectrical {
	#[allow(dead_code)]
	root: Obj,
}

impl GuiTabElectrical {
	pub fn new(parent: &Obj) -> Self {
		let root = Obj::new(parent);
		Self { root }
	}
}
