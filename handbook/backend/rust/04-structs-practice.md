# Блок 4: Практика — Структуры (Structs) и методы

## Цель практики

Закрепить работу со структурами и методами: создание типов, методы с `&self`/`&mut self`, associated functions (конструкторы).

---

## Задание 1: Rectangle — площадь и периметр

### Описание

Создайте структуру `Rectangle` с полями `width` и `height` и методами для площади и периметра.

### Требования

- Поля: `width: u32`, `height: u32`.
- Метод `area(&self) -> u32`.
- Метод `perimeter(&self) -> u32`.
- Associated function `square(side: u32) -> Self`.

### Вопросы для размышления

1. Почему для `area` и `perimeter` достаточно `&self`, а не владения?
2. Зачем нужна associated function `square` вместо обычного метода?

### Шаги

1. `cargo new structs_rectangle --bin`
2. Определите `struct Rectangle` и блок `impl` с методами.
3. В `main` создайте прямоугольник и квадрат через `Rectangle::square(5)`, выведите площадь и периметр.

### Пример решения

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect: {:?}, area: {}, perimeter: {}", rect, rect.area(), rect.perimeter());

    let sq = Rectangle::square(5);
    println!("square: {:?}, area: {}", sq, sq.area());
}
```

---

## Задание 2: User с валидацией

### Описание

Структура `User` с полями `username`, `email` и методами проверки (валидации).

### Требования

- Поля: `username: String`, `email: String`.
- Метод `is_valid_email(&self) -> bool`: проверка, что в `email` есть `@`.
- Метод `is_valid_username(&self) -> bool`: имя не пустое и не длиннее 20 символов.
- Associated function `new(username: String, email: String) -> Self`.

### Вопросы

1. Как передать в `new` владение строк и при этом оставить API удобным?
2. Нужен ли `&mut self` для методов валидации?

### Пример каркаса

```rust
struct User {
    username: String,
    email: String,
}

impl User {
    fn new(username: String, email: String) -> Self {
        User { username, email }
    }

    fn is_valid_email(&self) -> bool {
        self.email.contains('@')
    }

    fn is_valid_username(&self) -> bool {
        !self.username.is_empty() && self.username.len() <= 20
    }
}
```

Допишите логику и проверьте в `main` на валидных и невалидных данных.

---

## Задание 3: Point и расстояние

### Описание

Структура `Point` с координатами и методом расстояния до другой точки.

### Требования

- Поля: `x: f64`, `y: f64`.
- Метод `distance_to(&self, other: &Point) -> f64` — евклидово расстояние.
- Формула: `sqrt((x2-x1)^2 + (y2-y1)^2)`.

Подсказка: для корня используйте `value.sqrt()` (метод типа `f64`).

### Пример

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p1.distance_to(&p2)); // 5.0
}
```

---

## Задание 4: RPG character (несколько структур)

### Описание

Несколько структур для простой игры: персонаж с именем и здоровьем, оружие с уроном. Метод «ударить» уменьшает здоровье цели.

### Требования

- `struct Weapon { name: String, damage: u32 }`.
- `struct Character { name: String, health: u32 }`.
- У `Character`: метод `take_damage(&mut self, damage: u32)` — уменьшает `health` (не ниже 0).
- Функция или метод: нанести удар оружием по персонажу (передать `&Weapon`, `&mut Character`).

### Вопросы

1. Почему `take_damage` принимает `&mut self`?
2. Где хранить урон — в оружии или передавать числом?

Реализуйте в одном крейте, создайте персонажа и оружие в `main`, нанесите урон и выведите оставшееся здоровье.

---

## Итоги практики

- Создание структур и экземпляров с именованными полями.
- Методы с `&self` (чтение) и `&mut self` (изменение).
- Associated functions как конструкторы (`new`, `square`).
- Несколько структур в одном проекте и передача ссылок между ними.

После выполнения переходите к блоку 5 (Enums и pattern matching).
