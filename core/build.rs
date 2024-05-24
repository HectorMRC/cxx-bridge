fn main() {
    #[cfg(feature = "with_cxx")]
    cxx_build::bridge("src/lib.rs")
        .std("c++11")
        .compile("core");
}
