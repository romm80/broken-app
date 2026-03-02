# broken-app — отладка и оптимизация

## Запуск

```bash
cargo test
cargo bench --bench criterion
cargo bench --bench baseline
cargo +nightly miri test

cargo build --release
samply record ./target/release/demo

RUSTFLAGS="-Zsanitizer=address" \                                                 
    cargo +nightly test \
    --target aarch64-apple-darwin \
    -- --test-threads=1
    
RUSTFLAGS="-Zsanitizer=thread" \                                                  
    cargo +nightly test \
    --target aarch64-apple-darwin \
    -Z build-std \
    -- --test-threads=1
```

## Найденные и исправленные баги

| # | Функция | Тип | Исправление                                                                                |
|---|---------|-----|--------------------------------------------------------------------------------------------|
| 1 | `sum_even` | Off-by-one UB | переделано на итератор вместо get_unchecked                                                |
| 2 | `leak_buffer` | Утечка памяти | переделано на итератор вместо сдига от сырого указателя (есть коммит исправлений с unsafe) |
| 3 | `average_positive` | Логическая ошибка | исправлен алгоритм, fold аккумулирует сумму и количество                                   |
| 4 | `use_after_free` | Use-after-free UB | убраны into_raw/drop/*raw и заменены на обычное разыменование *b                           |
| 5 | `race_increment` | Data race | для счетчика использован AtomicU64                                                         |
| 6 | `normalize` | Неполная фильтрация | chars().filter(!is_whitespace()).flat_map(to_lowercase()) вместо replace(' ', "")          |

## Оптимизации

### `slow_fib` — алгоритмическая
Всесто рекурсии итеративный подход

### `slow_dedup` — алгоритмическая
HashSet для дедубликации, с одной сортировкой в конце

## Бенчмарки

| Функция | До | После | Ускорение |
|---|---|---|---|
| `slow_fib(32)` | 6.76 ms | 18 ns | ~375x |
| `slow_dedup` | 9.26 ms | 159 µs | ~58x |

## Артефакты

В папке `artifacts/` — логи прогонов cargo test, miri, ASan, TSan, flamegraph и результаты criterion до/после.

## Reference

Эталонная реализация: [reference-app](https://github.com/romm80/reference-app)        