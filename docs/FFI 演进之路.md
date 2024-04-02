# FFI 演进之路

## Pcre2 手工编译

- 使用 Pcre2 项目项目自身的能力进行编译；
- 手动进行编译静态库文件；

```shell
# 克隆 Pcre2 项目
git clone https://github.com/PhilipHazel/pcre2

# 进入项目
cd pcre2

# 创建一个构建目录并进入该目录
mkdir build && cd build

# 运行 CMake 来生成构建系统
cmake ..

# 用生成的构建系统编译项目
cmake --build .
```

## Rust 与 Cmake

- 将 Cmake 执行继承到 `build.rs` 中；
- 手工实现编译项目；
- 每次实现自动化进行编译；

```rust
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
```

## Rust 与 Cmake 精简编译过程

- Pcre2 项目中的项目编译脚本中已经实现了编译输出产物 `*.a`；
- 进一步精简编译过程；
- 同时修复一些编译过程中的告警；

```rust
// extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen::{CargoCallbacks, EnumVariation};

fn main() {
    // ...
    // 链接静态库
    link_pcre2(pcre2_dir.clone());
    // 生成 Rust 绑定
    binding_pcre2(pcre2_dir);
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
```

## Rust && CC

- 使用 Rust 的 cc 库替代 Cmake 编译；
- cc 与 Rust 继承，可高度实现自定义编译指令，而无需修改原项目编译指令；

## Rust FFI 绑定

- 在 Rust 中手工实现 C 函数绑定；

```rust
extern "C" {
    pub fn pcre2_regcomp(
        arg1: *mut regex_t,
        arg2: *const ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn pcre2_regexec(
        arg1: *const regex_t,
        arg2: *const ::core::ffi::c_char,
        arg3: usize,
        arg4: *mut regmatch_t,
        arg5: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn pcre2_regerror(
        arg1: ::core::ffi::c_int,
        arg2: *const regex_t,
        arg3: *mut ::core::ffi::c_char,
        arg4: usize,
    ) -> usize;
}
extern "C" {
    pub fn pcre2_regfree(arg1: *mut regex_t);
}
```

## Rust Bindgen 绑定

- 使用 bindgen 库进行自动化绑定；
- 在与一些较大的库进行绑定时推荐使用该方式进行绑定，大大减少工作量；

```rust
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
```

## Pcre2 绑定库封装

- 对 Pcre2 的 Rust 绑定进行进一步封装，提高可用性和复用性；
- 结合结构体进行封装，减少大量的样板代码；
