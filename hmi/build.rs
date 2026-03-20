use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_c_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read directory {}: {e}", dir.display()));

    for entry in entries {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {}: {e}", dir.display()));
        let path = entry.path();

        if path.is_dir() {
            collect_c_files_recursive(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            out.push(path);
        }
    }
}

fn collect_top_level_c_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read directory {}: {e}", dir.display()));

    for entry in entries {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {}: {e}", dir.display()));
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("c") {
            out.push(path);
        }
    }
}

fn rerun_if_changed_recursive(dir: &Path) {
    println!("cargo:rerun-if-changed={}", dir.display());

    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read directory {}: {e}", dir.display()));

    for entry in entries {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {}: {e}", dir.display()));
        let path = entry.path();

        if path.is_dir() {
            rerun_if_changed_recursive(&path);
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let lvgl_root = manifest_dir.join("vendor").join("lvgl");
    let lvgl_src = lvgl_root.join("src");
    let c_dir = manifest_dir.join("c");
    let lv_conf = c_dir.join("lv_conf.h");

    if !lvgl_root.exists() {
        panic!("LVGL root not found: {}", lvgl_root.display());
    }

    if !lvgl_src.exists() {
        panic!("LVGL src directory not found: {}", lvgl_src.display());
    }

    if !c_dir.exists() {
        panic!("C source directory not found: {}", c_dir.display());
    }

    if !lv_conf.exists() {
        panic!("LVGL config header not found: {}", lv_conf.display());
    }

    let mut sources = Vec::new();

    // Compile all LVGL library sources from vendor/lvgl/src
    collect_c_files_recursive(&lvgl_src, &mut sources);

    // Compile all top-level C files in ./c
    collect_top_level_c_files(&c_dir, &mut sources);

    if sources.is_empty() {
        panic!("No C source files found to compile");
    }

    sources.sort();

    let mut build = cc::Build::new();

    for src in &sources {
        build.file(src);
    }

    build
        // Include paths
        .include(&c_dir)
        .include(&lvgl_root)
        .include(&lvgl_src)
        // Tell LVGL to include lv_conf.h via include path lookup
        .define("LV_CONF_INCLUDE_SIMPLE", None)
        // Reasonable default C standard
        .flag_if_supported("-std=c11")
        .warnings(true);

    // Rebuild when sources or headers change
    rerun_if_changed_recursive(&lvgl_src);
    rerun_if_changed_recursive(&c_dir);

    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CFLAGS");

    build.compile("lvgl_ffi");
}
