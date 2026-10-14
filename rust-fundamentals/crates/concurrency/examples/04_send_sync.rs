//! 16.4 — Send & Sync: trait đánh dấu kiểu nào an toàn khi dùng giữa các thread.
//! Chạy: cargo run -p concurrency --example 04_send_sync
//!
//! - Send: được CHUYỂN ownership sang thread khác.
//! - Sync: được CHIA SẺ reference (&T) giữa nhiều thread. (T: Sync ⇔ &T: Send)
//! Cả hai được compiler tự implement nếu mọi field bên trong đều Send/Sync.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Hàm "kiểm tra lúc compile": chỉ gọi được nếu T thỏa trait bound.
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    println!("1. Kiểm tra lúc compile");
    assert_send::<i32>();
    assert_send::<String>();
    assert_send::<Arc<i32>>();
    assert_sync::<Mutex<i32>>();
    // assert_send::<Rc<i32>>();      // ❌ Rc<i32> cannot be sent between threads safely
    // assert_sync::<RefCell<i32>>(); // ❌ RefCell<i32> cannot be shared between threads safely
    println!("  i32, String, Arc<i32>: Send ✓   Mutex<i32>: Sync ✓");

    println!("\n2. Rc không qua được thread — lỗi bị chặn lúc compile");
    let rc = Rc::new(RefCell::new(0));
    *rc.borrow_mut() += 1; // dùng Rc trong 1 thread thì OK
    // thread::spawn(move || { *rc.borrow_mut() += 1; }); // ❌ E0277: `Rc<..>` cannot be sent
    println!("  rc = {} (chỉ dùng trong main thread)", rc.borrow());

    println!("\n3. Thay bằng Arc<Mutex<T>> thì chạy được");
    let shared = Arc::new(Mutex::new(0));
    let s = Arc::clone(&shared);
    thread::spawn(move || *s.lock().unwrap() += 1).join().unwrap();
    println!("  shared = {}", shared.lock().unwrap());

    println!("\n4. Bonus: thread::scope — mượn dữ liệu, không cần Arc hay move");
    // Scope đảm bảo mọi thread con kết thúc trước khi scope đóng,
    // nên compiler cho phép thread mượn biến cục bộ.
    let mut numbers = vec![1, 2, 3];
    let total = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| *total.lock().unwrap() += numbers.iter().sum::<i32>());
        s.spawn(|| println!("  đọc numbers từ thread khác: {numbers:?}"));
    }); // tự join tất cả ở đây
    numbers.push(4); // vẫn dùng được, vì chưa bị move
    println!("  total = {}, numbers = {numbers:?}", total.lock().unwrap());
}
