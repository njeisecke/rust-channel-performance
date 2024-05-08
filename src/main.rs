use std::time::Instant;

const CAPACITY: usize = 5;

fn bench(name: &str, f: fn()) {
    const RUN_COUNT: usize = 30;

    let start = Instant::now();

    for _ in 0..RUN_COUNT {
        f();
    }

    println!("{} {:?}", name, start.elapsed() / (RUN_COUNT as u32));
}

fn test_std() {
    let (tx, rx) = std::sync::mpsc::sync_channel(CAPACITY);

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

fn test_crossbeam() {
    let (tx, rx) = crossbeam::channel::bounded(CAPACITY);

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
    bench("std", test_std);
    bench("crossbeam", test_crossbeam);
}
