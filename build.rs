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
    build_pcre2(pcre2_dir.clone()).expect("Failed to build project with cmake");
    // 链接 PCRE2 静态库
    link_pcre2(pcre2_dir);
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

    Ok(())
}

/// 链接静态库
fn link_pcre2(pcre2_dir: PathBuf) {
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
    // ar rcs target pcre2/build/CMakeFiles/pcre2-posix-static.dir/src/pcre2posix.c.o
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = PathBuf::from(out_dir.clone()).join("libpcre2posix.a");
    Command::new("ar")
        .args([
            "rcs",
            lib_path.to_str().unwrap(),
            pcre2posix_o_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to create static library");

    // 静态库的搜索路径
    println!("cargo:rustc-link-search=native={}", out_dir);
    // 链接静态库
    println!("cargo:rustc-link-lib=static=pcre2posix");
}
