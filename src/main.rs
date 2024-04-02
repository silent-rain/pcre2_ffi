use std::{
    ffi::{c_char, c_int, CString},
    net::UdpSocket,
    ptr,
};

use pcre2_sys::{pcre2_regcomp, pcre2_regerror, pcre2_regexec, pcre2_regfree, regex_t, regmatch_t};

fn main() {
    let pattern = CString::new(r"(?<=\d{4})([^\s\d]{3,11})(?=\S)").expect("CString::new failed");
    let subject = CString::new("a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999").expect("reading text error");

    let mut regex: regex_t = unsafe { std::mem::zeroed() };

    let comp_error = unsafe { pcre2_regcomp(&mut regex as *mut regex_t, pattern.as_ptr(), 0) };
    if comp_error != 0 {
        let err_msg = get_error_message(comp_error, ptr::null());
        panic!(
            "pcre2_regcomp failed with error code: {}, message: {}",
            comp_error, err_msg
        );
    }

    let mut match_data: regmatch_t = unsafe { std::mem::zeroed() };
    let exec_error = unsafe {
        pcre2_regexec(
            &regex as *const regex_t,
            subject.as_ptr(),
            subject.as_bytes().len(),
            &mut match_data as *mut regmatch_t,
            0,
        )
    };
    // 只有当返回负数时，才表示发生了错误
    if exec_error < 0 {
        let err_msg = get_error_message(exec_error, &regex);
        unsafe { pcre2_regfree(&mut regex as *mut regex_t) };
        panic!(
            "pcre2_regexec failed with error code: {}, message: {}",
            exec_error, err_msg
        );
    }
    if exec_error > 0 {
        panic!("No match found");
    }

    // 使用匹配的起始和结束偏移量来获取匹配的字符串
    let match_start = match_data.rm_so as usize;
    let match_end = match_data.rm_eo as usize;
    let subject_slice = match subject.to_str() {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let matched_string = &subject_slice[match_start..match_end];
    println!("Matched string: {:?}", matched_string);

    unsafe {
        // 释放编译的正则表达式
        pcre2_regfree(&mut regex as *mut regex_t);
    }

    // 发送结果字符串到 Bash 脚本
    send_to_bash(matched_string).expect("string sending failed");
}

/// 获取错误消息的函数
fn get_error_message(error_code: c_int, compiled_regex: *const regex_t) -> String {
    let mut err_buf = [0; 128]; // 分配一个缓冲区来存储错误消息
    unsafe {
        pcre2_regerror(
            error_code,
            compiled_regex,
            err_buf.as_mut_ptr() as *mut c_char,
            err_buf.len(),
        );
    }
    // 直接创建一个 Rust 字符串，而不是使用 CString::from_raw
    let err_msg = match std::str::from_utf8(&err_buf) {
        Ok(msg) => msg.to_owned(),
        Err(_) => "Failed to convert error message to UTF-8".to_owned(),
    };
    // 移除尾部的空字符
    err_msg.trim_end_matches('\0').to_owned()
}

/// 发送数据到 Bash 脚本的函数
fn send_to_bash(data: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(data.as_bytes(), "127.0.0.1:34254")?;
    println!("send data: {:?}", data);
    Ok(())
}
