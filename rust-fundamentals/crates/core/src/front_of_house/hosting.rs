// Module `crate::front_of_house::hosting`

pub fn add_to_waitlist() {
    println!("hosting::add_to_waitlist");
}

pub fn seat_at_table() {
    println!("hosting::seat_at_table");
    // `super` = front_of_house. `serving` private nhưng là anh em -> truy cập được
    super::serving::take_order();
}
