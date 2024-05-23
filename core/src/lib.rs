#[cfg_attr(feature = "with_cxx", cxx::bridge)]
pub mod cxx_bridge {
    struct MySharedStruct {
        value: Vec<f32>,
    }
}

pub use cxx_bridge::*;