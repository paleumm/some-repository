use std::thread;

fn main() {
    let mut child_threads = Vec::new();

    for _ in 1..5 {
        let handles = thread::spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });

        child_threads.push(handles);
    }
    for i in child_threads {
        i.join().unwrap();
    }
}
