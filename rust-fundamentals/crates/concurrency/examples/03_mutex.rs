//! 16.3 — Shared state: nhiều thread cùng truy cập một dữ liệu qua Arc<Mutex<T>>.
//! Chạy: cargo run -p concurrency --example 03_mutex

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("1. Mutex trong một thread");
    let m = Mutex::new(5);
    {
        // lock() chặn tới khi lấy được khóa, trả về MutexGuard<i32>.
        // MutexGuard implement Deref (dùng như &mut i32) và Drop (tự mở khóa).
        let mut num = m.lock().unwrap();
        *num = 6;
    } // guard bị drop → mở khóa
    println!("  m = {m:?}");

    println!("\n2. Arc<Mutex<T>>: 10 thread cùng tăng một bộ đếm");
    // Rc<Mutex<_>> sẽ không compile: Rc không phải Send (xem 04_send_sync).
    // Arc = Atomically Reference Counted, bản thread-safe của Rc.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("  kết quả = {} (luôn đúng 10000, không bị data race)", *counter.lock().unwrap());

    println!("\n3. So sánh với chương 15");
    println!("  Rc<RefCell<T>>  — 1 thread,  kiểm tra borrow lúc chạy, sai thì panic");
    println!("  Arc<Mutex<T>>   — nhiều thread, khóa lúc chạy, sai thì có thể deadlock");
}
