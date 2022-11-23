fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:32b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b);
    assert_eq!(a, b);

    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            println!("\n");
        }
    }
}
