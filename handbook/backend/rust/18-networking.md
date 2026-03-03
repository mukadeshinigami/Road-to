# Блок 18: Сети и HTTP (Networking)

## Цель

Делать **HTTP-запросы** и создавать **простые серверы**: reqwest для клиента, введение в tokio и async, axum/hyper для сервера, работа с JSON (serde_json).

---

## 1. reqwest для HTTP-клиента

Крейт **reqwest** — удобный HTTP-клиент. Блокирующий режим (без async) или асинхронный с runtime (например, tokio).

Блокирующий пример:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
```

```rust
let body = reqwest::blocking::get("https://httpbin.org/get")?
    .text()?;
println!("{}", body);
```

Асинхронный (с tokio):

```toml
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://httpbin.org/get").await?.text().await?;
    println!("{}", body);
    Ok(())
}
```

POST, заголовки, таймауты — через методы `Client` и `RequestBuilder`.

---

## 2. tokio — async runtime (введение)

Rust не включает runtime в стандартную библиотеку. **Tokio** — распространённый асинхронный runtime. Атрибут `#[tokio::main]` превращает `async fn main()` в точку входа с запуском runtime. Для блокирующего кода достаточно `reqwest::blocking` без tokio.

---

## 3. async/await синтаксис

`async fn` возвращает будущее (Future); выполнение приостанавливается на `.await`. Код после `.await` выполняется, когда будущее прогрессирует (без блокировки потока). Подробнее — в блоке 19.

---

## 4. axum (или hyper) для серверов

**axum** — веб-фреймворк поверх hyper и tokio. Роутинг, извлечение данных (extractors), ответы в JSON.

Минимальный сервер:

```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Роуты, извлечение Path/Query/Json, состояние приложения — в блоке 22.

---

## 5. JSON: serde_json

Сериализация/десериализация структур:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User { id: u32, name: String }

// from response
let user: User = reqwest::get(url).await?.json().await?;

// to request body
let res = client.post(url).json(&user).send().await?;
```

---

## 6. REST API клиент и простой сервер

Клиент: использовать reqwest для GET/POST, парсить ответ через `.json::<T>()`. Сервер: axum с несколькими маршрутами, возврат JSON через `Json(serde_json::json!({ ... }))` или сериализованную структуру. Обработка ошибок и статусов — через методы Response (status(), text(), json()).

---

## 7. Итоги

- reqwest — HTTP-клиент (blocking или async с tokio).
- tokio — async runtime; #[tokio::main] для main.
- axum — маршрутизация и обработчики для HTTP-сервера.
- serde_json — обмен данными в JSON.

---

## 8. Что дальше?

Блок 19 — **Async/Await** углублённо; блок 22 — веб-фреймворки (роутинг, middleware, состояние).

---

## 9. Ресурсы

- [reqwest](https://docs.rs/reqwest/)
- [axum](https://docs.rs/axum/)
- [Tokio](https://tokio.rs/)
