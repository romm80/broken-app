/// Вариант 1: (алгоритмическая): оригинальный O(n²) цикл → HashSet, O(n) среднее
/// Вариант 2: (микро): HashSet → sort_unstable + dedup, O(n log n)
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut res = values.to_vec();
    res.sort_unstable();
    res.dedup();
    res
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let t = a;
                a = b;
                b = t + b;
            }
            b
        }
    }
}
