use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use prost::Message;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Once;

mod transport {
    include!(concat!(env!("OUT_DIR"), "/transport.rs"));
}

static INIT: Once = Once::new();

#[no_mangle]
pub extern "C" fn initialize() {
    INIT.call_once(|| {
        tokio::runtime::Runtime::new().unwrap();
    });
}

#[no_mangle]
pub extern "C" fn send_data(data: *const u8, len: usize) -> *mut c_char {
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };
    let packet = transport::DataPacket {
        data: data_slice.to_vec(),
    };

    let mut buf = Vec::new();
    packet.encode(&mut buf).unwrap();

    let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        stream.write_all(&buf).await.unwrap();

        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await.unwrap();
        response.truncate(n);

        CString::new(response).unwrap().into_raw()
    });

    result
}

#[no_mangle]
pub extern "C" fn free_data(data: *mut c_char) {
    unsafe {
        if data.is_null() { return }
        CString::from_raw(data)
    };
}