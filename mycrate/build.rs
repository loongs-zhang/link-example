use std::env::var;
use std::fs::{read_dir, rename};
use std::path::PathBuf;

fn main() {
    //fix dylib name
    let out_dir = PathBuf::from(var("OUT_DIR").expect("env not found"));
    let deps = out_dir
        .parent()
        .expect("can not find deps dir")
        .parent()
        .expect("can not find deps dir")
        .parent()
        .expect("can not find deps dir")
        .join("deps");
    let lib_names = [
        String::from("libdep.so"),
        String::from("libdep.dylib"),
        String::from("dep.lib"),
    ];
    for entry in read_dir(deps.clone())
        .expect("Failed to read deps")
        .flatten()
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !file_name.contains("dep") {
            continue;
        }
        if lib_names.contains(&file_name) {
            break;
        }
        if file_name.eq("dep.dll") {
            continue;
        }
        if cfg!(target_os = "linux") && file_name.ends_with(".so") {
            rename(deps.join(file_name), deps.join("libdep.so"))
                .expect("rename to libdep.so failed!");
        } else if cfg!(target_os = "macos") && file_name.ends_with(".dylib") {
            rename(deps.join(file_name), deps.join("libdep.dylib"))
                .expect("rename to libdep.dylib failed!");
        } else if cfg!(windows) {
            if file_name.ends_with(".dll") {
                rename(deps.join(file_name), deps.join("dep.dll"))
                    .expect("rename to dep.dll failed!");
            } else if file_name.ends_with(".lib") {
                //fixme when link targets like ${arch}-pc-windows-msvc, this will not work
                // it seems that .dll.lib has not been generated at this time
                rename(deps.join(file_name), deps.join("dep.lib"))
                    .expect("rename to dep.lib failed!");
            }
        }
    }
    //link hook dylib
    println!("cargo:rustc-link-lib=dylib=dep");
}
