//! 15.5 — RefCell<T>: interior mutability, kiểm tra borrow rules lúc RUNTIME.
//! Chạy: cargo run -p smart_pointers --example 05_refcell

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("1. Sửa giá trị bên trong dù biến là bất biến");
    let cell = RefCell::new(vec![1, 2, 3]); // `cell` không có `mut`
    cell.borrow_mut().push(4); // borrow_mut() -> RefMut<Vec<i32>>
    println!("  cell = {:?}", cell.borrow()); // borrow() -> Ref<Vec<i32>>

    println!("\n2. Vi phạm borrow rules → lỗi lúc chạy, không phải lúc compile");
    {
        let _r1 = cell.borrow(); // đang có 1 Ref (mượn bất biến)
        // let _w = cell.borrow_mut(); // ← sẽ PANIC: already borrowed
        match cell.try_borrow_mut() {
            Ok(_) => println!("  borrow_mut thành công"),
            Err(e) => println!("  try_borrow_mut thất bại: {e}"),
        }
    } // _r1 bị drop ở đây → hết mượn
    println!("  sau khi _r1 drop: try_borrow_mut ok = {}", cell.try_borrow_mut().is_ok());

    println!("\n3. Rc<RefCell<T>>: nhiều owner + cùng sửa được");
    let shared = Rc::new(RefCell::new(0));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;

    println!("  shared = {}", shared.borrow());
    println!("  strong_count = {}", Rc::strong_count(&shared));
}
