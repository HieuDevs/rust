//! 15.3 — Drop: chạy code dọn dẹp khi giá trị ra khỏi scope.
//! Chạy: cargo run -p smart_pointers --example 03_drop

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("  [drop] `{}`", self.data);
    }
}

fn main() {
    println!("1. Thứ tự drop: ngược với thứ tự khai báo");
    {
        let _c = CustomSmartPointer { data: String::from("c - tạo trước") };
        let _d = CustomSmartPointer { data: String::from("d - tạo sau") };
        println!("  Đã tạo c và d, sắp ra khỏi scope...");
    }

    println!("\n2. Drop sớm bằng std::mem::drop");
    let e = CustomSmartPointer { data: String::from("e") };
    println!("  Đã tạo e");
    // e.drop();  // ❌ E0040: không được gọi drop() trực tiếp (sẽ bị double free)
    drop(e); // ✅ std::mem::drop — nhận ownership rồi để giá trị ra khỏi scope
    println!("  e đã bị drop trước khi main kết thúc");

    println!("\n3. Hết main");
}
