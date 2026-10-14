//! Chương 16 — Fearless Concurrency. Mỗi phần là một example riêng trong `examples/`.

fn main() {
    println!("Chương 16 — Fearless Concurrency. Chạy từng ví dụ:\n");
    for name in ["01_threads", "02_channels", "03_mutex", "04_send_sync"] {
        println!("  cargo run -p concurrency --example {name}");
    }
}
