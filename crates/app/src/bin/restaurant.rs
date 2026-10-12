// Crate bin dùng thư viện `core` (khai báo trong crates/app/Cargo.toml).

use core::back_of_house::{self, Appetizer, Breakfast}; // nested path + `self`
use core::hosting; // có được nhờ `pub use` trong core/src/lib.rs
use core::hosting::add_to_waitlist as join_waitlist; // đổi tên bằng `as`

fn main() {
    core::eat_at_restaurant();

    println!("\n== gọi từ crate app ==");
    hosting::add_to_waitlist();
    join_waitlist();

    let b = Breakfast::summer("Sourdough");
    println!("toast = {}, fruit = {}", b.toast, b.fruit());
    // println!("{}", b.seasonal_fruit); // ❌ field private

    let a = Appetizer::Salad;
    println!("Appetizer: {a:?}");

    back_of_house::fix_incorrect_order();

    // core::front_of_house::hosting::add_to_waitlist(); // ❌ E0603: module `front_of_house` is private
}
