# Блок 5: Перечисления (Enums) и pattern matching

## Цель

Освоить **перечисления (enums)** и **сопоставление с образцом (pattern matching)**:
варианты с данными, `Option<T>`, `Result<T, E>`, `match`, `if let`, `while let`.

---

## 1. Зачем нужны enums

В Rust enum — это тип, значение которого может быть **одним из нескольких вариантов**. В отличие от C/Java, варианты могут **нести данные**. Это заменяет null и часто — целые иерархии классов.

### Пример: IP-адрес

```rust
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback6 = IpAddr::V6(String::from("::1"));
}
```

Один тип `IpAddr` описывает оба вида адресов; данные привязаны к варианту.

---

## 2. Option<T> — замена null

В Rust нет `null`. Вместо него используется `Option<T>`:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`T` — generic: может быть любой тип. Примеры:

```rust
let some_number = Some(5);
let some_string = Some(String::from("hello"));
let absent: Option<i32> = None;
```

Работа с `Option`: проверять, есть ли значение, через `match` или методы вроде `unwrap`, `expect`, `unwrap_or` (осторожно с `unwrap` в продакшене).

> Вопрос: почему наличие `Option<T>` в стандартной библиотеке лучше, чем разрешить null для любого типа?

---

## 3. Result<T, E> — обработка ошибок

Тип для операций, которые могут завершиться ошибкой:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Пример:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => { /* работа с file */ }
        Err(e) => println!("Failed to open: {:?}", e),
    }
}
```

Обработка через `match` обязательна (или через `?`, `unwrap` и т.д.) — компилятор не даст забыть про ошибку.

---

## 4. match — исчерпывающее сопоставление

`match` — выражение, которое сравнивает значение с набором **паттернов** и возвращает результат соответствующей ветки. Все варианты должны быть обработаны.

### Синтаксис

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Ветки могут быть блоками:

```rust
match coin {
    Coin::Penny => {
        println!("Lucky penny!");
        1
    }
    other => value_in_cents(other),
}
```

### Паттерны с данными

Варианты enum могут нести значения — их можно извлекать в переменные:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
    }
}
```

### Связывание с `@` и охранники (guards)

- `@` — сохранить значение в переменную при совпадении: `n @ 1..=10 => println!("{}", n)`.
- Guard — дополнительное условие: `Some(n) if n > 10 => ...`.

### Wildcard и прочее

- `_` — «всё остальное», без привязки значения.
- `..` — в структурах/кортежах «остальные поля».

---

## 5. if let и while let

Когда нужна только **одна** ветка из `match`, удобнее `if let`:

```rust
let some_value = Some(7);
if let Some(x) = some_value {
    println!("x = {}", x);
}
```

Аналогично `while let` для циклов, пока паттерн совпадает:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## 6. Типичные ошибки

- Забыть обработать вариант в `match` — компилятор выдаст ошибку (исчерпываемость).
- Использовать `unwrap()` без обработки ошибки — паника при `None`/`Err`.
- Путать `Option::None` и `Result::Err`: разные типы, разная семантика.

---

## 7. Best practices

- Предпочитать `match` для полной обработки вариантов; `if let` — когда интересует один случай.
- Для ошибок предпочитать `Result` и оператор `?`, а не панику.
- Использовать `expect("message")` только когда паника по смыслу допустима; в библиотечном коде лучше возвращать `Result`.

---

## 8. Итоги

- Enum — тип с несколькими вариантами; варианты могут нести данные.
- `Option<T>` заменяет null; `Result<T, E>` — стандартный способ ошибок.
- `match` — исчерпывающее сопоставление с паттернами; `if let` / `while let` упрощают один вариант.

---

## 9. Что дальше?

В блоке 6 — **коллекции**: `Vec<T>`, `String`, `HashMap<K, V>` и базовые итераторы.

---

## 10. Ресурсы

- [The Book: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Book: Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust by Example: Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
