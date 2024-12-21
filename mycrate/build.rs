use cargo_metadata::MetadataCommand;
use std::env::var;
use std::fs::{copy, read_dir};
use std::path::PathBuf;

fn main() {
    // build dylib
    let out_dir = PathBuf::from(var("OUT_DIR").expect("OUT_DIR not found"));
    let target = var("TARGET").expect("env not found");
    let mut cargo = std::process::Command::new("cargo");
    let mut cmd = cargo.arg("build").arg("--target").arg(target.clone());
    if cfg!(not(debug_assertions)) {
        cmd = cmd.arg("--release");
    }
    let mut hook_toml = PathBuf::from(var("CARGO_MANIFEST_DIR").expect("env not found"))
        .parent()
        .expect("parent not found")
        .join("dep")
        .join("Cargo.toml");
    let metadata = MetadataCommand::default()
        .no_deps()
        .exec()
        .expect("read cargo metadata failed");
    let package = if hook_toml.exists() {
        metadata
            .packages
            .iter()
            .find(|pkg| pkg.name.eq("mycrate"))
            .expect("read current package failed")
    } else {
        metadata
            .packages
            .first()
            .expect("read current package failed")
    };
    let dependency = package
        .dependencies
        .iter()
        .find(|dep| dep.name.eq("dep"))
        .expect("dep not found");
    if !hook_toml.exists() {
        // 使用cargo_metadata读到依赖版本，结合CARGO_HOME获取dep的toml
        let dep_src_dir = PathBuf::from(var("CARGO_HOME").expect("CARGO_HOME not found"))
            .join("registry")
            .join("src");
        let crates_parent_dirs = Vec::from_iter(
            read_dir(dep_src_dir.clone())
                .expect("Failed to read deps")
                .flatten(),
        );
        let crates_parent = if crates_parent_dirs.len() == 1 {
            crates_parent_dirs.first().expect("host dir not found")
        } else {
            let rustup_dist_server =
                var("RUSTUP_DIST_SERVER").expect("RUSTUP_DIST_SERVER not found");
            let host = rustup_dist_server
                .split("://")
                .last()
                .expect("host not found");
            crates_parent_dirs
                .iter()
                .find(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                        .contains(host)
                })
                .unwrap_or_else(|| {
                    crates_parent_dirs
                        .iter()
                        .find(|entry| {
                            entry
                                .file_name()
                                .to_string_lossy()
                                .to_string()
                                .contains("crates.io")
                        })
                        .expect("host dir not found")
                })
        }
        .file_name()
        .to_string_lossy()
        .to_string();
        let version = &dependency
            .req
            .comparators
            .first()
            .expect("version not found");
        hook_toml = dep_src_dir
            .join(crates_parent)
            .join(format!(
                "dep-{}.{}.{}",
                version.major,
                version.minor.unwrap_or(0),
                version.patch.unwrap_or(0)
            ))
            .join("Cargo.toml");
    }
    if !dependency.uses_default_features {
        cmd = cmd.arg("--no-default-features");
    }
    let features: Vec<&str> = Vec::new();
    if let Err(e) = cmd
        .arg("--features")
        .arg(features.join(","))
        .arg("--manifest-path")
        .arg(hook_toml)
        .arg("--target-dir")
        .arg(out_dir.clone())
        .status()
    {
        panic!("failed to build dylib {}", e);
    }
    // correct dylib path
    let hook_deps = out_dir
        .join(target)
        .join(if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        })
        .join("deps");
    let deps = out_dir
        .parent()
        .expect("can not find deps dir")
        .parent()
        .expect("can not find deps dir")
        .parent()
        .expect("can not find deps dir")
        .join("deps");
    for entry in read_dir(hook_deps.clone())
        .expect("can not find deps dir")
        .flatten()
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.contains("dep") {
            continue;
        }
        if cfg!(target_os = "linux") && file_name.ends_with(".so") {
            let from = hook_deps.join(file_name);
            let to = deps.join("libdep.so");
            copy(from.clone(), to.clone()).expect("copy to libdep.so failed!");
        } else if cfg!(target_os = "macos") && file_name.ends_with(".dylib") {
            let from = hook_deps.join(file_name);
            let to = deps.join("libdep.dylib");
            copy(from.clone(), to.clone()).expect("copy to libdep.dylib failed!");
        } else if cfg!(windows) {
            if file_name.ends_with(".dll") {
                let from = hook_deps.join(file_name);
                let to = deps.join("dep.dll");
                copy(from.clone(), to.clone()).expect("copy to dep.dll failed!");
            } else if file_name.ends_with(".lib") {
                let from = hook_deps.join(file_name);
                let to = deps.join("dep.lib");
                copy(from.clone(), to.clone()).expect("copy to dep.lib failed!");
            }
        }
    }
    // link dylib
    println!("cargo:rustc-link-search=native={:?}", deps);
    println!("cargo:rustc-link-lib=dylib=dep");
}
