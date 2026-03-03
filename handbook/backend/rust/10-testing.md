# Блок 10: Тестирование (Testing)

## Цель

Научиться писать **тесты** в Rust: unit-тесты рядом с кодом, integration-тесты, атрибуты и команды `cargo test`, а также документационные тесты.

---

## 1. Встроенная поддержка тестов

Rust поставляется с тестовым каркасом без внешних крейтов. Тест — функция с атрибутом `#[test]`. Запуск: `cargo test`.

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

Тест «проходит», если функция завершается без паники. Паника (в том числе из `assert!`) помечает тест как проваленный.

---

## 2. assert!, assert_eq!, assert_ne!

- **assert!(expression)** — паника, если выражение `false`.
- **assert_eq!(left, right)** — паника, если `left != right`; при провале выводит оба значения (требуют `Debug`).
- **assert_ne!(left, right)** — паника, если `left == right`.

Сообщение: третий аргумент — формат и аргументы как в `println!`:

```rust
assert!(v.is_empty(), "Vec was not empty: {:?}", v);
assert_eq!(x, 5, "expected x to be 5, got {}", x);
```

---

## 3. #[should_panic]

Тест должен **упасть** (паника ожидаема):

```rust
#[test]
#[should_panic]
fn panics() {
    panic!("expected panic");
}
```

Можно сузить по тексту сообщения: `#[should_panic(expected = "division by zero")]`.

---

## 4. Result<T, E> в тестах

Вместо паники тест может возвращать `Result<(), E>`:

```rust
#[test]
fn with_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("math is broken"))
    }
}
```

Удобно с оператором `?` внутри теста.

---

## 5. Организация: unit vs integration

### Unit-тесты

Обычно в том же файле, что и код, в модуле `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }
}
```

`#[cfg(test)]` — код компилируется только при `cargo test`. `use super::*` — доступ к родительскому модулю.

### Integration-тесты

Отдельные файлы в каталоге `tests/` в корне крейта. Каждый файл — отдельный крейт, который линкуется с библиотекой. Используют только публичный API. Подходят для проверки сценариев «извне».

```
my_crate/
  src/
    lib.rs
  tests/
    integration_test.rs
```

В `tests/integration_test.rs`:

```rust
use my_crate::some_public_function;

#[test]
fn integration_example() {
    assert_eq!(some_public_function(), 42);
}
```

---

## 6. cargo test

- **cargo test** — запуск всех тестов.
- **cargo test name** — только тесты, в имени которых есть подстрока `name`.
- **cargo test -- --ignored** — запуск только игнорируемых тестов (см. `#[ignore]`).
- **cargo test -- --nocapture** — показывать вывод (println!) тестов.

---

## 7. #[ignore]

Тезт можно временно отключить:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // long running
}
```

Запуск только таких: `cargo test -- --ignored`.

---

## 8. Документационные тесты (doc tests)

Код в комментариях `///` к публичным элементам выполняется как тест:

```rust
/// Adds two numbers.
///
/// # Examples
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

При `cargo test` этот блок компилируется и запускается. Удобно для примеров в документации.

---

## 9. Best practices

- Именовать тесты по смыслу: что проверяем и при каких условиях.
- Один тест — одна логическая проверка; при необходимости несколько `assert` в одном тесте.
- Сложные данные выносить в вспомогательные функции (например, `fn make_fixture()`).
- Integration-тесты — для сценариев использования публичного API.

---

## 10. Итоги

- `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`; при необходимости `#[should_panic]` и `Result` в тестах.
- Unit-тесты в `#[cfg(test)] mod tests` в том же модуле; integration — в `tests/`.
- `cargo test`, фильтрация по имени, `--ignored`, `--nocapture`.
- Doc tests в документации к публичным элементам.

---

## 11. Что дальше?

Блок 11 — **итераторы и замыкания (closures)**: функциональный стиль, цепочки методов, создание своих итераторов.

---

## 12. Ресурсы

- [The Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example: Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
