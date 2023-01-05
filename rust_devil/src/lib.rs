#[cxx::bridge]
mod ffi {

    extern "Rust" {
        fn hello_from_rust();

    }
}

pub fn hello_from_rust() {
    println!("Hello From Rust!")
}
