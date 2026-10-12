// Chương 9: Error Handling — panic! và Result<T, E>
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};

// main trả về Result -> được dùng `?` ngay trong main
fn main() -> Result<(), Box<dyn Error>> {
    // Chạy: cargo run -p app --bin error-handling -- panic
    if std::env::args().nth(1).as_deref() == Some("panic") {
        panic_demo();
    }

    let path = std::env::temp_dir().join("rust_book_hello.txt");
    let _ = fs::remove_file(&path); // dọn file cũ (bỏ qua lỗi nếu không có)

    println!("===== 9.2 match trên Result =====");
    open_or_create(&path);
    open_or_create(&path); // lần 2: file đã có

    println!("\n===== unwrap / expect / unwrap_or_else =====");
    unwrap_family(&path);

    println!("\n===== 9.2 Propagating errors với ? =====");
    match read_username_long(&path) {
        Ok(name) => println!("read_username_long  -> Ok({name:?})"),
        Err(e) => println!("read_username_long  -> Err({e})"),
    }
    match read_username_short(Path::new("/khong/ton/tai.txt")) {
        Ok(name) => println!("read_username_short -> Ok({name:?})"),
        Err(e) => println!("read_username_short -> Err({e})  ← lỗi được trả ngược lên"),
    }

    println!("\n===== ? với Option =====");
    println!("last_char_of_first_line(\"Hello\\nWorld\") = {:?}", last_char_of_first_line("Hello\nWorld"));
    println!("last_char_of_first_line(\"\")             = {:?}", last_char_of_first_line(""));

    println!("\n===== 9.3 Kiểu dữ liệu tự kiểm tra (Guess) =====");
    let g = Guess::new(42);
    println!("Guess::new(42).value() = {}", g.value());
    println!("Guess::try_new(500)    = {:?}", Guess::try_new(500).map(|g| g.value()));

    println!("\n===== ? trong main với Box<dyn Error> =====");
    let n: i32 = "123".parse()?; // ParseIntError -> tự chuyển thành Box<dyn Error>
    println!("\"123\".parse() = {n}");
    let bad: Result<i32, _> = "abc".parse::<i32>();
    println!("\"abc\".parse() = {bad:?}");
    // let m: i32 = "abc".parse()?; // bỏ comment: main trả Err -> exit code 1

    fs::remove_file(&path)?;
    Ok(())
}

// ───────────────────── 9.1 panic! ─────────────────────
fn panic_demo() {
    let v = vec![1, 2, 3];
    let i = 99;
    println!("Truy cập v[{i}] với v = {v:?} ...");
    let _x = v[i]; // panic: index out of bounds
}

// ───────────────────── 9.2 match trên Result ─────────────────────
fn open_or_create(path: &PathBuf) {
    // File::open trả về Result<File, io::Error>
    let _file = match File::open(path) {
        Ok(file) => {
            println!("Mở được file có sẵn");
            file
        }
        // Phân loại lỗi bằng error.kind()
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(mut fc) => {
                    use std::io::Write;
                    fc.write_all(b"ferris").expect("ghi file thất bại");
                    println!("Không có file -> đã tạo mới (ghi \"ferris\")");
                    fc
                }
                Err(e) => panic!("Không tạo được file: {e:?}"),
            },
            other => panic!("Lỗi mở file: {other:?}"),
        },
    };
}

// ───────────────────── unwrap / expect ─────────────────────
fn unwrap_family(path: &PathBuf) {
    // unwrap: Ok -> lấy giá trị, Err -> panic với message mặc định
    let _f1 = File::open(path).unwrap();

    // expect: giống unwrap nhưng tự ghi message -> dễ debug hơn (nên dùng hơn unwrap)
    let _f2 = File::open(path).expect("file phải tồn tại vì vừa tạo ở trên");

    // unwrap_or_else: xử lý lỗi bằng closure, không panic
    let content = fs::read_to_string("/khong/ton/tai.txt").unwrap_or_else(|e| {
        println!("unwrap_or_else bắt được lỗi: {e}");
        String::from("(mặc định)")
    });
    println!("content = {content}");
}

// ───────────────────── 9.2 Propagating errors ─────────────────────
// Cách dài: tự match và return Err
fn read_username_long(path: &Path) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Cách ngắn: `?` = "Ok thì lấy giá trị, Err thì return Err ngay"
fn read_username_short(path: &Path) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
    // Ngắn nhất: fs::read_to_string(path)
}

// `?` cũng dùng được với Option (hàm phải trả về Option)
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// ───────────────────── 9.3 To panic! or not to panic! ─────────────────────
// Đóng gói điều kiện hợp lệ vào kiểu: đã có Guess thì chắc chắn 1..=100
pub struct Guess {
    value: i32, // private -> không ai tạo Guess { value: 500 } được
}

impl Guess {
    // Vi phạm hợp đồng (bug của người gọi) -> panic
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    // Dữ liệu từ bên ngoài (user input) -> trả Result để người gọi tự xử lý
    pub fn try_new(value: i32) -> Result<Guess, String> {
        if (1..=100).contains(&value) {
            Ok(Guess { value })
        } else {
            Err(format!("{value} không nằm trong 1..=100"))
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
