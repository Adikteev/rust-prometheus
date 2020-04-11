// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "gen")]
fn generate_protobuf_binding_file() {
    use protobuf_codegen_pure::Customize;
    let mut options = Customize::default();
    options.generate_accessors = Some(true);
    protobuf_codegen_pure::Codegen::new()
        .out_dir("proto")
        .input("proto/proto_model.proto")
        .includes(&["proto"])
        .customize(options)
        .run().unwrap();
}

#[cfg(not(feature = "gen"))]
fn generate_protobuf_binding_file() {}

fn main() {
    generate_protobuf_binding_file()
}
