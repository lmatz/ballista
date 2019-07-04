fn main() {

    println!("Running build.rs ...");

    // Build helloworld
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(
            &["proto/ballista/ballista.proto"],
            &["proto/ballista"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

//    // Build metadata
//    tower_grpc_build::Config::new()
//        .enable_server(true)
//        .enable_client(true)
//        .build(&["proto/metadata/metadata.proto"], &["proto/metadata"])
//        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
//
//    // Build routeguide
//    tower_grpc_build::Config::new()
//        .enable_server(true)
//        .enable_client(true)
//        .build(
//            &["proto/routeguide/route_guide.proto"],
//            &["proto/routeguide"],
//        )
//        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}