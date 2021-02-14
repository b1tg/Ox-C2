extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/c2.proto"], &["src/"]).unwrap();

    // let mut config = prost_build::Config::new();
    // let service_generator = prost_build::ServiceGenerator::
    // config.service_generator(service_generator);
}
