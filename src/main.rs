fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let mut threads = vec![];

    for i in 0..8 {
        let handle = std::thread::spawn(move || {
            let result = fibonacci(4000);
            println!("Thread {} finished with result: {}", i, result);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}
