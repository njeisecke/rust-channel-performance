use std::time::Instant;

fn bench(name: &str, f: fn()) {
    const RUN_COUNT: usize = 30;

    let start = Instant::now();

    for _ in 0..RUN_COUNT {
        f();
    }

    println!("{} {:?}", name, start.elapsed() / (RUN_COUNT as u32));
}

fn test_foo() {
    let (tx, rx) = std::sync::mpsc::sync_channel(5);

    let handle = std::thread::spawn(move || {
        let mut acc: i64 = 0;
        while let Ok(value) = rx.recv() {
            acc += value;
        }
        acc
    });

    for v in 0..100_000 {
        tx.send(v).unwrap();
    }
    drop(tx);

    let sum = handle.join().unwrap();

    println!("sum {}", sum);
}

fn main() {
    bench("foo", test_foo);
}
