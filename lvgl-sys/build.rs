use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_c_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
	for entry in fs::read_dir(dir).unwrap() {
		let path = entry.unwrap().path();
		if path.is_dir() {
			collect_c_files_recursive(&path, out);
		} else if path.extension().and_then(|s| s.to_str()) == Some("c") {
			out.push(path);
		}
	}
}

fn rerun_if_changed_recursive(dir: &Path) {
	for entry in fs::read_dir(dir).unwrap() {
		let path = entry.unwrap().path();
		if path.is_dir() {
			rerun_if_changed_recursive(&path);
		} else if path.extension().and_then(|s| s.to_str()) == Some("c") {
			println!("cargo:rerun-if-changed={}", path.display());
		} else if path.extension().and_then(|s| s.to_str()) == Some("h") {
			println!("cargo:rerun-if-changed={}", path.display());
		}
	}
}

fn main() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let workspace_root = manifest_dir.parent().unwrap();
	let lvgl_root = workspace_root.join("vendor").join("lvgl");
	let lvgl_src = lvgl_root.join("src");
	let conf_dir = manifest_dir.join("c");

	let mut lvgl_sources = Vec::new();
	collect_c_files_recursive(&lvgl_src, &mut lvgl_sources);
	lvgl_sources.sort();

	rerun_if_changed_recursive(&lvgl_src);

	println!(
		"cargo:rerun-if-changed={}",
		conf_dir.join("lv_conf.h").display()
	);
	println!("cargo:rerun-if-env-changed=CC");
	println!("cargo:rerun-if-env-changed=CFLAGS");
	println!("cargo:rerun-if-env-changed=TARGET");

	let mut lvgl_build = cc::Build::new();
	lvgl_build
		.include(&conf_dir)
		.include(&lvgl_root)
		.include(&lvgl_src)
		.define("LV_CONF_INCLUDE_SIMPLE", None)
		.flag_if_supported("-std=c11")
		.warnings(true);
	for src in &lvgl_sources {
		lvgl_build.file(src);
	}
	lvgl_build.compile("lvgl");

	println!("cargo:rustc-link-lib=static=lvgl");
	println!(
		"cargo:rustc-link-search=native={}/release/build/lvgl-sys-*/out",
		env::var("OUT_DIR").unwrap().split("/out").next().unwrap()
	);

	pkg_config::Config::new()
		.probe("sdl2")
		.expect("Failed to find SDL2 via pkg-config");
}
