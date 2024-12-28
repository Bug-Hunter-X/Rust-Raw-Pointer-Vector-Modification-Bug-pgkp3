fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let mut v_clone = v.clone();
    let ptr = v_clone.as_mut_ptr();
    unsafe {
        std::ptr::copy(ptr.add(0),ptr, len);
        *ptr = 10;        
        
    }
    println!("{:?}", v_clone);
}