//! 17.2 — Concurrency bằng async: spawn_task, join, channel async.
//! So sánh với chương 16 (thread::spawn, join(), mpsc).
//! Chạy: cargo run -p async_await --example 02_concurrency

use std::time::Duration;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn main() {
    trpl::block_on(async {
        println!("1. spawn_task ≈ thread::spawn, nhưng là task nhẹ do runtime quản lý");
        let handle = trpl::spawn_task(async {
            for i in 1..=5 {
                println!("  [task] {i}");
                trpl::sleep(ms(10)).await;
            }
        });
        for i in 1..=3 {
            println!("  [main] {i}");
            trpl::sleep(ms(10)).await;
        }
        handle.await.unwrap(); // ≈ handle.join().unwrap()

        println!("\n2. trpl::join: chạy 2 future đồng thời, không cần spawn");
        let fut1 = async {
            for i in 1..=3 {
                println!("  [fut1] {i}");
                trpl::sleep(ms(10)).await;
            }
        };
        let fut2 = async {
            for i in 1..=3 {
                println!("  [fut2] {i}");
                trpl::sleep(ms(10)).await;
            }
        };
        trpl::join(fut1, fut2).await; // join "công bằng": luân phiên poll từng future

        println!("\n3. Channel async: recv().await thay vì recv() block");
        let (tx, mut rx) = trpl::channel();
        let tx2 = tx.clone();

        // `async move` quan trọng: tx bị move vào block → drop khi block xong
        // → rx.recv() trả None → consumer kết thúc. Bỏ `move` thì chương trình treo.
        let producer1 = async move {
            for w in ["xin", "chào", "từ", "tx1"] {
                tx.send(format!("tx1: {w}")).unwrap();
                trpl::sleep(ms(30)).await;
            }
        };
        let producer2 = async move {
            for w in ["thêm", "tin", "từ", "tx2"] {
                tx2.send(format!("tx2: {w}")).unwrap();
                trpl::sleep(ms(45)).await;
            }
        };
        let consumer = async {
            while let Some(msg) = rx.recv().await {
                println!("  nhận {msg}");
            }
        };
        trpl::join!(producer1, producer2, consumer); // macro join! nhận số lượng future tùy ý
    });
}
