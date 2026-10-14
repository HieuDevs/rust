//! 17.3 — Làm việc với nhiều future: join_all, trait object, yield_now, timeout.
//! Chạy: cargo run -p async_await --example 03_many_futures

use std::pin::Pin;
use std::time::Duration;
use trpl::Either;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

// Tự viết timeout từ select + sleep — future chỉ là giá trị, ghép lại được.
async fn timeout<F: Future>(fut: F, max: Duration) -> Result<F::Output, Duration> {
    match trpl::select(fut, trpl::sleep(max)).await {
        Either::Left(output) => Ok(output),
        Either::Right(()) => Err(max),
    }
}

async fn work(name: &str, duration: u64) -> &str {
    trpl::sleep(ms(duration)).await;
    name
}

fn main() {
    trpl::block_on(async {
        println!("1. join_all: số lượng future tùy ý (cùng kiểu)");
        let futures: Vec<_> = (1..=5)
            .map(|i| async move {
                trpl::sleep(ms(10 * (6 - i))).await; // future sau xong trước
                println!("  future {i} xong");
                i * 10
            })
            .collect();
        let results = trpl::join_all(futures).await;
        println!("  kết quả (giữ đúng thứ tự ban đầu): {results:?}");

        println!("\n2. Khác kiểu → Pin<Box<dyn Future>>");
        // Mỗi async block là một kiểu ẩn danh riêng, nên không bỏ chung vào Vec được.
        // Box<dyn Future> xóa kiểu; Pin vì future có thể tự tham chiếu (xem 05_traits).
        let a = async { println!("  block a") };
        let b = async { println!("  block b — kiểu khác a") };
        let c = async {
            trpl::sleep(ms(5)).await;
            println!("  block c — có await")
        };
        let futs: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(a), Box::pin(b), Box::pin(c)];
        trpl::join_all(futs).await;

        println!("\n3. yield_now: nhường quyền chạy");
        // Async là cooperative: runtime chỉ chuyển task tại .await.
        // Code nặng không có .await sẽ "chiếm sóng", future khác phải đợi.
        let heavy = |name: &'static str, yield_between: bool| async move {
            for i in 1..=3 {
                std::thread::sleep(ms(5)); // giả lập việc CPU nặng (block thật)
                println!("  [{name}] bước {i}");
                if yield_between {
                    trpl::yield_now().await;
                }
            }
        };
        println!("  -- không yield: a chạy hết rồi mới tới b");
        trpl::join(heavy("a", false), heavy("b", false)).await;
        println!("  -- có yield: a và b xen kẽ");
        trpl::join(heavy("a", true), heavy("b", true)).await;

        println!("\n4. Timeout tự viết");
        for (name, d) in [("nhanh", 10), ("chậm", 200)] {
            match timeout(work(name, d), ms(50)).await {
                Ok(n) => println!("  {n}: xong trong hạn"),
                Err(max) => println!("  {name}: hết giờ sau {}ms", max.as_millis()),
            }
        }
    });
}
