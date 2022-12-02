fn main() {
    let mut n_nonzero = 0;

    for i in 0..10000 {
        let ptr = i as *const u8;
        let bytes = unsafe { *ptr };

        if bytes != 0 {
            n_nonzero += 1;
        }
    }
    println!("non-zero bytes in memory: {}", n_nonzero);
}
