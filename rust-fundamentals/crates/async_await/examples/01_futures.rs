//! 17.1 — Future và cú pháp async/await.
//! Chạy: cargo run -p async_await --example 01_futures

use std::time::{Duration, Instant};
use trpl::Either;

// `async fn` không chạy ngay: gọi nó chỉ trả về một Future<Output = String>.
// Compiler biến thân hàm thành một state machine, mỗi `.await` là một điểm có thể tạm dừng.
async fn fetch(name: &str, ms: u64) -> String {
    println!("  [{name}] bắt đầu");
    trpl::sleep(Duration::from_millis(ms)).await; // giả lập chờ mạng — KHÔNG block thread
    println!("  [{name}] xong sau {ms}ms");
    format!("dữ liệu từ {name}")
}

// `async fn main` không được phép nếu không có runtime.
// Rust không kèm sẵn runtime → phải tự chọn (Tokio, smol, ...). Ở đây trpl bọc Tokio.
fn main() {
    trpl::block_on(async {
        println!("1. Future là lazy: tạo ra chưa chạy, phải .await mới chạy");
        let fut = fetch("lazy", 10);
        println!("  đã tạo future nhưng chưa có dòng [lazy] nào được in...");
        let data = fut.await;
        println!("  => {data}");

        println!("\n2. await lần lượt: B chỉ bắt đầu khi A xong");
        let start = Instant::now();
        let a = fetch("A", 100).await;
        let b = fetch("B", 100).await;
        println!("  {a} | {b} — mất ~{}ms", start.elapsed().as_millis());

        println!("\n3. select: chạy đồng thời, lấy cái xong trước, hủy cái còn lại");
        let start = Instant::now();
        let winner = match trpl::select(fetch("chậm", 200), fetch("nhanh", 50)).await {
            Either::Left(d) => d,
            Either::Right(d) => d,
        };
        // Không có dòng "[chậm] xong": future thua bị drop = bị hủy.
        println!("  thắng: {winner} — mất ~{}ms", start.elapsed().as_millis());
    });
}
