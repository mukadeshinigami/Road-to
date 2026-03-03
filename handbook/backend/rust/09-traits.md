# Блок 9: Трейты (Traits)

## Цель

Понять **интерфейсы Rust** и полиморфизм: определение трейтов, реализация для типов, методы по умолчанию, trait bounds, `impl Trait`, trait objects (`dyn Trait`), derive-макросы и стандартные трейты.

---

## 1. Что такое trait

**Trait** — набор методов, который тип может реализовать. Аналог интерфейсов в других языках, но с возможностью методов по умолчанию и без наследования типов.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Тип «реализует» трейт, если предоставляет все обязательные (и при желании переопределяет методы по умолчанию).

---

## 2. Реализация трейта: impl Trait for Type

```rust
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
```

Один и тот же трейт можно реализовать для разных типов; один тип может реализовывать много трейтов.

---

## 3. Методы по умолчанию

В определении трейта метод может иметь тело — тогда реализация для типа может его не переопределять:

```rust
pub trait Summary {
    fn summarize(&self) -> String;

    fn short(&self) -> String {
        format!("(Read more: {}...)", self.summarize())
    }
}
```

Тип обязан реализовать только `summarize`; `short` уже доступен по умолчанию.

---

## 4. Trait bounds (ограничения типов)

Дженерик функция может требовать, чтобы тип реализовывал трейт:

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

Синтаксис: `T: Summary` или `T: Summary + Display`. В блоке `where`: `where T: Summary`.

---

## 5. impl Trait как параметр и как возвращаемый тип

Вместо дженерика можно писать `impl Trait` — «любой тип, реализующий этот трейт»:

```rust
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

Возвращаемый тип:

```rust
fn returns_summarizable() -> impl Summary {
    Article { title: String::from("Hi"), author: String::from("Me") }
}
```

Ограничение: при возврате компилятор знает один конкретный тип (не несколько разных в разных ветках, если только они не упакованы в enum/Box).

---

## 6. Trait objects: dyn Trait

Когда нужна **коллекция разных типов**, каждый из которых реализует один трейт, используют **trait object** — указатель на значение + таблица методов (vtable):

```rust
fn process(items: &[&dyn Summary]) {
    for item in items {
        println!("{}", item.summarize());
    }
}
```

- `&dyn Summary` — ссылка на любой тип, реализующий `Summary`.
- `Box<dyn Summary>` — то же в куче (владеющий указатель).

Ограничения: трейт должен быть **object-safe** (обычно нельзя возвращать `Self`, использовать дженерики в методах так, чтобы размер типа был неизвестен). Подробнее — в книге.

---

## 7. Derive макросы

Стандартные трейты часто выводятся автоматически:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

Частые: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Default`.

---

## 8. Стандартные трейты

- **Display** — форматирование для пользователя (`"{}"`).
- **Debug** — отладочный вывод (`"{:?}"`).
- **Clone** — явное клонирование; **Copy** — копирование по значению (подтип Clone).
- **PartialEq**, **Eq** — сравнение на равенство.

Реализация своего `Display`:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

---

## 9. Типичные ошибки

- Трейт не в области видимости — методы трейта недоступны (нужен `use ...`).
- Trait object для трейта с методами, возвращающими `Self` или с generic-методами — не object-safe.
- Путать `impl Trait` (статическая подстановка типа) и `dyn Trait` (динамическая диспетчеризация).

---

## 10. Best practices

- Делать трейты с минимально нужным набором методов; методы по умолчанию — для расширения без ломания реализаций.
- В библиотеках предпочитать приём `impl Trait` или дженерики с bounds для гибкости.
- `dyn Trait` — когда действительно нужна гетерогенная коллекция или выбор типа в рантайме.

---

## 11. Итоги

- Trait — контракт методов; реализация — `impl TraitName for Type`.
- Trait bounds ограничивают дженерики; `impl Trait` упрощает сигнатуры.
- `dyn Trait` — полиморфизм в рантайме (trait object).
- Derive и стандартные трейты (`Debug`, `Display`, `Clone`, `Copy`, `PartialEq`) — основа идиоматичного кода.

---

## 12. Что дальше?

Блок 10 — **тестирование**: `#[test]`, `assert!`, unit/integration тесты, `cargo test`.

---

## 13. Ресурсы

- [The Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example: Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
