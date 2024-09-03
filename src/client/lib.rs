/*
 * @Author: x-star
 * @Date: 2024-09-02 18:22:42
 * @LastEditors: x-star
 * @LastEditTime: 2024-09-02 18:24:02
 * @FilePath: /rsdatamove/src/client/lib.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by Epic, All Rights Reserved. 
 */
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn send_data(data: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(data) };
    let data_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        stream.write_all(data_str.as_bytes()).await.unwrap();
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        buf.truncate(n);
        CString::new(buf).unwrap().into_raw()
    });

    response
}

#[no_mangle]
pub extern "C" fn free_data(data: *mut c_char) {
    if data.is_null() { return; }
    unsafe { CString::from_raw(data) };
}