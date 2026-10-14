//! 18.2 — Trait Objects: Cho phép một tập hợp chứa nhiều kiểu dữ liệu khác nhau.
//! Chạy: cargo run -p oop --example 02_trait_objects

// 1. Định nghĩa trait chung
pub trait Draw {
    fn draw(&self);
}

// 2. Struct Screen lưu trữ danh sách các trait object `Box<dyn Draw>`
// Nhờ `dyn Draw`, các phần tử trong Vec KHÔNG cần phải cùng một kiểu struct cụ thể!
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // Gọi thông qua dynamic dispatch (vtable)
        }
    }
}

// 3. Các struct cụ thể triển khai trait Draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("  [Button] vẽ nút '{}' kích thước {}x{}", self.label, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "  [SelectBox] vẽ dropdown {}x{} với {} lựa chọn: {:?}",
            self.width,
            self.height,
            self.options.len(),
            self.options
        );
    }
}

// Struct bên thứ 3 tự định nghĩa cũng có thể tích hợp vào Screen!
pub struct TextField {
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("  [TextField] vẽ ô nhập dữ liệu: \"{}\"", self.placeholder);
    }
}

fn main() {
    println!("--- 18.2: Trait Objects & Dynamic Dispatch ---");

    // Tạo màn hình chứa hỗn hợp nhiều component khác nhau
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 30,
                options: vec![
                    String::from("Có"),
                    String::from("Không"),
                    String::from("Có thể"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("Xác nhận"),
            }),
            Box::new(TextField {
                placeholder: String::from("Nhập họ tên..."),
            }),
        ],
    };

    screen.run();

    println!("\n* So sánh Dispatch:");
    println!("  - Static Dispatch (Generics <T: Draw>): Nhanh nhất, monomorphization lúc compile,");
    println!("    nhưng bắt buộc tất cả phần tử trong Vec phải CÙNG 1 kiểu.");
    println!("  - Dynamic Dispatch (Box<dyn Draw>): Linh hoạt, cho phép trộn lẫn nhiều kiểu vào Vec,");
    println!("    phí tổn nhỏ khi tra cứu con trỏ hàm qua vtable lúc runtime.");
}
