#![crate_type = "staticlib"]
#![no_main]

#[no_mangle]
extern "C" {
    static mut Global: i32;
    fn foo();
}

#[no_mangle]
pub extern "C" fn bar() {
    let mut a = 100;
    unsafe {
        Global = 20;
    }
    a += 200;
}

#[no_mangle]
fn foo_bar() {
    unsafe {
        foo();
        bar();
    }
}
