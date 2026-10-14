//! 18.3 (Phần 2) — Type State Pattern: Mã hóa trạng thái thành Kiểu dữ liệu (Phong cách Rust Idiomatic).
//! Chạy: cargo run -p oop --example 04_state_pattern_rust
//!
//! Thay vì kiểm tra trạng thái lúc RUNTIME, ta biến mỗi trạng thái thành một TYPE riêng biệt.
//! Lợi ích: Lỗi logic (như đọc nội dung bản nháp) biến thành LỖI BIÊN DỊCH (Compile-time error)!

// 1. Trạng thái Draft: Chỉ có thể thêm text hoặc gửi duyệt
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Tiêu thụ self (ownership) của DraftPost và chuyển đổi thành PendingReviewPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl Default for DraftPost {
    fn default() -> Self {
        Self::new()
    }
}

// 2. Trạng thái PendingReview: Không thể sửa text, không thể đọc content, chỉ có thể approve
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // Tiêu thụ self và chuyển đổi thành Post hoàn chỉnh (đã xuất bản)
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// 3. Trạng thái Post đã hoàn chỉnh: Lúc này MỚI CÓ method content()
pub struct Post {
    content: String,
}

impl Post {
    pub fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    println!("--- 18.3: Type State Pattern kiểu Rust Idiomatic ---");

    // Bắt đầu với DraftPost
    let mut post = DraftPost::new();
    post.add_text("Lập trình Rust với Type State Pattern thật tuyệt vời!");

    // post.content(); // ❌ LỖI COMPILE NGAY LẬP TỨC: `DraftPost` không có method `content`!

    // Chuyển sang PendingReviewPost
    let post = post.request_review();

    // post.add_text("thêm chữ"); // ❌ LỖI COMPILE: Không thể thêm chữ khi đang chờ duyệt!
    // post.content();            // ❌ LỖI COMPILE: Chưa duyệt nên vẫn không có method `content`!

    // Duyệt bài để trở thành Post đã xuất bản
    let post = post.approve();

    // Lúc này method content() mới tồn tại và gọi được
    println!("Nội dung bài viết: \"{}\"", post.content());
    assert_eq!("Lập trình Rust với Type State Pattern thật tuyệt vời!", post.content());

    println!("\n* Ưu điểm vượt trội của Type State Pattern trong Rust:");
    println!("  1. Không có Box trên heap, không có vtable (tối ưu hiệu năng tối đa).");
    println!("  2. Không cần Option::take() hay unwrap().");
    println!("  3. Bộ kiểm tra kiểu của Rust ngăn chặn trạng thái không hợp lệ ngay từ lúc compile!");
}
