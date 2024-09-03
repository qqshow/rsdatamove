/*
 * @Author: x-star
 * @Date: 2024-09-02 18:22:42
 * @LastEditors: x-star
 * @LastEditTime: 2024-09-03 14:35:41
 * @FilePath: /rsdatamove/src/lib.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by Epic, All Rights Reserved. 
 */
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::slice;
use tokio::time::timeout;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn initialize() {
    println!("Client library initialized");
}

#[no_mangle]
pub extern "C" fn allocate_buffer(size: usize) -> *mut c_void {
    let buffer = vec![0u8; size].into_boxed_slice();
    let ptr = Box::into_raw(buffer) as *mut c_void;
    ptr
}

#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut c_void, size: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(slice::from_raw_parts_mut(ptr as *mut u8, size));
        }
    }
}

#[no_mangle]
pub extern "C" fn send_data(data: *const c_void, len: usize) -> *mut c_char {
    let data_slice = unsafe { slice::from_raw_parts(data as *const u8, len) };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        match timeout(Duration::from_secs(5), TcpStream::connect("127.0.0.1:8080")).await {
            Ok(Ok(mut stream)) => {
                stream.write_all(data_slice).await.unwrap();
                let mut buf = vec![0; 1024];
                let n = stream.read(&mut buf).await.unwrap();
                buf.truncate(n);
                CString::new(buf).unwrap().into_raw()
            },
            Ok(Err(e)) => {
                println!("连接错误: {}", e);
                CString::new(format!("连接错误: {}", e)).unwrap().into_raw()
            },
            Err(_) => {
                println!("连接超时");
                CString::new("连接超时").unwrap().into_raw()
            }
        }
    });

    response
}

#[no_mangle]
pub extern "C" fn free_data(data: *mut c_char) {
    if !data.is_null() {
        unsafe { CString::from_raw(data) };
    }
}