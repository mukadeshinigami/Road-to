# Блок 7: Практика — Обработка ошибок (Error Handling)

## Цель практики

Закрепить использование `Result`, оператора `?`, своих типов ошибок и чтение файлов с обработкой ошибок.

---

## Задание 1: Чтение файла с обработкой ошибок

### Описание

Функция `read_file(path: &str) -> Result<String, std::io::Error>`: прочитать содержимое файла в строку. Ошибки не паниковать, а возвращать.

### Требования

- Использовать `std::fs::read_to_string(path)` (он уже возвращает `Result<String, io::Error>`).
- В `main`: вызвать и обработать через `match` или `if let`; при ошибке вывести сообщение.

### Пример

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() {
    match read_file("hello.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Проверьте на существующем и несуществующем файле.

---

## Задание 2: Парсер с кастомными ошибками

### Описание

Парсер строки в число (например, «возраст»). Свой тип ошибки: «пустая строка», «не число», «число вне диапазона».

### Требования

- Enum ошибок, например: `ParseError::Empty`, `ParseError::InvalidNumber`, `ParseError::OutOfRange`.
- Функция `parse_age(s: &str) -> Result<u8, ParseError>`: пустая строка → Empty; не число → InvalidNumber; не 0..=150 → OutOfRange.

### Вопросы

1. Зачем свой тип ошибки вместо `String`?
2. Как преобразовать ошибку парсинга от `str::parse()` в `ParseError::InvalidNumber`?

Каркас:

```rust
#[derive(Debug)]
enum ParseError {
    Empty,
    InvalidNumber,
    OutOfRange,
}

fn parse_age(s: &str) -> Result<u8, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let n: u8 = s.parse().map_err(|_| ParseError::InvalidNumber)?;
    if n > 150 {
        return Err(ParseError::OutOfRange);
    }
    Ok(n)
}
```

В `main` вызовите для нескольких строк и обработайте ошибки.

---

## Задание 3: Калькулятор с propagation ошибок

### Описание

Функция `eval(expr: &str) -> Result<f64, String>`: парсит простые выражения вида `"1 + 2"`, `"10 / 0"`. При делении на 0 или неверном формате — `Err(...)`. Использовать `?` внутри для проброса.

### Требования

- Разбить по пробелу: первый токен — число, второй — оператор, третий — число.
- При ошибке парсинга числа или неизвестном операторе — `Err`.
- Деление на ноль — `Err("division by zero")`.

### Подсказка

Разбить: `let parts: Vec<&str> = expr.split_whitespace().collect();`. Проверить длину, распарсить `parts[0]` и `parts[2]` в `f64`, обработать оператор. Внутри можно вызывать вспомогательные функции, возвращающие `Result`, и использовать `?`.

Реализуйте и протестируйте несколько выражений.

---

## Задание 4: Валидатор с Result

### Описание

Функция `validate_user(username: &str, email: &str) -> Result<(), String>`: проверки (имя не пустое, длина; в email есть `@`). При первой же ошибке вернуть `Err("описание")`, иначе `Ok(())`.

### Требования

- Имя: не пустое, не длиннее 30 символов.
- Email: не пустой, содержит `@`.

### Пример

```rust
fn validate_user(username: &str, email: &str) -> Result<(), String> {
    if username.trim().is_empty() {
        return Err("username is empty".to_string());
    }
    if username.len() > 30 {
        return Err("username too long".to_string());
    }
    if email.is_empty() || !email.contains('@') {
        return Err("invalid email".to_string());
    }
    Ok(())
}
```

Добавьте вызовы в `main` для валидных и невалидных пар (username, email).

---

## Итоги практики

- Чтение файлов через `fs::read_to_string` и обработка `Result`.
- Собственный тип ошибок (enum) и использование в `Result`.
- Проброс ошибок через `?` и явный `match`.
- Валидация ввода с возвратом `Result<(), E>`.

Далее — блок 8 (Дженерики).
