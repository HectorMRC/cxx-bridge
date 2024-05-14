#[cxx::bridge]
mod cxx_bridge {
    unsafe extern "C++" {
        include!("core/src/lib.rs.h");
        
        type MySharedStruct = core::cxx_bridge::MySharedStruct;

        include!("consumer/include/header.h");
        fn do_something(t: MySharedStruct) -> MySharedStruct;
    }

    extern "Rust" {
        fn do_another_thing(t: MySharedStruct) -> MySharedStruct;
    }
}


fn do_another_thing(t: cxx_bridge::MySharedStruct) -> cxx_bridge::MySharedStruct {
    cxx_bridge::do_something(t)
}
