# Блок 14: Практика — Модули и организация кода (Modules)

## Цель практики

Разделить проект на модули, вынести библиотеку с публичным API и при желании собрать workspace из нескольких крейтов.

---

## Задание 1: Разделение проекта на модули

### Описание

Создать проект с `main.rs`. Вынести «логику» в модули: например, `math` (функции add, mul) и `greet` (функция greet(name)). В `main` импортировать через `use` и вызвать функции.

### Требования

- В корне (main.rs или отдельный файл, подключённый как mod) объявить `mod math;` и `mod greet;`.
- Файлы `math.rs` и `greet.rs` в `src/` с публичными функциями.
- В main: `use crate::math; use crate::greet;` (или точечный импорт) и вызов.

### Пример структуры

```
src/
  main.rs   // mod math; mod greet; fn main() { ... }
  math.rs   // pub fn add(a: i32, b: i32) -> i32 { a + b }
  greet.rs  // pub fn greet(name: &str) -> String { format!("Hello, {}!", name) }
```

Реализуйте и запустите `cargo run`.

---

## Задание 2: Библиотека с публичным API

### Описание

В том же проекте добавить `lib.rs` (или перейти на библиотечный крейт с бинарником). В lib объявить модули и сделать часть функций публичными через `pub` и при необходимости `pub use`. В `main.rs` использовать библиотеку как `use crate::...` (в одном крейте — тот же crate).

### Требования

- В `lib.rs`: `pub mod math; pub mod greet;` и при желании `pub use math::add; pub use greet::greet;`.
- В `main.rs`: если есть lib, то `use project_name::add;` (имя пакета из Cargo.toml). В одном крейте с lib и bin оба видят один crate, в main можно `use crate::math::add;`.

Если проект только bin, то модули в src и main обращается к ним через crate. Если добавить lib, то публичный API задаётся в lib.rs. Реализуйте один из вариантов и убедитесь, что вызов из main работает.

---

## Задание 3: Вложенные модули и use

### Описание

Структура: `app` → `api` → `handlers`. В `app/mod.rs` объявить `pub mod api;`. В `api/mod.rs` объявить `pub mod handlers;`. В `handlers.rs` — функция `pub fn index() -> String`. Из main (или из app) вызвать `app::api::handlers::index()` и дополнительно сделать в app реэкспорт: `pub use api::handlers::index;` и вызвать `app::index()`.

### Требования

- Файлы: `src/app/mod.rs`, `src/app/api/mod.rs`, `src/app/api/handlers.rs`.
- В корне (main или lib): `mod app;` и вызов через полный путь и через реэкспорт.

Реализуйте и проверьте оба вызова.

---

## Задание 4: Workspace с несколькими крейтами (опционально)

### Описание

В корне репозитория создать workspace: два крейта `lib_math` и `bin_app`. `bin_app` зависит от `lib_math`. В `lib_math` — функция `add(a: i32, b: i32) -> i32`. В `bin_app` в main вызвать `lib_math::add(1, 2)` и вывести результат.

### Требования

- Корневой `Cargo.toml`: `[workspace] members = ["lib_math", "bin_app"]`.
- В `bin_app/Cargo.toml`: `lib_math = { path = "../lib_math" }`.
- В `bin_app/src/main.rs`: `use lib_math::add;` (если add публична в lib_math).

Создайте структуру папок и Cargo.toml, соберите `cargo build -p bin_app` и запустите бинарник.

---

## Итоги практики

- Разделение кода по файлам и модулям (mod, pub).
- Публичный API библиотеки и реэкспорт (pub use).
- Вложенные модули и пути (crate, super).
- Workspace из нескольких крейтов и зависимость path.

Далее — блок 15 (Cargo и зависимости).
