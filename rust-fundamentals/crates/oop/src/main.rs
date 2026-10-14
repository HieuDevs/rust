//! Chương 18 — Object-Oriented Programming Features of Rust.
//! Mỗi phần là một ví dụ thực hành riêng biệt trong thư mục `examples/`.

fn main() {
    println!("Chương 18 — Object-Oriented Programming Features. Chạy từng ví dụ:\n");
    for name in [
        "01_encapsulation",
        "02_trait_objects",
        "03_state_pattern_oop",
        "04_state_pattern_rust",
    ] {
        println!("  cargo run -p oop --example {name}");
    }
}
