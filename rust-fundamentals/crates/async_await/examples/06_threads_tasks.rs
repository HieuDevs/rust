//! 17.6 — Kết hợp future, task và thread.
//! Chạy: cargo run -p async_await --example 06_threads_tasks
//!
//! Quy tắc thực tế:
//!   - I/O-bound, số lượng lớn (mạng, file)  → async task
//!   - CPU-bound (tính toán nặng)            → thread (Tokio: spawn_blocking / rayon)
//!   - Thường kết hợp cả hai.

use std::thread;
use std::time::{Duration, Instant};

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn main() {
    println!("1. Thread gửi, async nhận (channel nối 2 thế giới)");
    let (tx, mut rx) = trpl::channel();
    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i).unwrap(); // send() của trpl::channel là hàm thường, gọi từ thread được
            thread::sleep(ms(20));
        }
    });
    trpl::block_on(async {
        while let Some(i) = rx.recv().await {
            println!("  async nhận từ thread: {i}");
        }
    });

    println!("\n2. Task rất nhẹ: 10.000 task cùng chờ 10ms");
    trpl::block_on(async {
        let start = Instant::now();
        let handles: Vec<_> = (0..10_000u64)
            .map(|i| {
                trpl::spawn_task(async move {
                    trpl::sleep(ms(10)).await;
                    i
                })
            })
            .collect();
        let mut sum = 0;
        for h in handles {
            sum += h.await.unwrap();
        }
        // 10.000 OS thread sẽ tốn hàng GB stack; 10.000 task chỉ vài MB.
        println!("  sum = {sum}, tổng thời gian ~{}ms", start.elapsed().as_millis());
    });

    println!("\n3. Việc CPU nặng → đẩy sang thread, async vẫn chạy mượt");
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        thread::spawn(move || {
            let s: u64 = (1..=20_000_000u64).sum(); // nặng, nếu chạy trong async sẽ chặn runtime
            tx.send(s).unwrap();
        });
        let ticker = async {
            for i in 1..=3 {
                println!("  [async] vẫn phản hồi {i}");
                trpl::sleep(ms(10)).await;
            }
        };
        let result = async { rx.recv().await.unwrap() };
        let ((), sum) = trpl::join(ticker, result).await;
        println!("  thread tính xong: {sum}");
    });
}
