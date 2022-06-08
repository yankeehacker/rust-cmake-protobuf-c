extern crate cmake;
use cmake::Config;

fn main() {
    let _protobuf_dst = Config::new("../deps/protobuf/cmake")
        .define("protobuf_BUILD_TESTS","OFF")
        .build();
    let _protobuf_c_dst = Config::new("../deps/protobuf-c/build-cmake")
        .cxxflag("-std=c++11")
        .build();

}
