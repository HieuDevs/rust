// Crate root của lib `core`.
// `mod xxx;` = đưa file xxx.rs vào cây module (bắt buộc, Rust không tự quét file).

mod front_of_house; // private: chỉ dùng được bên trong crate `core`
pub mod back_of_house; // public: crate khác (app) truy cập được

// Re-export: app gọi được `core::hosting::...` dù `front_of_house` là private.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("== core::eat_at_restaurant ==");

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Path ngắn nhờ `use` ở trên
    hosting::seat_at_table();

    // Struct có field private -> phải tạo qua constructor
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // `toast` là pub -> sửa được
    // meal.seasonal_fruit = String::from("blueberries"); // ❌ field private
    println!("I'd like {} toast please", meal.toast);

    // `pub enum` -> mọi variant tự động pub
    let order = back_of_house::Appetizer::Soup;
    println!("Appetizer: {order:?}");
}

// Private ở crate root, nhưng module con vẫn gọi được qua `super::`
fn deliver_order() {
    println!("crate::deliver_order");
}