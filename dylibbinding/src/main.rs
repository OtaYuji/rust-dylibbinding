extern crate libc;

extern {
    fn from_the_library(a: libc::uint8_t, b: libc::uint8_t) -> libc::uint8_t;
}

fn main() {
    unsafe {
        println!("Adding: {}", from_the_library(1, 2));
    }
}