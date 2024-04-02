// extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen::{CargoCallbacks, EnumVariation};

fn main() {
    // 获取项目的根目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pcre2_dir = PathBuf::from(manifest_dir.clone()).join("pcre2");

    // 克隆 PCRE2 项目
    download_pcre2(pcre2_dir.clone()).expect("Failed to clone PCRE2 repository");
    // 编译 PCRE2 项目
    build_pcre2(pcre2_dir.clone()).expect("Failed to build pcre2 project with cmake");
    // 链接静态库
    link_pcre2(pcre2_dir.clone());
    // 生成 Rust 绑定
    binding_pcre2(pcre2_dir);
}

/// 克隆 PCRE2 项目
fn download_pcre2(pcre2_dir: PathBuf) -> std::io::Result<()> {
    // 检查 pcre2 目录是否存在
    if !pcre2_dir.exists() {
        Command::new("git")
            .args([
                "clone",
                "https://github.com/PhilipHazel/pcre2",
                pcre2_dir.to_str().unwrap(),
            ])
            .status()?;
    }
    Ok(())
}

/// 编译 PCRE2 项目
fn build_pcre2(pcre2_dir: PathBuf) -> std::io::Result<()> {
    // 创建构建目录
    let build_dir = pcre2_dir.join("build");
    if !build_dir.exists() {
        Command::new("mkdir").arg(&build_dir).status()?;
    }

    // 进入构建目录
    env::set_current_dir(&build_dir)?;

    // 运行 CMake 来生成构建系统
    Command::new("cmake").arg("..").status()?;

    // 用生成的构建系统编译项目
    Command::new("cmake").arg("--build").arg(".").status()?;

    // 退出构建目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    env::set_current_dir(manifest_dir)?;
    Ok(())
}

/// 链接静态库
fn link_pcre2(pcre2_dir: PathBuf) {
    let lib_path = pcre2_dir.join("build");
    // 搜索静态库路径
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    // 链接静态库
    println!("cargo:rustc-link-lib=static=pcre2-8");
    println!("cargo:rustc-link-lib=static=pcre2-posix");
}

/// 生成 Rust 绑定
fn binding_pcre2(pcre2_dir: PathBuf) {
    // 指定头文件的位置
    let header_path = pcre2_dir.join("src/pcre2posix.h");

    // 使用 bindgen 生成绑定
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .generate_comments(true)
        // u128 类型在 FFI（外部函数接口）中并不是安全的，因为它没有一个已知的稳定的 ABI（应用程序二进制接口）。
        // 这意味着不同的编译器或不同的平台可能会以不同的方式处理这种类型，从而导致潜在的不兼容问题。
        .blocklist_type("u128") // 阻止生成包含 u128 的绑定
        .raw_line("pub type u128 = u64;") // u128 替换为 u64
        .use_core()
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 将绑定写入 bindings.rs 文件。
    let out_path = PathBuf::from("./src").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
