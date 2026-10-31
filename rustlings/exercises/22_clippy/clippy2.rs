fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(value) = option {
        res += value;
    }

    println!("{res}");
}
