//! Chương 17 — Async & Await. Mỗi phần là một example riêng trong `examples/`.

fn main() {
    println!("Chương 17 — Async & Await. Chạy từng ví dụ:\n");
    for name in [
        "01_futures",
        "02_concurrency",
        "03_many_futures",
        "04_streams",
        "05_traits",
        "06_threads_tasks",
    ] {
        println!("  cargo run -p async_await --example {name}");
    }
}
