use std::env;
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let project_root = Path::new(&current_dir);
    let libraries_root = if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86") {
            project_root.join("lib").join("linux").join("x86")
        } else { // x86_64 only
            project_root.join("lib").join("linux").join("x86_64")
        }
    } else if cfg!(target_os = "macos") { // universal library
        project_root.join("lib").join("osx")
    } else { // windows
        if cfg!(target_arch = "x86") {
            project_root.join("lib").join("windows").join("x86")
        } else { // x86_64 only
            project_root.join("lib").join("windows").join("x86_64")
        }
    };

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=DYLD_FALLBACK_LIBRARY_PATH=$DYLD_FALLBACK_LIBRARY_PATH:{}", libraries_root.display());
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-env=LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{}", libraries_root.display());
    } else { // windows
        println!("cargo:rustc-env=PATH=$PATH;{}", libraries_root.display());
    }
    println!("cargo:rustc-link-search=native={}", libraries_root.display());
}
