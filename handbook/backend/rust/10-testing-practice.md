# Блок 10: Практика — Тестирование (Testing)

## Цель практики

Написать unit- и integration-тесты, использовать `assert!`/`assert_eq!`, тесты с `Result` и с ожидаемой паникой.

---

## Задание 1: Тесты для математических функций

### Описание

В библиотечном крейте (или в модуле с функциями) реализовать функцию `add(a: i32, b: i32) -> i32` и написать несколько тестов: положительные числа, ноль, отрицательные.

### Требования

- Модуль `#[cfg(test)] mod tests` с `use super::*`.
- Минимум 3 теста: например `add_positive`, `add_zero`, `add_negative`.

### Пример

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_zero() {
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

Запустите `cargo test` и убедитесь, что все проходят.

---

## Задание 2: Тесты для структур с методами

### Описание

Структура `Rectangle` с методом `area()` (из блока 4). Написать тесты: площадь квадрата, площадь прямоугольника, периметр.

### Требования

- Тесты в том же файле в `#[cfg(test)] mod tests`.
- Проверить `Rectangle::square(4).area() == 16` и аналогично для прямоугольника и периметра.

### Пример

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn perimeter(&self) -> u32 { 2 * (self.width + self.height) }
    fn square(side: u32) -> Self { Rectangle { width: side, height: side } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_area() {
        assert_eq!(Rectangle::square(4).area(), 16);
    }

    #[test]
    fn rect_perimeter() {
        let r = Rectangle { width: 3, height: 5 };
        assert_eq!(r.perimeter(), 16);
    }
}
```

Добавьте тест для площади прямоугольника.

---

## Задание 3: Тесты с ожидаемыми ошибками

### Описание

Функция `parse_age(s: &str) -> Result<u8, String>` (или с enum ошибок): пустая строка и нечисло — `Err`. Написать тесты: успешный парсинг, пустая строка, не число, число вне диапазона (если есть такая проверка).

### Требования

- Использовать `assert!(parse_age("").is_err())`, `assert_eq!(parse_age("25"), Ok(25))` и т.п.
- Либо тесты, возвращающие `Result<(), String>`, с `?` внутри.

### Пример

```rust
fn parse_age(s: &str) -> Result<u8, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty".into());
    }
    s.parse().map_err(|_| "invalid number".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_age() {
        assert_eq!(parse_age("25"), Ok(25));
    }

    #[test]
    fn empty_is_err() {
        assert!(parse_age("").is_err());
    }

    #[test]
    fn non_number_is_err() {
        assert!(parse_age("abc").is_err());
    }
}
```

Реализуйте или адаптируйте под свою сигнатуру и добавьте тесты.

---

## Задание 4: Integration-тесты для библиотеки

### Описание

Создать библиотечный крейт с одной публичной функцией (например, `greet(name: &str) -> String`). В каталоге `tests/` добавить файл `integration_test.rs`, который импортирует эту функцию и проверяет результат.

### Требования

- В `Cargo.toml`: `[lib]` (или по умолчанию lib есть).
- В `src/lib.rs`: `pub fn greet(name: &str) -> String { format!("Hello, {}!", name) }`.
- В `tests/integration_test.rs`: `use crate_name::greet;` и тест `assert_eq!(greet("World"), "Hello, World!")`.

Имя крейта смотрите в `Cargo.toml` (package.name); в тестах имя крейта превращается в имя модуля (дефисы заменяются на подчёркивания). Запустите `cargo test` и убедитесь, что unit- и integration-тесты выполняются.

---

## Итоги практики

- Unit-тесты в `#[cfg(test)] mod tests` с `assert_eq!` и `assert!`.
- Тесты для методов структур и для функций, возвращающих `Result`.
- Integration-тесты в `tests/*.rs` с использованием только публичного API.
- Запуск и фильтрация тестов через `cargo test`.

Далее — блок 11 (Итераторы и замыкания).
