fn main() {
    let mut s = String::from("hello");

    let mut r1 = &mut s;
    {
        let r2 = &mut r1;
        r2.push_str(" world");
        println!("{r2}"); // In ra: hello world
    } // r2 kết thúc ở đây

    // Bây giờ r1 lại dùng được bình thường:
    println!("{r1}"); // In ra: hello world
}
