# Блок 14: Модули и организация кода (Modules)

## Цель

Организовать **крупный проект**: модули (mod), видимость (pub), файлы как модули, use и pub use, пути (super, self, crate), workspaces.

---

## 1. Модули: mod

Модуль группирует код и управляет видимостью. Объявление: `mod name { ... }` или `mod name;` (тело в отдельном файле).

```rust
mod frontend {
    pub mod http {
        pub fn handle() {}
    }
}

fn main() {
    frontend::http::handle();
}
```

Вложенные модули: `mod a { mod b { } }`. Внутри модуля элементы по умолчанию приватны; с `pub` — видны снаружи.

---

## 2. Видимость: pub

- **pub** — виден снаружи модуля (и родительского дерева, пока не ограничен).
- **pub(crate)** — виден во всём текущем крейте.
- **pub(super)** — виден в родительском модуле.
- **pub(in path)** — виден в указанном модуле.

Без `pub` функция/структура/тип приватны для модуля.

---

## 3. Файлы как модули

`mod name;` означает: искать в файле `name.rs` в той же папке или в папке `name/mod.rs`. Содержимое файла — тело модуля.

Структура:

```
src/
  main.rs      // корень бинарного крейта
  lib.rs       // корень библиотечного крейта
  frontend.rs  // mod frontend;
  frontend/
    mod.rs     // содержимое модуля frontend
    http.rs    // mod http; внутри frontend
```

В `main.rs` или `lib.rs`: `mod frontend;`. В `frontend/mod.rs`: `pub mod http;` и т.д.

---

## 4. mod.rs vs название.rs

- Один файл-модуль: `frontend.rs` → `mod frontend;`.
- Модуль с подмодулями: каталог `frontend/` с `mod.rs` внутри; в родителе — `mod frontend;`. Подмодули: `frontend/http.rs` и в `frontend/mod.rs` — `pub mod http;`.

Современный стиль: можно обойтись без `mod.rs`, используя `frontend.rs` для модуля и `frontend/` для подмодулей (в `frontend.rs` тогда `pub mod something` и файл `frontend/something.rs`).

---

## 5. use для импорта

Сокращение путей:

```rust
use std::collections::HashMap;
use crate::frontend::http::handle;
use crate::frontend::http::handle as handle_req;
```

После `use` можно вызывать `HashMap::new()`, `handle()`. Группировка: `use std::fmt::{Display, Debug};`. Реэкспорт: `pub use crate::frontend::http::handle;` — тогда внешние крейты смогут писать `use my_crate::handle`.

---

## 6. as для алиасов

```rust
use std::io::Result as IoResult;
use my_crate::LongTypeName as Short;
```

---

## 7. pub use (реэкспорт)

Сделать путь к элементу короче или стабильным для пользователей крейта:

```rust
pub use frontend::http::handle;  // снаружи: use my_crate::handle;
```

---

## 8. super, self, crate

- **crate** — корень текущего крейта (lib или main).
- **super** — родительский модуль.
- **self** — текущий модуль.

Пример: в `frontend/http.rs` обратиться к соседу — через `super::other_module` или из корня `crate::frontend::other_module`.

---

## 9. Workspaces

Несколько крейтов в одном репозитории. В корне `Cargo.toml`:

```toml
[workspace]
members = ["crate_a", "crate_b"]
```

Каждый member — отдельная папка с своим `Cargo.toml`. Зависимости между крейтами: в `crate_b/Cargo.toml` указать `crate_a = { path = "../crate_a" }`. Общая целевая директория `target/` в корне workspace.

---

## 10. Типичные ошибки

- Забыть `pub` — «private in public» или элемент не найден снаружи.
- Объявить `mod foo;` без файла `foo.rs` или `foo/mod.rs` — ошибка компиляции.
- Путать корень крейта: в бинарнике корень — файл с `main()`, обычно `main.rs`; в библиотеке — `lib.rs`.

---

## 11. Best practices

- Логическая группировка: по фичам или по слоям (frontend, backend, db).
- Публичный API минимален: pub только то, что нужно снаружи; остальное — без pub.
- Использовать `pub use` для удобного и стабильного API.

---

## 12. Итоги

- mod, pub, вложенные модули; файлы как модули (file.rs или dir/mod.rs).
- use, as, pub use; пути crate, super, self.
- Workspace для нескольких крейтов в одном репозитории.

---

## 13. Что дальше?

Блок 15 — **Cargo и зависимости**: Cargo.toml, crates.io, semver, features, cargo build/doc/clippy/fmt, публикация.

---

## 14. Ресурсы

- [The Book: Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example: Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
