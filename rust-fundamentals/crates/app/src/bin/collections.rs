// Chương 8: Common Collections — Vec<T>, String, HashMap<K, V>
use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hash_maps();
}

// ───────────────────────── 8.1 Vec<T> ─────────────────────────
fn vectors() {
    println!("===== 8.1 Vec<T> =====");

    // Tạo vector
    let mut v: Vec<i32> = Vec::new(); // rỗng -> phải ghi kiểu
    v.push(5);
    v.push(6);
    let v2 = vec![1, 2, 3]; // macro vec! -> Rust tự suy ra Vec<i32>
    println!("v = {v:?}, v2 = {v2:?}");

    // Đọc phần tử: 2 cách
    let third: &i32 = &v2[2]; // index: sai index -> PANIC
    println!("v2[2] = {third}");

    match v2.get(100) {
        // get: trả về Option<&T>, sai index -> None (an toàn)
        Some(x) => println!("v2.get(100) = {x}"),
        None => println!("v2.get(100) = None (không có phần tử)"),
    }

    // Borrow rule: đang giữ tham chiếu bất biến thì không push được
    let mut v3 = vec![1, 2, 3];
    let first = &v3[0];
    // v3.push(4); // ❌ E0502: push có thể cấp phát lại bộ nhớ -> `first` thành con trỏ treo
    println!("first = {first}");
    v3.push(4); // ✅ OK vì `first` không còn được dùng nữa

    // Duyệt
    for i in &v3 {
        print!("{i} ");
    }
    println!();

    // Duyệt và sửa: dùng &mut và * (dereference)
    for i in &mut v3 {
        *i += 50;
    }
    println!("sau khi +50: {v3:?}");

    // Lưu nhiều kiểu trong 1 Vec: dùng enum
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            Cell::Int(n) => print!("Int({n}) "),
            Cell::Float(f) => print!("Float({f}) "),
            Cell::Text(t) => print!("Text({t}) "),
        }
    }
    println!();
} // <- v, v2, v3, row bị drop ở đây (cùng toàn bộ phần tử)

// ───────────────────────── 8.2 String ─────────────────────────
fn strings() {
    println!("\n===== 8.2 String =====");

    // Tạo String
    let mut s = String::new();
    s.push_str("Xin"); // nối &str
    s.push(' '); // nối 1 char
    s += "chào"; // += cũng nối &str
    let s2 = "tiếng Việt".to_string(); // &str -> String
    println!("s = {s}, s2 = {s2}");

    // Nối bằng + : s1 bị MOVE, s3 chỉ mượn (&)
    let s1 = String::from("Hello, ");
    let s3 = String::from("world!");
    let joined = s1 + &s3; // fn add(self, s: &str) -> String
    // println!("{s1}"); // ❌ s1 đã bị move
    println!("joined = {joined}, s3 vẫn dùng được = {s3}");

    // format! : không lấy ownership, dễ đọc hơn
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let f = format!("{a}-{b}-{c}");
    println!("format! = {f}");

    // KHÔNG index được bằng số: String là UTF-8 bytes
    let hello = String::from("Chào");
    // let h = hello[0]; // ❌ E0277: `String` cannot be indexed by `{integer}`
    println!("\"{hello}\": len() = {} bytes, nhưng chỉ có {} ký tự", hello.len(), hello.chars().count());

    // Duyệt theo ký tự hoặc theo byte
    print!("chars: ");
    for ch in hello.chars() {
        print!("[{ch}] ");
    }
    println!();
    print!("bytes: ");
    for b in hello.bytes() {
        print!("{b} ");
    }
    println!("   ('à' chiếm 2 bytes: 195 160)");

    // Slice theo byte: phải cắt đúng ranh giới ký tự
    // "Chào" = C(1 byte) h(1 byte) à(2 bytes) o(1 byte)
    println!("&hello[0..2] = {}", &hello[0..2]); // "Ch" ✅
    // &hello[0..3] sẽ PANIC: byte 3 nằm giữa ký tự 'à'
    println!("get(0..3) = {:?} (an toàn, trả None thay vì panic)", hello.get(0..3));
}

// ───────────────────────── 8.3 HashMap<K, V> ─────────────────────────
fn hash_maps() {
    println!("\n===== 8.3 HashMap<K, V> =====");

    // Tạo & thêm
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Đọc: get trả về Option<&V>
    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("Blue = {score}");

    // Duyệt (thứ tự KHÔNG cố định)
    for (key, value) in &scores {
        println!("  {key}: {value}");
    }

    // Ownership: String bị move vào map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}"); // ❌ đã move vào map
    println!("map = {map:?}");

    // 3 cách cập nhật
    scores.insert(String::from("Blue"), 25); // 1. Ghi đè
    scores.entry(String::from("Red")).or_insert(30); // 2. Chỉ thêm nếu chưa có
    scores.entry(String::from("Blue")).or_insert(999); //    Blue đã có -> giữ 25
    println!("scores sau cập nhật: Blue={}, Red={}", scores["Blue"], scores["Red"]);

    // 3. Cập nhật dựa trên giá trị cũ: đếm từ
    let text = "hello world wonderful world";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0); // &mut i32
        *count += 1;
    }
    println!("đếm từ: {counts:?}");
}
