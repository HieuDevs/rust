//! Chương 15 — Smart Pointers. Mỗi phần là một example riêng trong `examples/`.

fn main() {
    println!("Chương 15 — Smart Pointers. Chạy từng ví dụ:\n");
    for name in [
        "01_box",
        "02_deref",
        "03_drop",
        "04_rc",
        "05_refcell",
        "06_weak",
    ] {
        println!("  cargo run -p smart_pointers --example {name}");
    }
}
