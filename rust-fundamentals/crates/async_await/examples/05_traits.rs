//! 17.5 — Bên trong async: trait Future, Pin/Unpin, trait Stream.
//! Chạy: cargo run -p async_await --example 05_traits
//!
//! pub trait Future {
//!     type Output;
//!     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//! }
//! `.await` ≈ vòng lặp: poll → Pending thì nhường runtime, chờ được "đánh thức" → poll lại → Ready thì lấy giá trị.

use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use std::time::Duration;
use trpl::{Stream, StreamExt};

// ---------- Tự viết một Future ----------
struct Countdown {
    remaining: u32,
    polls: u32,
}

impl Future for Countdown {
    type Output = &'static str;

    // Countdown: Unpin (mọi field đều Unpin) nên sửa field qua Pin<&mut Self> thoải mái.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls += 1;
        if self.remaining == 0 {
            println!("  poll #{} → Ready", self.polls);
            Poll::Ready("bùm!")
        } else {
            println!("  poll #{} → Pending (còn {})", self.polls, self.remaining);
            self.remaining -= 1;
            // Báo runtime "hãy poll tôi lại". Thiếu dòng này → không ai đánh thức → treo mãi.
            // Future thật (socket, timer) sẽ gọi wake() khi dữ liệu sẵn sàng.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// ---------- Tự viết một Stream ----------
struct Counter {
    n: u32,
    max: u32,
}

impl Stream for Counter {
    type Item = u32;

    // Giống Future::poll nhưng trả Option: Some = còn giá trị, None = hết stream.
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u32>> {
        if self.n < self.max {
            self.n += 1;
            Poll::Ready(Some(self.n))
        } else {
            Poll::Ready(None)
        }
    }
}

fn assert_unpin<T: Unpin>(_: &T) {}

fn main() {
    println!("1. Future tự viết chạy được trên runtime như mọi future khác");
    let result = trpl::block_on(Countdown { remaining: 3, polls: 0 });
    println!("  kết quả: {result}");

    println!("\n2. Stream tự viết dùng được với StreamExt");
    trpl::block_on(async {
        let mut doubled = Counter { n: 0, max: 4 }.map(|x| x * 2);
        while let Some(v) = doubled.next().await {
            println!("  {v}");
        }
    });

    println!("\n3. Vì sao cần Pin");
    // Async block có thể giữ reference trỏ vào CHÍNH biến bên trong nó qua điểm .await
    // → future là kiểu tự tham chiếu (self-referential).
    // Nếu future bị move sang địa chỉ khác, `r` sẽ trỏ vào vùng nhớ cũ → lỗi.
    // Pin hứa: "giá trị này không bị move nữa" → reference bên trong luôn hợp lệ.
    let fut = async {
        let data = vec![1, 2, 3];
        let r = &data; // trỏ vào `data` nằm trong chính future
        trpl::sleep(Duration::from_millis(1)).await; // tạm dừng, giữ cả data lẫn r
        println!("  r vẫn hợp lệ sau await: {r:?}");
    };
    // assert_unpin(&fut); // ❌ E0277: async block không phải Unpin
    let fut = pin!(fut); // ghim lên stack
    trpl::block_on(fut);

    assert_unpin(&Countdown { remaining: 0, polls: 0 }); // ✅ struct thường là Unpin
    println!("  Countdown: Unpin ✓ — async block: !Unpin (phải pin trước khi poll)");
}
