//! 16.1 — Tạo thread với thread::spawn, chờ bằng join, chuyển ownership bằng `move`.
//! Chạy: cargo run -p concurrency --example 01_threads

use std::thread;
use std::time::Duration;

fn main() {
    println!("1. spawn + join");
    // spawn trả về JoinHandle<T>; T là giá trị closure trả về.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  [spawned] i = {i}");
            thread::sleep(Duration::from_millis(1));
        }
        "xong" // giá trị trả về, lấy được qua join()
    });

    for i in 1..=3 {
        println!("  [main]    i = {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // Không có join(): main kết thúc → mọi thread con bị dừng, dù chưa chạy xong.
    // join() chặn thread hiện tại cho tới khi thread kia kết thúc.
    let result = handle.join().unwrap();
    println!("  thread con trả về: {result:?}");
    // Thứ tự dòng [main]/[spawned] mỗi lần chạy có thể khác nhau — do OS lập lịch.

    println!("\n2. Closure `move`: chuyển ownership vào thread");
    let v = vec![1, 2, 3];
    // Bỏ `move` → lỗi E0373: closure có thể sống lâu hơn `v`
    // (compiler không biết thread con chạy bao lâu, main có thể drop `v` trước).
    let handle = thread::spawn(move || {
        println!("  vector trong thread: {v:?}");
    });
    // println!("{v:?}"); // ❌ E0382: `v` đã bị move vào thread
    handle.join().unwrap();

    println!("\n3. Nhiều thread, gom kết quả");
    let handles: Vec<_> = (1..=4)
        .map(|id| thread::spawn(move || id * id))
        .collect();
    let squares: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  bình phương: {squares:?}");
}
