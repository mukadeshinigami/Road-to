# Блок 8: Практика — Дженерики (Generics)

## Цель практики

Закрепить дженерик функции, структуры, методы и trait bounds на простых задачах.

---

## Задание 1: Функция поиска максимума (generic)

### Описание

Функция `largest<T>(list: &[T]) -> Option<&T>`, возвращающая ссылку на максимальный элемент среза. Для пустого среза — `None`.

### Требования

- Дженерик по `T`.
- Trait bound: `T: PartialOrd` (для сравнения).
- Пустой срез → `None`; иначе перебор и сравнение.

### Вопросы

1. Почему возвращаем `Option<&T>`, а не `Option<T>`?
2. Зачем нужен именно `PartialOrd`, а не только `Eq`?

### Пример

```rust
fn largest<T>(list: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    list.first().map(|mut max| {
        for item in list.iter().skip(1) {
            if item > max {
                max = item;
            }
        }
        max
    })
}
```

Или через цикл с проверкой на пустоту. Проверьте на `vec![1, 5, 2]` и на пустом срезе.

---

## Задание 2: Структура Point<T>

### Описание

Структура `Point<T>` с полями `x: T`, `y: T`. Метод `x(&self) -> &T`. Для `Point<f64>` дополнительно метод `distance_from_origin(&self) -> f64`.

### Требования

- Дженерик структура и общий `impl<T> Point<T>` с методом `x`.
- Специализированный `impl Point<f64>` с `distance_from_origin` (евклидова норма).

### Пример

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("{}", p.distance_from_origin()); // 5.0
}
```

Добавьте метод `new(x: T, y: T) -> Self` в общий impl.

---

## Задание 3: Обёртка Wrapper<T> с методами

### Описание

Структура `Wrapper<T>`, хранящая одно значение. Методы: `new(value: T) -> Self`, `value(&self) -> &T`. Реализовать только для типов, реализующих `Display`, метод `print(&self)` — выводит внутреннее значение.

### Требования

- Поле `value: T`.
- `new` и `value` в `impl<T> Wrapper<T>`.
- `impl<T: std::fmt::Display> Wrapper<T>` с методом `print`.

### Вопросы

1. Зачем ограничивать `print` только типами с `Display`?
2. Где хранится значение — на стеке или в куче?

Реализуйте и вызовите `Wrapper::new(42).print()` и аналогично для строки.

---

## Задание 4: Generic коллекция — Stack

### Описание

Стек (LIFO) на основе `Vec<T>`: `push`, `pop`, `peek`, `is_empty`, `len`.

### Требования

- Структура `Stack<T>` с полем `data: Vec<T>` (или обёртка над Vec).
- Методы: `new()`, `push(T)`, `pop() -> Option<T>`, `peek() -> Option<&T>`, `is_empty() -> bool`, `len() -> usize`.

### Пример каркаса

```rust
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
```

В `main` добавьте несколько элементов, вызовите `peek` и `pop` и выведите результат.

---

## Итоги практики

- Дженерик функции с trait bounds (`PartialOrd`, `Display`).
- Дженерик структуры и общие vs специализированные impl.
- Обёртки и простые generic-коллекции на базе `Vec`.

Далее — блок 9 (Трейты).
