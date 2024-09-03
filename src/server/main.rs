/*
 * @Author: x-star
 * @Date: 2024-09-02 18:22:44
 * @LastEditors: x-star
 * @LastEditTime: 2024-09-03 13:40:40
 * @FilePath: /rsdatamove/src/server/main.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by Epic, All Rights Reserved. 
 */
use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::io; // 添加这一行

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器正在监听 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("新的连接: {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(_) => return,
                };

                // 将接收到的数据转换为字符串并打印
                let received = String::from_utf8_lossy(&buf[0..n]);
                println!("收到来自 {} 的数据: {}", addr, received);

                // 发送响应
                let response = format!("服务器已收到: {}", received);
                if socket.write_all(response.as_bytes()).await.is_err() {
                    return;
                }
            }
        });
    }
}