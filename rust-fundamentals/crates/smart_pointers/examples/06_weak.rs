//! 15.6 — Reference cycle & Weak<T>: tránh rò rỉ bộ nhớ.
//! Chạy: cargo run -p smart_pointers --example 06_weak

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Cây: cha SỞ HỮU con (Rc), con chỉ THAM CHIẾU YẾU tới cha (Weak).
// Nếu parent cũng là Rc → cha giữ con, con giữ cha → strong_count không bao giờ về 0 → leak.
#[allow(dead_code)] // `children` chỉ dùng để giữ ownership các node con
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn counts(name: &str, n: &Rc<Node>) {
    println!(
        "  {name}: strong = {}, weak = {}",
        Rc::strong_count(n),
        Rc::weak_count(n)
    );
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("1. Mới tạo leaf");
    println!("  leaf.parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    counts("leaf", &leaf);

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Rc::downgrade tạo Weak, tăng weak_count chứ không tăng strong_count
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("\n2. Gắn leaf vào branch");
        // upgrade(): Weak -> Option<Rc>, trả về None nếu giá trị đã bị drop
        println!("  leaf.parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
        counts("branch", &branch);
        counts("leaf", &leaf);
    } // branch: strong 1 -> 0 → bị drop (weak_count không giữ nó lại)

    println!("\n3. branch ra khỏi scope");
    println!("  leaf.parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    counts("leaf", &leaf);
}
