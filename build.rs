fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("compiling kms.proto file");
    tonic_build::compile_protos("./src/api/grpc/proto/kms.proto")?;
    Ok(())
}
