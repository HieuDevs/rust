//! 15.2 — Deref: cho smart pointer hành xử như một reference.
//! Chạy: cargo run -p smart_pointers --example 02_deref

use std::ops::Deref;

// Tự viết một "Box" đơn giản (lưu ý: MyBox KHÔNG đặt dữ liệu lên heap,
// mục đích chỉ để minh họa trait Deref).
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // 1. Toán tử * với reference thường
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);

    // 2. Toán tử * với Box
    let y = Box::new(x);
    assert_eq!(5, *y);

    // 3. Toán tử * với MyBox — chỉ chạy được nhờ impl Deref.
    //    `*y` thực chất là `*(y.deref())`.
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    println!("*MyBox::new(5) = {}", *y);

    // 4. Deref coercion: &MyBox<String> -> &String -> &str
    //    Compiler tự gọi deref() bao nhiêu lần cần thiết, lúc compile, không tốn chi phí.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Nếu không có deref coercion, phải viết:
    hello(&(*m)[..]);
}
