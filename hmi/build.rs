use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn is_source_file(path: &Path) -> bool {
	matches!(
		path.extension().and_then(|s| s.to_str()),
		Some("c") | Some("cpp")
	)
}

fn is_watch_file(path: &Path) -> bool {
	matches!(
		path.extension().and_then(|s| s.to_str()),
		Some("h") | Some("hpp") | Some("c") | Some("cpp")
	)
}

fn walk_c_tree(dir: &Path, out: &mut Vec<PathBuf>, predicate: fn(&Path) -> bool) {
	for entry in fs::read_dir(dir).unwrap() {
		let path = entry.unwrap().path();
		if path.is_dir() {
			walk_c_tree(&path, out, predicate);
		} else if path.is_file() && predicate(&path) {
			out.push(path);
		}
	}
}

fn main() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let c_dir = manifest_dir.join("c");

	let mut gui_sources = Vec::new();
	walk_c_tree(&c_dir, &mut gui_sources, is_source_file);
	gui_sources.sort();

	// Watch recursively for changes to GUI C/C++ files and headers
	let mut gui_watch_files = Vec::new();
	walk_c_tree(&c_dir, &mut gui_watch_files, is_watch_file);
	gui_watch_files.sort();
	for path in gui_watch_files {
		println!("cargo:rerun-if-changed={}", path.display());
	}

	println!("cargo:rerun-if-env-changed=CC");
	println!("cargo:rerun-if-env-changed=CFLAGS");
	println!("cargo:rerun-if-env-changed=TARGET");

	let workspace_root = manifest_dir.parent().unwrap();
	let lvgl_root = workspace_root.join("vendor").join("lvgl");
	let lvgl_conf = workspace_root.join("lvgl-sys").join("c");

	let mut gui_build = cc::Build::new();
	gui_build
		.include(&c_dir)
		.include(&lvgl_root)
		.include(&lvgl_root.join("src"))
		.include(&lvgl_conf)
		.define("LV_CONF_INCLUDE_SIMPLE", None)
		.flag_if_supported("-std=c11")
		.flag_if_supported("-std=c++17")
		.warnings(true);
	for src in &gui_sources {
		gui_build.file(src);
	}
	gui_build.compile("hmi_gui");

	println!("cargo:rustc-link-lib=static=hmi_gui");
	println!("cargo:rustc-link-lib=static=lvgl");

	pkg_config::Config::new()
		.probe("sdl2")
		.expect("Failed to find SDL2 via pkg-config");
}
