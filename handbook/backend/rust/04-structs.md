# Блок 4: Структуры (Structs) и методы

## Цель

Научиться создавать **собственные типы данных** с помощью структур (`struct`) и методов (`impl`), чтобы группировать связанные данные и поведение.

---

## 1. Зачем нужны структуры

До сих пор мы работали с примитивами и кортежами. Но в реальных программах нужны **именованные поля** и **поведение** (методы). Структура в Rust — это тип, объединяющий несколько полей под одним именем.

### Сравнение с другими языками

- **C:** `struct` — только данные, без методов (функции пишут отдельно).
- **JavaScript/TypeScript:** объекты с полями и методами.
- **Rust:** `struct` — данные + методы в блоках `impl`; нет наследования, есть композиция и трейты.

---

## 2. Определение структуры и создание экземпляров

### Базовый синтаксис

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.username);
}
```

Поля задаются как `имя: тип`. Порядок полей при создании экземпляра может не совпадать с порядком в определении.

### Доступ к полям

Через точку: `user1.username`, `user1.email`. Если экземпляр изменяемый (`let mut user1 = ...`), можно писать `user1.sign_in_count += 1;`.

### Struct update syntax

Можно создать новый экземпляр на основе существующего с помощью `..`:

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    username: String::from("bob"),
    ..user1  // остальные поля копируются из user1
};
```

Владение: поля типа `String` при этом **перемещаются** в `user2`, поэтому `user1` после этого нельзя использовать целиком (только те поля, которые не были перемещены).

> Вопрос: какие поля у `User` имеют тип `Copy`? Что произойдёт с `user1.active` и `user1.sign_in_count` после `..user1`?

---

## 3. Tuple structs и unit-like structs

### Tuple structs

Структура без имён полей — только типы. Удобно, когда нужен отдельный тип с семантикой, но не хочется именовать поля:

```rust
struct Point(i32, i32);
struct Color(u8, u8, u8);

fn main() {
    let p = Point(10, 20);
    println!("{}", p.0);
}
```

Доступ по индексу: `.0`, `.1`.

### Unit-like structs

Структура без полей. Используется когда нужно реализовать трейт для типа без данных (например, маркеры):

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

---

## 4. Методы: блоки `impl`

Методы — это функции, связанные со структурой. Они определяются в блоке `impl ИмяСтруктуры`.

### self, &self, &mut self

- `self` — владение экземпляром (редко нужно).
- `&self` — неизменяемая ссылка (чтение полей).
- `&mut self` — изменяемая ссылка (можно менять поля).

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
    rect.scale(2);
    println!("Area after scale: {}", rect.area());
}
```

Компилятор автоматически добавляет ссылку при вызове: `rect.area()` превращается в `Rectangle::area(&rect)`.

### Associated functions (функции, не методы)

Функции в `impl` без параметра `self` называются *associated functions*. Они вызываются через `::`:

```rust
impl Rectangle {
    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

// Usage:
let sq = Rectangle::square(10);
```

`Self` — алиас для типа структуры (здесь `Rectangle`). Такие функции часто используют как конструкторы.

---

## 5. Множественные блоки impl

Для одной структуры можно писать несколько блоков `impl` — они эквивалентны одному большому. Удобно для организации кода или условной компиляции (`#[cfg(...)]`).

```rust
impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    }
}

impl Rectangle {
    fn height(&self) -> u32 {
        self.height
    }
}
```

---

## 6. Типичные ошибки

- **Забыли `mut`** при вызове метода с `&mut self`: нужен `let mut r = ...`.
- **Передача по владению** в метод, когда достаточно `&self`: лишние move, после которых экземпляр нельзя использовать.
- **Путаница между `Self` и `self`**: `Self` — тип, `self` — экземпляр.

---

## 7. Best practices

- Именование структур в **PascalCase**, полей — в **snake_case**.
- Группировать связанные методы в логические блоки `impl`.
- Конструкторы оформлять как associated functions (`new`, `from_*`).
- Для вывода отладки использовать `#[derive(Debug)]` и `println!("{:?}", x)`.

---

## 8. Итоги

- `struct` задаёт тип с именованными полями (или tuple/unit-like).
- Экземпляры создаются через `Имя { поле: значение, ... }`.
- Методы объявляются в `impl` с `&self` / `&mut self` / `self`.
- Associated functions без `self` — конструкторы и утилиты, вызов через `Struct::function()`.

---

## 9. Что дальше?

В следующем блоке — **перечисления (enums)** и **pattern matching**: варианты с данными, `Option<T>`, `Result<T, E>` и мощный `match`.

---

## 10. Ресурсы

- [The Book: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example: Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
