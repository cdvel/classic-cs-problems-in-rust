use cached::proc_macro::cached;

#[cached]
pub fn fibonacci(n: i64) -> i64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
