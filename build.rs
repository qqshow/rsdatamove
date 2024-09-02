fn main() {
    prost_build::compile_protos(&["src/proto/transport.proto"], &["src/proto/"]).unwrap();
}