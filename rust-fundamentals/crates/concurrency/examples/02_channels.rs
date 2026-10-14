//! 16.2 — Message passing: các thread gửi dữ liệu cho nhau qua channel.
//! "Do not communicate by sharing memory; share memory by communicating."
//! Chạy: cargo run -p concurrency --example 02_channels

use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    println!("1. Một producer, một consumer");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("xin chào");
        tx.send(msg).unwrap();
        // println!("{msg}"); // ❌ E0382: send() đã lấy ownership của msg
    });

    // recv() chặn cho tới khi nhận được; trả Err khi mọi tx đã bị drop.
    let received = rx.recv().unwrap();
    println!("  nhận: {received}");

    println!("\n2. Gửi nhiều giá trị, dùng rx như iterator");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for word in ["một", "hai", "ba"] {
            tx.send(word).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // tx bị drop ở đây → vòng for bên dưới tự kết thúc
    });
    for msg in rx {
        println!("  nhận: {msg}");
    }

    println!("\n3. Nhiều producer: clone tx");
    let (tx, rx) = mpsc::channel();
    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in 1..=2 {
                tx.send(format!("producer {id} → tin {n}")).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
    drop(tx); // drop bản gốc, nếu không vòng for sẽ chờ mãi
    for msg in rx {
        println!("  {msg}");
    }
}
