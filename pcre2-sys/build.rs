extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 获取项目的根目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pcre2_dir = PathBuf::from(manifest_dir.clone()).join("pcre2");

    // 克隆 PCRE2 项目
    download_pcre2(pcre2_dir.clone()).expect("Failed to clone PCRE2 repository");
    // 编译 PCRE2 项目
    build_pcre2(pcre2_dir.clone()).expect("Failed to build pcre2 project with cmake");
    // 链接静态库 -  PCRE2
    link_pcre2(pcre2_dir.clone());
    // 链接静态库 - PCRE2 Posix
    link_pcre2_posix(pcre2_dir.clone());
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

/// 链接静态库 - PCRE2
fn link_pcre2(pcre2_dir: PathBuf) {
    // 指定静态库位置
    let pcre2_path = pcre2_dir
        .join("build")
        .join("CMakeFiles")
        .join(format!("{}.dir", "pcre2-8-static"))
        .join("src");
    // 检查对象文件价是否存在
    if !pcre2_path.exists() {
        panic!("Object file dir {:?} does not exist", pcre2_path);
    }

    // 创建静态库文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = PathBuf::from(out_dir.clone()).join("libpcre2.a");
    Command::new("ar")
        .args([
            "qc",
            lib_path.to_str().unwrap(),
            pcre2_dir
                .parent()
                .unwrap()
                .join("pcre2_chartables.c.o")
                .to_str()
                .unwrap(),
            pcre2_path.join("pcre2_auto_possess.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_chkdint.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_compile.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_config.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_context.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_convert.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_dfa_match.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_error.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_extuni.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_find_bracket.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_jit_compile.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_maketables.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_match.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_match_data.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_newline.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_ord2utf.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_pattern_info.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_script_run.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_serialize.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_string_utils.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_study.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_substitute.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_substring.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_tables.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_ucd.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_valid_utf.c.o").to_str().unwrap(),
            pcre2_path.join("pcre2_xclass.c.o").to_str().unwrap(),
        ])
        .status()
        .expect("Failed to create static library");

    Command::new("ranlib")
        .args([lib_path.to_str().unwrap()])
        .status()
        .expect("Failed to ranlib static library");

    // 静态库的搜索路径
    println!("cargo:rustc-link-search=native={}", out_dir);
    // 链接静态库
    println!("cargo:rustc-link-lib=static=pcre2");
}

/// 链接静态库 - PCRE2 Posix
fn link_pcre2_posix(pcre2_dir: PathBuf) {
    // 指定静态库位置
    let pcre2posix_o_path = pcre2_dir
        .join("build")
        .join("CMakeFiles")
        .join(format!("{}.dir", "pcre2-posix-static"))
        .join("src")
        .join("pcre2posix.c.o");
    // 检查对象文件是否存在
    if !pcre2posix_o_path.exists() {
        panic!("Object file {:?} does not exist", pcre2posix_o_path);
    }

    // 创建静态库文件
    // ar qc target pcre2/build/CMakeFiles/pcre2-posix-static.dir/src/pcre2posix.c.o
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = PathBuf::from(out_dir.clone()).join("libpcre2posix.a");
    Command::new("ar")
        .args([
            "qc",
            lib_path.to_str().unwrap(),
            pcre2posix_o_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to create static library");

    Command::new("ranlib")
        .args([lib_path.to_str().unwrap()])
        .status()
        .expect("Failed to ranlib static library");

    // 静态库的搜索路径
    println!("cargo:rustc-link-search=native={}", out_dir);
    // 链接静态库
    println!("cargo:rustc-link-lib=static=pcre2posix");
}

/// 生成 Rust 绑定
fn binding_pcre2(pcre2_dir: PathBuf) {
    // 指定头文件的位置
    let header_path = pcre2_dir.join("src/pcre2posix.h");

    // 使用 bindgen 生成绑定
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    // 将绑定写入 bindings.rs 文件。
    let out_path = PathBuf::from("./src").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
