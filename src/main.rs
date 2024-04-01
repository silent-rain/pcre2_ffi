use std::{ffi::CString, net::UdpSocket, ptr};

use pcre2_sys::{pcre2_regcomp, pcre2_regexec, pcre2_regfree, regex_t, regmatch_t};

/*
// ffi
extern "C" {
    fn pcre2_regcomp(
        pattern: *const c_void,
        pattern_str: *const c_char,
        flags: c_int,
    ) -> *mut c_void;
    fn pcre2_regexec(
        code: *const c_void,
        subject: *const c_char,
        length: usize,
        matches: *mut c_void,
        flags: c_int,
    ) -> c_int;
    fn pcre2_regfree(code: *mut c_void);
}
*/

fn main() {
    let pattern =
        CString::new(r#"(?<=\d{4})([^\s\d]{3,11})(?=[^\s])"#).expect("CString::new failed");
    let subject = CString::new("a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999").expect("reading text error");

    let mut regex = regex_t {
        re_pcre2_code: ptr::null_mut(),
        re_match_data: ptr::null_mut(),
        re_endp: ptr::null(),
        re_nsub: 0,
        re_erroffset: 0,
        re_cflags: 0,
    };
    // let mut regex: std::mem::MaybeUninit<regex_t> = std::mem::MaybeUninit::uninit();

    let comp_error = unsafe { pcre2_regcomp(&mut regex as *mut regex_t, pattern.as_ptr(), 0) };
    if comp_error != 0 {
        panic!("pcre2 regcomp failed with error code: {}", comp_error);
    }

    let mut match_data = regmatch_t { rm_so: 0, rm_eo: 0 };
    let exec_error = unsafe {
        pcre2_regexec(
            &regex as *const regex_t,
            subject.as_ptr(),
            subject.as_bytes().len(),
            &mut match_data as *mut regmatch_t,
            0,
        )
    };
    if exec_error <= 0 {
        panic!(
            "No match found or an error occurred with error code: {}",
            exec_error
        );
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

// 发送数据到 Bash 脚本的函数
fn send_to_bash(data: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(data.as_bytes(), "127.0.0.1:12345")?;
    Ok(())
}
