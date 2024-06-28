fn main() {
    extern "C" {
        fn bar();
    }
    unsafe {
        bar();
    }
}
