//! 15.4 — Rc<T>: đếm tham chiếu, cho phép nhiều owner (chỉ trong 1 thread).
//! Chạy: cargo run -p smart_pointers --example 04_rc

use std::rc::Rc;

#[allow(dead_code)] // field chỉ được đọc qua Debug
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    //      b: 3 ─┐
    //            ├─> a: 5 -> 10 -> Nil
    //      c: 4 ─┘
    // Với Box, `a` bị move vào `b` nên không dùng lại cho `c` được.
    // Với Rc, cả b và c cùng sở hữu a.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("sau khi tạo a      : strong_count = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a)); // Rc::clone chỉ tăng bộ đếm, KHÔNG deep copy
    println!("sau khi tạo b      : strong_count = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("sau khi tạo c      : strong_count = {}", Rc::strong_count(&a));
        println!("c = {c:?}");
    }
    println!("c ra khỏi scope    : strong_count = {}", Rc::strong_count(&a));

    // Rc<T> chỉ cho reference BẤT BIẾN. Muốn sửa → cần RefCell (xem 05_refcell).
}
