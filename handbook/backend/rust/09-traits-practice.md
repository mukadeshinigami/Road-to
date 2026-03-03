# Блок 9: Практика — Трейты (Traits)

## Цель практики

Закрепить определение трейтов, реализацию для типов, trait bounds и при необходимости trait objects.

---

## Задание 1: Trait Drawable для геометрических фигур

### Описание

Трейт `Drawable` с методом `draw(&self) -> String` (возвращает текстовое описание фигуры). Реализовать для структур `Circle` и `Rectangle`.

### Требования

- `trait Drawable { fn draw(&self) -> String; }`
- `Circle { radius: f64 }` — в `draw` вывести радиус и площадь.
- `Rectangle { width: u32, height: u32 }` — вывести размеры и площадь.

### Вопросы

1. Зачем возвращать `String`, а не печатать в консоль?
2. Как вызвать `draw` для значения неизвестного конкретного типа (через `&impl Drawable` или `&dyn Drawable`)?

### Пример

```rust
trait Drawable {
    fn draw(&self) -> String;
}

struct Circle { radius: f64 }
struct Rectangle { width: u32, height: u32 }

impl Drawable for Circle {
    fn draw(&self) -> String {
        let area = std::f64::consts::PI * self.radius * self.radius;
        format!("Circle(r={}, area={:.2})", self.radius, area)
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle({}x{}, area={})", self.width, self.height, self.width * self.height)
    }
}

fn main() {
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4, height: 5 };
    println!("{}", c.draw());
    println!("{}", r.draw());
}
```

Добавьте функцию `print_drawable(d: &impl Drawable)` и вызовите для обеих фигур.

---

## Задание 2: Trait Summary для текстов

### Описание

Трейт `Summary` с методом `summarize(&self) -> String`. Реализовать для типа `NewsArticle` (заголовок, автор, текст) — краткое изложение до 50 символов.

### Требования

- Метод по умолчанию `short_summary(&self) -> String`, возвращающий первые 20 символов `summarize` + "...".
- В `main` создать статью и вызвать оба метода.

### Каркас

```rust
trait Summary {
    fn summarize(&self) -> String;

    fn short_summary(&self) -> String {
        let s = self.summarize();
        if s.len() <= 20 {
            s
        } else {
            format!("{}...", &s[..20])
        }
    }
}

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        if self.content.len() <= 50 {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..50])
        }
    }
}
```

Реализуйте и проверьте.

---

## Задание 3: Реализация собственного Display

### Описание

Структура `Point { x: i32, y: i32 }`. Реализовать `std::fmt::Display`, чтобы `println!("{}", point)` выводило `(x, y)`.

### Требования

- `impl fmt::Display for Point` с методом `fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`.
- Использовать `write!(f, "({}, {})", self.x, self.y)`.

### Пример

```rust
use std::fmt;

struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p);
}
```

---

## Задание 4: Коллекция разных типов через trait objects

### Описание

Вектор `Vec<Box<dyn Drawable>>` (из задания 1): добавить круг и прямоугольник, пройти по вектору и вызвать `draw` у каждого элемента.

### Требования

- Создать `let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(circle), Box::new(rect)];`
- Цикл `for s in &shapes { println!("{}", s.draw()); }`

### Вопросы

1. Зачем здесь `Box<dyn Drawable>`, а не `Box<Circle>` и `Box<Rectangle>` по отдельности?
2. Какой размер у `Box<dyn Drawable>` в памяти?

Реализуйте и убедитесь, что вывод корректен для обеих фигур.

---

## Итоги практики

- Определение трейта и реализация для нескольких типов.
- Методы по умолчанию в трейте.
- Реализация `Display` для своего типа.
- Использование `dyn Trait` в коллекции для полиморфизма в рантайме.

Далее — блок 10 (Тестирование).
