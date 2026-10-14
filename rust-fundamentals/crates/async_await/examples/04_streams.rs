//! 17.4 — Stream: "iterator bất đồng bộ", trả nhiều giá trị theo thời gian.
//!   Iterator::next()      -> Option<T>
//!   StreamExt::next()     -> impl Future<Output = Option<T>>   (phải .await)
//! Chạy: cargo run -p async_await --example 04_streams

use std::pin::pin;
use std::time::Duration;
use trpl::{IntervalStream, ReceiverStream, Stream, StreamExt};

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

// Trả về một stream: giá trị đến dần qua channel.
fn messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        for (i, word) in ["a", "b", "c", "d"].into_iter().enumerate() {
            tx.send(format!("tin {i}: {word}")).unwrap();
            trpl::sleep(ms(40)).await;
        }
    });
    ReceiverStream::new(rx)
}

fn main() {
    trpl::block_on(async {
        println!("1. Stream từ iterator + adapter map/filter (giống Iterator)");
        let mut odd_squares = trpl::stream_from_iter(1..=6).map(|n| n * n).filter(|n| n % 2 == 1);
        while let Some(v) = odd_squares.next().await {
            println!("  {v}");
        }

        println!("\n2. Stream từ channel");
        // next() yêu cầu Unpin; `impl Stream` không hứa Unpin → ghim lại bằng pin!
        let mut msgs = pin!(messages());
        while let Some(m) = msgs.next().await {
            println!("  {m}");
        }

        println!("\n3. merge 2 stream: tin nhắn + tick định kỳ");
        let mut count = 0;
        let ticks = IntervalStream::new(trpl::interval(ms(25))).map(move |_| {
            count += 1;
            format!("    tick {count}")
        });
        let mut merged = pin!(messages().merge(ticks).take(10));
        while let Some(item) = merged.next().await {
            println!("  {item}");
        }
    });
}
