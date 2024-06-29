#![no_main]

#[no_mangle]
extern "C" {
    static mut Global: i32;
    fn foo();
}

#[no_mangle]
pub extern "C" fn bar() {
    unsafe {
        Global = 20;
    }
}
