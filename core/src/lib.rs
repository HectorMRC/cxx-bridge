#[cxx::bridge]
pub mod cxx_bridge {
    struct MySharedStruct {
        value: Vec<f32>,
    }
}
