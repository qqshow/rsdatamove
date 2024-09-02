package main

/*
#cgo LDFLAGS: -L. -lclient
#include <stdlib.h>

void initialize();
char* send_data(const char* data, int len);
void free_data(char* data);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	C.initialize()

	data := "Hello, Server!"
	cdata := C.CString(data)
	defer C.free(unsafe.Pointer(cdata))

	response := C.send_data(cdata, C.int(len(data)))
	defer C.free_data(response)

	fmt.Println(C.GoString(response))
}
