// Chương 10.3: Lifetimes — đảm bảo tham chiếu luôn hợp lệ
use std::fmt::Display;

// ───────────── 1. Lifetime trong hàm ─────────────
// 'a nghĩa là: kết quả sống được bao lâu thì x VÀ y phải sống ít nhất bấy lâu
// -> thực tế 'a = phần giao (ngắn hơn) của lifetime x và y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Kết quả chỉ phụ thuộc x -> y không cần 'a
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// ───────────── 2. Lifetime elision (compiler tự suy) ─────────────
// Viết:          fn first_word(s: &str) -> &str
// Compiler hiểu: fn first_word<'a>(s: &'a str) -> &'a str   (luật 1 + luật 2)
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// ───────────── 3. Struct chứa tham chiếu ─────────────
// ImportantExcerpt không được sống lâu hơn chuỗi mà `part` đang mượn
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Luật 3: có &self -> kết quả mặc định mang lifetime của self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// ───────────── 4. Gộp generics + trait bound + lifetime ─────────────
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("== 1. longest<'a> ==");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    println!("longest = {}", longest(string1.as_str(), string2));

    // Hợp lệ: result được dùng TRONG scope mà cả 2 chuỗi còn sống
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("trong scope trong: {result}");
    }
    // Nếu khai báo `result` ngoài scope rồi dùng ở đây -> E0597 (xem scratch demo)

    println!("first = {}", first("hello", "world"));

    println!("\n== 2. Elision ==");
    println!("first_word(\"xin chào Rust\") = {}", first_word("xin chào Rust"));

    println!("\n== 3. Struct chứa tham chiếu ==");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("excerpt.part = {:?}, level = {}", excerpt.part, excerpt.level());
    println!("returned = {}", excerpt.announce_and_return_part("hi"));

    println!("\n== 4. 'static ==");
    let s: &'static str = "I have a static lifetime."; // nằm trong binary, sống suốt chương trình
    println!("{s}");

    println!("\n== 5. Generics + trait + lifetime ==");
    let r = longest_with_an_announcement("Rust", "Go", 2026);
    println!("r = {r}");
}
