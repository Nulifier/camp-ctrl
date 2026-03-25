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

fn build_lvgl(lvgl_root: &Path, lvgl_src: &Path, conf_dir: &Path) {
	// Collect all C source files from the lvgl source directory
	let mut lvgl_sources = Vec::new();
	collect_c_files_recursive(&lvgl_src, &mut lvgl_sources);
	lvgl_sources.sort();

	// Watch recursively for changes to all C source files and headers in the lvgl source directory
	rerun_if_changed_recursive(&lvgl_src);
	rerun_if_changed_recursive(&conf_dir);

	// Watch for changes to environment variables that affect the build
	println!("cargo:rerun-if-env-changed=CC");
	println!("cargo:rerun-if-env-changed=CFLAGS");
	println!("cargo:rerun-if-env-changed=TARGET");

	// Build the lvgl C library
	let mut lvgl_build = cc::Build::new();
	lvgl_build
		.include(&lvgl_root)
		.include(&lvgl_src)
		.include(&conf_dir)
		.define("LV_CONF_INCLUDE_SIMPLE", None)
		.flag_if_supported("-std=c11")
		.warnings(false);

	for src in &lvgl_sources {
		lvgl_build.file(src);
	}
	lvgl_build.compile("lvgl");

	pkg_config::Config::new()
		.probe("sdl2")
		.expect("Failed to find SDL2 via pkg-config");
}

fn build_bindings(lvgl_root: &Path, lvgl_src: &Path, conf_dir: &Path, shims_dir: &Path) {
	let mut cc_args = vec![
		// Definitions
		"-DLV_CONF_INCLUDE_SIMPLE",
		// Include paths
		"-I",
		lvgl_root.to_str().unwrap(),
		"-I",
		lvgl_src.to_str().unwrap(),
		"-I",
		conf_dir.to_str().unwrap(),
		// Compiler flags
		"-std=c11",
		// Warnings
		// "-Wall",
		// "-Wextra",
	];

	// Set correct target triple for bindgen when cross-compiling
	let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
	let host = env::var("HOST").expect("Cargo build scripts always have HOST");
	if target != host {
		cc_args.push("-target");
		cc_args.push(target.as_str());
	}

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let bindings = bindgen::Builder::default()
		.header(shims_dir.join("lvgl_sys.h").to_str().unwrap())
		.generate_comments(true)
		.derive_default(true)
		.layout_tests(false)
		.use_core()
		.ctypes_prefix("cty")
		.clang_args(&cc_args)
		.generate()
		.expect("Failed to generate bindings");

	// Output to file
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Failed to write bindings");
}

fn main() {
	// Define our directory structure
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let workspace_root = manifest_dir.parent().unwrap();
	let lvgl_root = workspace_root.join("vendor").join("lvgl");
	let lvgl_src = lvgl_root.join("src");
	let conf_dir = manifest_dir.join("conf");
	let shims_dir = manifest_dir.join("shims");

	build_lvgl(&lvgl_root, &lvgl_src, &conf_dir);
	build_bindings(&lvgl_root, &lvgl_src, &conf_dir, &shims_dir);
}
