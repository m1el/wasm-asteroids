use std::mem;

#[no_mangle]
pub extern "C"
fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub extern "C"
fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
