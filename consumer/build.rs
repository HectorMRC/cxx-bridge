fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++11")
        .compile("client");
}
