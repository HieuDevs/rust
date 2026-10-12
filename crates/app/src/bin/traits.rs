// Chương 10.2: Traits — định nghĩa hành vi chung
use std::fmt::{Debug, Display};

// ───────────── 1. Định nghĩa trait ─────────────
pub trait Summary {
    // Bắt buộc implement (không có thân hàm)
    fn summarize_author(&self) -> String;

    // Có default implementation -> implement hay không tuỳ ý
    // Default được phép gọi method khác trong trait
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// ───────────── 2. Implement trait cho các kiểu ─────────────
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // Override default
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Không viết summarize -> dùng default
}

// ───────────── 3. Trait làm tham số ─────────────
// Cách 1: impl Trait (ngắn gọn)
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Cách 2: trait bound <T: Trait> — giống hệt cách 1, nhưng ép 2 tham số CÙNG kiểu
fn notify_both<T: Summary>(a: &T, b: &T) {
    println!("Both: [{}] & [{}]", a.summarize(), b.summarize());
}

// Nhiều bound: dùng +
fn show(item: &(impl Summary + Display)) {
    println!("Display = {item} | Summary = {}", item.summarize());
}

// Bound dài -> dùng where cho dễ đọc
fn compare<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    format!("t = {t}, u = {u:?}")
}

// ───────────── 4. Trả về impl Trait ─────────────
// Người gọi chỉ biết "là thứ gì đó implement Summary"
// Lưu ý: mọi nhánh phải trả về CÙNG 1 kiểu cụ thể
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// ───────────── 5. Implement có điều kiện ─────────────
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display CHỈ tồn tại khi T vừa Display vừa PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Implement Display cho SocialPost để dùng được hàm `show`
impl Display for SocialPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = if self.reply { "reply" } else if self.repost { "repost" } else { "post" };
        write!(f, "{kind} \"{}\"", self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
    };
    let post = SocialPost {
        username: String::from("ferris"),
        content: String::from("Rust traits are cool"),
        reply: false,
        repost: false,
    };

    println!("== 1-2. Implement & default ==");
    println!("article.summarize() = {}", article.summarize()); // bản override
    println!("post.summarize()    = {}", post.summarize()); // bản default

    println!("\n== 3. Trait làm tham số ==");
    notify(&article); // NewsArticle được
    notify(&post); // SocialPost cũng được
    notify_both(&post, &post); // cùng kiểu
    // notify_both(&article, &post); // ❌ T phải là 1 kiểu duy nhất
    show(&post); // SocialPost có cả Summary + Display
    // show(&article); // ❌ NewsArticle chưa implement Display
    println!("{}", compare(&"hello", &vec![1, 2]));

    println!("\n== 4. Trả về impl Trait ==");
    println!("{}", returns_summarizable().summarize());

    println!("\n== 5. Implement có điều kiện ==");
    Pair::new(3, 7).cmp_display(); // i32: Display + PartialOrd ✅
    Pair::new('a', 'Z').cmp_display(); // char ✅ ('a'=97 > 'Z'=90)
    let _p = Pair::new(vec![1], vec![2]); // Vec không Display -> tạo được...
    // _p.cmp_display(); // ❌ ...nhưng không có method cmp_display

    println!("\n== Blanket impl có sẵn trong std ==");
    // std có: impl<T: Display> ToString for T
    // -> SocialPost implement Display nên TỰ ĐỘNG có .to_string()
    println!("post.to_string() = {}", post.to_string());
}
