# Блок 5: Практика — Перечисления (Enums) и pattern matching

## Цель практики

Закрепить enums, `Option`, `Result`, `match`, `if let` на практических задачах.

---

## Задание 1: Светофор (TrafficLight)

### Описание

Enum с вариантами цветов светофора и методом длительности (в секундах).

### Требования

- Варианты: `Red`, `Yellow`, `Green`.
- Метод `duration(&self) -> u32`: Red — 30, Yellow — 3, Green — 25.

### Пример

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Duration: {} sec", light.duration());
}
```

---

## Задание 2: Парсер команд CLI (Command enum)

### Описание

Enum вариантов команд: `Exit`, `Help`, `Run { name: String }`, `Status`. Функция принимает строку и возвращает `Option<Command>`.

### Требования

- Парсить строки: `"exit"` → `Exit`, `"help"` → `Help`, `"run <name>"` → `Run { name }`, `"status"` → `Status`.
- Неизвестная строка → `None`.

### Вопросы

1. Почему здесь уместен `Option<Command>`, а не `Result<Command, String>`?
2. Как разбить `"run myapp"` на команду и имя?

### Каркас

```rust
enum Command {
    Exit,
    Help,
    Run { name: String },
    Status,
}

fn parse(input: &str) -> Option<Command> {
    let input = input.trim();
    // TODO: match по input, вернуть Some(...) или None
}
```

Допишите парсер и проверьте в `main` на нескольких строках.

---

## Задание 3: Калькулятор с Result

### Описание

Функция `calculate(op: char, a: f64, b: f64) -> Result<f64, String>`: поддерживать `+`, `-`, `*`, `/`. При делении на 0 или неизвестной операции — `Err("message")`.

### Требования

- Использовать `match` по `op`.
- Деление на ноль → `Err("division by zero")`.
- Неизвестный оператор → `Err("unknown operator")`.

### Пример

```rust
fn calculate(op: char, a: f64, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(String::from("division by zero"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(String::from("unknown operator")),
    }
}
```

В `main` обработайте результат через `match` или `if let Ok(result) = ...`.

---

## Задание 4: Безопасная работа с Option

### Описание

Функция `first_word(s: &str) -> Option<&str>`: вернуть первое слово (до первого пробела). Если слов нет — `None`.

### Требования

- Использовать методы слайсов/строк: `find`, `split_whitespace` или ручной обход.
- Возвращать `Some(&str)` с подстрокой или `None`.

### Вопросы

1. Почему возвращаемый тип — `Option<&str>`, а не `Option<String>`?
2. Как вызывающий код должен обработать `None`?

Пример вызова:

```rust
let s = String::from("hello world");
if let Some(word) = first_word(&s) {
    println!("First word: {}", word);
}
```

Реализуйте `first_word` и несколько тестовых строк.

---

## Итоги практики

- Enum с вариантами с данными и методы через `impl` и `match`.
- Парсинг ввода в `Option<Command>` или `Result<T, E>`.
- Обработка ошибок через `match` и `if let`.
- Безопасный доступ к данным через `Option` вместо null.

Далее — блок 6 (Коллекции).
