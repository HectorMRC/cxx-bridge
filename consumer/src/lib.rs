#[cxx::bridge]
mod cxx_bridge {
    unsafe extern "C++" {
        include!("core/src/lib.rs.h");
        
        type MySharedStruct = core::cxx_bridge::MySharedStruct;

        fn do_something(t: MySharedStruct) -> MySharedStruct;
    }

    extern "Rust" {
        fn do_another_thing(t: MySharedStruct) -> MySharedStruct;
    }
}

use core::cxx_bridge::MySharedStruct;

fn do_another_thing(t: MySharedStruct) -> MySharedStruct {
    cxx_bridge::do_something(t)
}
