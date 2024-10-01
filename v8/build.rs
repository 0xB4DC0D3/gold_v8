use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cpp/v8.cc");
    println!("cargo:rerun-if-changed=src/cpp/v8.h");

    fetch_and_build_v8();
    link_v8();

    cc::Build::new()
        .file("src/cpp/v8.cc")
        .include(
            get_v8_dir()
                .expect("Can't get V8 directory for cxx")
                .join("v8")
                .join("include"),
        )
        .flag("/Zc:__cplusplus")
        .flag("/std:c++20")
        .compile("cc_v8");

    bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("src/cpp/v8.hpp")
        .clang_args(["-x", "c++"])
        .clang_arg(format!(
            "-I{}",
            get_v8_dir()
                .expect("Can't get V8 directory for cxx")
                .join("v8")
                .join("include")
                .to_str()
                .unwrap()
        ))
        .clang_arg("-std=c++20")
        .allowlist_item("v8cxx__.*")
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings for V8")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Unable to save bindings");
}

fn get_build_dir() -> Option<PathBuf> {
    Some(
        PathBuf::from(env::var("OUT_DIR").ok()?)
            .parent()?
            .parent()?
            .parent()?
            .parent()?
            .to_path_buf(),
    )
}

fn link_v8() {
    println!(
        "cargo:rustc-link-search=native={}",
        get_v8_build_dir().unwrap().to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=static=v8_monolith");
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=dbghelp");
}

fn fetch_and_build_v8() {
    let build_profile = env::var("PROFILE").expect("Can't get build profile");

    if is_v8_built(&build_profile) {
        return;
    }

    env::set_var("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");

    let build_dir = get_build_dir().expect("Can't get build directory");
    let depot_tools_dir = build_dir.join("depot_tools");
    let v8_dir = get_v8_dir().expect("Can't get V8 directory");

    if !depot_tools_dir.exists() {
        assert!(Command::new("git")
            .arg("clone")
            .arg("https://chromium.googlesource.com/chromium/tools/depot_tools.git")
            .current_dir(build_dir)
            .status()
            .unwrap()
            .success());
    }

    assert!(Command::new("python")
        .args(["-m", "pip", "install", "httplib2"])
        .status()
        .expect("Failed to install httplib2 for Python")
        .success());

    assert!(Command::new("python")
        .arg(depot_tools_dir.join("gclient").with_extension("py"))
        .args(["config", "https://chromium.googlesource.com/v8/v8.git"])
        .current_dir(v8_dir.as_path())
        .status()
        .expect("Failed to configure gclient")
        .success());

    assert!(Command::new("python")
        .arg(depot_tools_dir.join("gclient").with_extension("py"))
        .arg("sync")
        .current_dir(v8_dir.as_path())
        .status()
        .expect("Failed to sync V8")
        .success());

    let gn_args = [
        "is_debug=false",
        "is_component_build=false",
        "use_custom_libcxx=false",
        "use_lld=false",
        "v8_static_library=true",
        "v8_monolithic=true",
        "v8_use_external_startup_data=false",
        "target_cpu=\"x86\"",
        "treat_warnings_as_errors=false",
    ];

    assert!(Command::new("python")
        .arg(depot_tools_dir.join("gn").with_extension("py"))
        .arg("gen")
        .arg(format!("out.gn/x86.{build_profile}"))
        .arg(format!("--args={}", gn_args.join(" ")))
        .current_dir(&v8_dir.join("v8"))
        .status()
        .expect("Failed to run gn to create a build configuration")
        .success());

    assert!(Command::new("python")
        .arg(depot_tools_dir.join("ninja").with_extension("py"))
        .arg("-C")
        .arg(format!("out.gn/x86.{build_profile}"))
        .current_dir(&v8_dir.join("v8"))
        .status()
        .expect("Failed to build V8")
        .success());
}

fn is_v8_built(build_profile: &str) -> bool {
    get_v8_dir().map_or(false, |v8_dir| {
        v8_dir
            .join("v8")
            .join("out.gn")
            .join(format!("x86.{build_profile}"))
            .join("obj")
            .join("v8_monolith")
            .with_extension("lib")
            .exists()
    })
}

fn get_v8_build_dir() -> Option<PathBuf> {
    Some(
        get_v8_dir()?
            .join("v8")
            .join("out.gn")
            .join(format!("x86.{}", env::var("PROFILE").unwrap()))
            .join("obj"),
    )
}

fn get_v8_dir() -> Option<PathBuf> {
    let dir = get_build_dir()?.join("v8");

    if !dir.exists() {
        std::fs::create_dir(&dir).unwrap();
    }

    Some(dir)
}
