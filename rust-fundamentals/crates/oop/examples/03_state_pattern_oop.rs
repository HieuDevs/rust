//! 18.3 (Phần 1) — State Pattern theo phong cách OOP truyền thống.
//! Chạy: cargo run -p oop --example 03_state_pattern_oop

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // Ủy quyền hành vi lấy nội dung cho State hiện tại
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // Dùng take() để lấy giá trị ra khỏi Option (để lại None tạm thời)
        // nhằm lấy ownership của state cũ và chuyển sang state mới
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

// Trait State đại diện cho các trạng thái của bài viết
trait State {
    // self: Box<Self> nghĩa là phương thức này chỉ hợp lệ khi được gọi trên Box giữ giá trị
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// Trạng thái 1: Bản nháp
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Draft chưa được review thì không thể approve, giữ nguyên trạng thái
    }
}

// Trạng thái 2: Đang chờ duyệt
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Đã ở trạng thái chờ duyệt rồi
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Trạng thái 3: Đã xuất bản
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    println!("--- 18.3: State Pattern kiểu OOP truyền thống ---");
    let mut post = Post::new();

    post.add_text("Hôm nay tôi học lập trình Rust!");
    println!("1. Khi ở Draft:          content = \"{}\"", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("2. Khi ở PendingReview:  content = \"{}\"", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("3. Khi đã Published:      content = \"{}\"", post.content());
    assert_eq!("Hôm nay tôi học lập trình Rust!", post.content());

    println!("\n* Nhận xét về cách tiếp cận OOP truyền thống trong Rust:");
    println!("  - Điểm tốt: logic từng trạng thái được đóng gói riêng trong từng struct.");
    println!("  - Điểm hạn chế: Cần `Option` + `take()` để đổi ownership, tốn cấp phát `Box` trên heap,");
    println!("    và kiểm tra logic trạng thái chỉ xảy ra lúc runtime.");
}
