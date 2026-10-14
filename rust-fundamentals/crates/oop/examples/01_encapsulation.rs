//! 18.1 — Đóng gói (Encapsulation) và bảo toàn tính đúng đắn (Invariants).
//! Chạy: cargo run -p oop --example 01_encapsulation

/// Struct đóng gói danh sách số và giá trị trung bình tự động cập nhật.
/// Các trường `list` và `average` là PRIVATE (mặc định), người dùng không thể
/// sửa trực tiếp làm sai lệch giá trị trung bình.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // Hàm helper nội bộ (private): tính lại trung bình mỗi khi thêm/bớt
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = if self.list.is_empty() {
            0.0
        } else {
            total as f64 / self.list.len() as f64
        };
    }
}

impl Default for AveragedCollection {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("--- 18.1: Đóng gói với AveragedCollection ---");
    let mut ac = AveragedCollection::new();

    ac.add(10);
    ac.add(20);
    ac.add(30);
    println!("Sau khi thêm 10, 20, 30 -> Trung bình: {:.2}", ac.average());

    let removed = ac.remove();
    println!("Lấy ra phần tử: {:?}", removed);
    println!("Sau khi remove -> Trung bình mới: {:.2}", ac.average());

    // ac.list.push(100); // ❌ Lỗi E0616: field `list` of `AveragedCollection` is private
    // ac.average = 999.0; // ❌ Lỗi E0616: field `average` of `AveragedCollection` is private
    println!("✓ Tính bao gói bảo vệ dữ liệu: không thể sửa field private từ bên ngoài!");
}
