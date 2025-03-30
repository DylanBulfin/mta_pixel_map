fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["src/protos"])
        // Inputs must reside in some of include paths.
        .input("src/protos/gtfs-realtime.proto")
        .input("src/protos/gtfs-realtime-NYCT.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("src/protos")
        .run_from_script();
}
