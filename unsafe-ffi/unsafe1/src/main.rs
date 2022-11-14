static mut THREAD_COUNT: u32 = 4;
use std::env::var;

fn change_thread_count(count: u32) {
    unsafe {
        THREAD_COUNT = count;
    }
}

struct MyStruct(*mut u16);
unsafe impl Send for MyStruct {}
unsafe impl Sync for MyStruct {}

fn main() {
    let mut num = 23;
    let borrowed_num = &num;
    let raw_ptr = borrowed_num as *const i32;
    let raw_ptr_mut = &mut num as *mut i32;
    unsafe {
        assert!(*raw_ptr == 23);
        println!("raw1 is {}", *raw_ptr);
        println!("raw2 is {}", *raw_ptr_mut);
    }

    if let Some(thread_count) = var("THREAD_COUNT").ok() {
        change_thread_count(thread_count.parse::<u32>().unwrap());
    }

    unsafe {
        println!("Thread count is {}", THREAD_COUNT);
    }
}
