#[link(name="hello", kind="static")]
extern {
    fn hello();
    fn add(x: i32, y: i32) -> i32;
    fn puts(s: *const std::os::raw::c_char);
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        hello();
        println!("add(1, 2) = {}", add(1, 2));
        let s = "ABC";
        let s_with_null = std::ffi::CString::new(s).unwrap();
        puts(s_with_null.as_ptr());
        println!("{}", strlen(s_with_null.as_ptr()));
    };
    let s = "ABC";
    let n = unsafe {
        let s_with_null = std::ffi::CString::new(s).unwrap();
        strlen(s_with_null.as_ptr())
    };
    println!("{}", n);
}
