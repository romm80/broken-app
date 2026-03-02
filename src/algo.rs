use std::collections::HashSet;

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let uniq:HashSet<u64> = HashSet::from_iter(values.iter().cloned());
    let mut res = uniq.into_iter().collect::<Vec<u64>>();
    res.sort();
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
