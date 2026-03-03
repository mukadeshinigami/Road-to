# Блок 6: Коллекции (Collections)

## Цель

Научиться работать с **динамическими структурами данных** стандартной библиотеки: `Vec<T>`, `String`/`&str`, `HashMap<K, V>` и базовое использование итераторов.

---

## 1. Vec<T> — динамический массив

`Vec` — массив переменной длины в куче. Элементы хранятся последовательно.

### Создание и основные методы

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

let v2 = vec![10, 20, 30];  // макрос для инициализации
```

- `push(element)` — добавить в конец (требует `mut`).
- `pop() -> Option<T>` — удалить и вернуть последний элемент.
- `len()` — количество элементов.
- `get(index) -> Option<&T>` — безопасный доступ по индексу.
- `&v[index]` — прямой доступ; паника при выходе за границы.

### Итерация

```rust
for i in &v {
    println!("{}", i);
}
for i in &mut v {
    *i += 1;
}
```

Или через явный итератор: `v.iter()`, `v.iter_mut()`, `v.into_iter()`.

### Связь с владением

- `Vec` владеет элементами в куче.
- При передаче `Vec` в функцию по значению владение передаётся (move).
- Часто используют `&Vec<T>` или `&[T]` (слайс), чтобы только читать или передавать без владения.

> Вопрос: чем отличается `v.get(0)` от `v[0]`? Когда что использовать?

---

## 2. String и &str

В Rust два основных строковых типа:

- **`String`** — владеющая строка в куче, изменяемая (как `Vec<u8>` для UTF-8).
- **`&str`** — срез строки (view), неизменяемый; может указывать на `String` или на литерал.

### Создание и изменение String

```rust
let mut s = String::new();
let s2 = String::from("hello");
let s3 = "world".to_string();

s.push_str("foo");
s.push('!');
```

- `push_str(&str)` — добавить подстроку.
- `push(char)` — добавить один символ.
- `format!("{} {}", a, b)` — аналог `println!`, но возвращает `String`.

### Методы для работы с содержимым

- `len()` — размер в **байтах** (не в символах; UTF-8).
- `chars()` — итератор по символам (char).
- `split(separator)` — разбиение на подстроки.
- `contains(&str)`, `find(char)` — поиск.

### Индексация

Прямой индекс по байтам (`s[0]`) в Rust **не разрешён** для строк (из-за UTF-8). Используют слайсы по байтам `&s[0..4]` (осторожно с границами символов) или итераторы `chars()`.

---

## 3. HashMap<K, V>

Хранение пар ключ–значение. Нужен `use std::collections::HashMap;`.

### Создание и вставка

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(String::from("Blue"), 10);
map.insert(String::from("Yellow"), 50);
```

Ключи должны реализовывать `Eq` и `Hash` (например, `String`, числа).

### Получение значения

- `get(&key) -> Option<&V>` — по ключу.
- `get_mut(&key) -> Option<&mut V>` — изменяемая ссылка.

```rust
let score = map.get("Blue");
match score {
    Some(s) => println!("{}", s),
    None => println!("not found"),
}
```

### entry и or_insert

Удобный API для «вставить, если нет» или обновить:

```rust
map.entry(String::from("Blue")).or_insert(0);
// или счётчик слов:
*map.entry(word).or_insert(0) += 1;
```

`entry(key)` возвращает `Entry`, у которого методы `or_insert(value)`, `or_insert_with(fn)` и т.д.

### Итерация

```rust
for (k, v) in &map {
    println!("{}: {}", k, v);
}
```

---

## 4. Итераторы — базовое знакомство

Итератор — способ обходить коллекцию по одному элементу. Методы:

- `iter()` — ссылки на элементы (`&T`).
- `iter_mut()` — изменяемые ссылки (`&mut T`).
- `into_iter()` — владение (consuming).

Цепочки методов (lazy):

- `map(|x| ...)` — преобразование.
- `filter(|x| ...)` — отбор.
- `collect()` — собрать в коллекцию (Vec, HashMap и т.д.).

Пример:

```rust
let v = vec![1, 2, 3, 4];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

Подробнее итераторы и замыкания — в блоке 11.

---

## 5. Типичные ошибки

- Забыть `mut` при `push`/`insert`/`push_str`.
- Путать `String` и `&str` в сигнатурах: владение vs заимствование.
- Ожидать индексацию строки по символам — в Rust строки в UTF-8, индексация по байтам и только через слайсы (с осторожностью).

---

## 6. Best practices

- По умолчанию принимать `&str` в функциях (гибче, чем только `String`).
- Для подсчёта/агрегации использовать `HashMap::entry().or_insert()`.
- Выбирать подходящий вид итератора: `iter` / `iter_mut` / `into_iter`.

---

## 7. Итоги

- `Vec<T>` — динамический массив: `push`, `pop`, `get`, итерация.
- `String` — владеющая строка; `&str` — срез; работа через `chars`, `split`, `format!`.
- `HashMap<K, V>` — ключ–значение: `insert`, `get`, `entry`, `or_insert`.
- Итераторы: `iter()`, `map`, `filter`, `collect()`.

---

## 8. Что дальше?

Блок 7 — **обработка ошибок**: `Result` углублённо, оператор `?`, свои типы ошибок, best practices.

---

## 9. Ресурсы

- [The Book: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [The Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [The Book: Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
