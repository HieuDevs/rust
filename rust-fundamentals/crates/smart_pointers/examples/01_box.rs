//! 15.1 — Box<T>: đặt giá trị lên heap, một owner duy nhất.
//! Chạy: cargo run -p smart_pointers --example 01_box

// Kiểu đệ quy: nếu viết `Cons(i32, List)` compiler báo lỗi E0072
// vì không tính được kích thước. Box có kích thước cố định (1 con trỏ),
// nên phá được vòng đệ quy vô hạn.
#[allow(dead_code)] // field chỉ được đọc qua Debug
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 1. Giá trị đơn giản trên heap
    let b = Box::new(5);
    println!("b = {b}");
    // `b` ra khỏi scope → cả Box (trên stack) lẫn số 5 (trên heap) được giải phóng.

    // 2. Cons list: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {list:?}");

    // 3. So sánh kích thước
    println!("size_of::<i32>()      = {}", std::mem::size_of::<i32>());
    println!(
        "size_of::<Box<i32>>() = {}",
        std::mem::size_of::<Box<i32>>()
    );
    println!("size_of::<List>()     = {}", std::mem::size_of::<List>());
}
