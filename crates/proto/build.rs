fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/serialized_schema.proto"], &["src/"]).unwrap();
    Ok(())
}