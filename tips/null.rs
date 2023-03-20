use std::ptr;

fn main() {
    let p: *mut i32= ptr::null_mut();
    assert!(p.is_null());
}
