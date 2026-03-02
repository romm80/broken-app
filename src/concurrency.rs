use std::sync::atomic::AtomicU64;
use std::sync::{atomic};
use std::thread;
use std::time::Duration;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Небезопасный инкремент через несколько потоков.
/// Использует global static mut без синхронизации — data race.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, atomic::Ordering::Relaxed);
    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                COUNTER.fetch_add(1, atomic::Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    COUNTER.load(atomic::Ordering::Relaxed)
}

/// Плохая «синхронизация» — просто sleep, возвращает потенциально устаревшее значение.
pub fn read_after_sleep() -> u64 {
    thread::sleep(Duration::from_millis(10));
    COUNTER.load(atomic::Ordering::Relaxed)
}

/// Сброс счётчика (также небезопасен, без синхронизации).
pub fn reset_counter() {
    COUNTER.store(0, atomic::Ordering::Relaxed);
}
