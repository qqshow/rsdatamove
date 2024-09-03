/*
 * @Author: x-star
 * @Date: 2024-09-02 18:13:23
 * @LastEditors: x-star
 * @LastEditTime: 2024-09-03 13:32:16
 * @FilePath: /rsdatamove/src/test_client.go
 * @Description:
 *
 * Copyright (c) 2024 by Epic, All Rights Reserved.
 */
package main

/*
#cgo LDFLAGS: -L. -lclient
#include <stdlib.h>

void initialize();
void* allocate_buffer(size_t size);
void free_buffer(void* ptr, size_t size);
char* send_data(const void* data, size_t len);
void free_data(char* data);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	C.initialize()

	data := "Hello, Server! I'm go client"
	bufferSize := C.size_t(len(data))
	buffer := C.allocate_buffer(bufferSize)
	defer C.free_buffer(buffer, bufferSize)

	// 将数据复制到分配的缓冲区
	copy(unsafe.Slice((*byte)(buffer), bufferSize), []byte(data))

	response := C.send_data(buffer, bufferSize)
	defer C.free_data(response)

	fmt.Println(C.GoString(response))
}
