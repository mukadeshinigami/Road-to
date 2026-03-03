# Блок 15: Cargo и зависимости (Cargo & Dependencies)

## Цель

Научиться **управлять проектом и зависимостями**: структура Cargo.toml, добавление крейтов с crates.io, версионирование (semver), feature flags, команды cargo (build, doc, clippy, fmt), профили сборки и кратко — публикация крейта.

---

## 1. Cargo.toml — структура

Корневой манифест проекта:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["You"]
description = "Short description"

[dependencies]
serde = "1.0"
reqwest = { version = "0.11", features = ["json"] }

[dev-dependencies]
tempfile = "3.0"

[build-dependencies]
# для build.rs
```

- **package** — метаданные пакета.
- **dependencies** — зависимости для сборки и рантайма.
- **dev-dependencies** — только для тестов, примеров, бенчмарков.
- **build-dependencies** — для скрипта сборки (build.rs).

---

## 2. Зависимости с crates.io

По умолчанию версия берётся с crates.io:

```toml
serde = "1.0"           # совместимая версия (semver)
tokio = { version = "1", features = ["full"] }
rand = "0.8"
```

Локальный путь или git:

```toml
my_lib = { path = "../my_lib" }
my_other = { git = "https://github.com/user/repo", branch = "main" }
```

---

## 3. Версионирование: semver

Указание версии в Cargo интерпретируется по semver:

- `"1.0"` — совместимы 1.0.x (не 2.0).
- `"^1.0"` — то же (по умолчанию).
- `"~1.0"` — 1.0.x, но не 1.1.
- `"*"` — любая (не рекомендуется для продакшена).
- `"=1.0.0"` — ровно эта версия.

После первой публикации крейта изменение публичного API должно отражаться в номере версии (major/minor/patch).

---

## 4. Feature flags

Крейт может объявлять опциональные возможности:

```toml
[features]
default = ["std"]
std = []
json = ["serde", "serde_json"]
```

В коде: `#[cfg(feature = "json")]`. Подключение у зависимостей:

```toml
[dependencies]
my_crate = { version = "0.1", features = ["json"] }
```

Отключение default-фич: `default-features = false`.

---

## 5. cargo build и cargo build --release

- **cargo build** — дебаг-сборка (быстрая компиляция, без агрессивной оптимизации).
- **cargo build --release** — релиз: оптимизации, больше времени компиляции, бинарник в `target/release/`.

Профили задаются в `Cargo.toml` в секции `[profile.release]` (opt-level, lto и т.д.).

---

## 6. cargo doc

Генерирует документацию по публичному API из кода и doc-комментариев (`///`). Результат в `target/doc/`. Запуск встроенного сервера: `cargo doc --open`.

---

## 7. cargo clippy

Линтер с дополнительными проверками (стиль, типичные ошибки, производительность). Запуск: `cargo clippy`. Рекомендуется включать в CI.

---

## 8. cargo fmt

Форматирование кода по стандарту. Запуск: `cargo fmt`. Обычно настраивают format on save в редакторе.

---

## 9. Профили сборки

В `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

- **opt-level** — уровень оптимизации (0–3, s/z для размера).
- **lto** — link-time optimization.
- **codegen-units** — меньше единиц компиляции часто даёт быстрее код, дольше сборка.

---

## 10. Публикация крейта (кратко)

- Учётная запись на crates.io, API token.
- `cargo publish` из корня пакета.
- Версия в Cargo.toml должна быть увеличена при каждом релизе; удалять версии нельзя, только yank.
- Документация и описание в Cargo.toml улучшают страницу крейта на crates.io.

---

## 11. Итоги

- Cargo.toml: package, dependencies, dev-dependencies, features.
- Версии: semver, path, git.
- Команды: build, run, test, doc, clippy, fmt; release-профиль.
- Публикация: cargo publish, crates.io.

---

## 12. Что дальше?

Блок 16 — **макросы**: declarative macros, процедурные макросы (введение).

---

## 13. Ресурсы

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Semver in Cargo](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
