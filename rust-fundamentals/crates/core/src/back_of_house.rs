// Module `crate::back_of_house`

pub struct Breakfast {
    pub toast: String,      // public field
    seasonal_fruit: String, // private field
}

impl Breakfast {
    // Có field private nên bên ngoài không tự tạo `Breakfast { .. }` được -> cần constructor
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }

    // Getter: cho đọc field private nhưng không cho sửa
    pub fn fruit(&self) -> &str {
        &self.seasonal_fruit
    }
}

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub fn fix_incorrect_order() {
    cook_order(); // cùng module
    super::deliver_order(); // `super` = crate root, gọi được hàm private của module cha
}

fn cook_order() {
    println!("back_of_house::cook_order");
}
