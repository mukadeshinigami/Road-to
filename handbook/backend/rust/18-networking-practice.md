# Блок 18: Практика — Сети и HTTP (Networking)

## Цель практики

Сделать HTTP-клиент для API, парсить JSON, поднять простой REST API сервер.

---

## Задание 1: HTTP-клиент для получения данных с API

### Описание

Написать программу, которая выполняет GET к публичному API (например, https://jsonplaceholder.typicode.com/posts/1) и выводит тело ответа в консоль. Использовать reqwest (blocking или async с tokio).

### Требования

- Обработка ошибок (сеть, статус не 2xx — по желанию).
- Вывод текста ответа или распарсенного JSON.

### Пример (blocking)

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let body = reqwest::blocking::get(url)?.text()?;
    println!("{}", body);
    Ok(())
}
```

Реализуйте и запустите.

---

## Задание 2: Парсинг JSON ответов

### Описание

Тот же API (или /posts) возвращает JSON. Определить структуру (например, Post с полями id, title, body) с serde, получить ответ и распарсить в структуру (или Vec структур). Вывести заголовок первого поста.

### Требования

- `#[derive(Deserialize)]` для структуры; поля опциональны при необходимости (Option).
- Использовать `response.json::<Post>()` или `reqwest::get(...).await?.json::<Post>().await?`.

### Пример

```rust
#[derive(Deserialize)]
struct Post { id: u32, title: String, body: String }

let post: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1").await?.json().await?;
println!("{}", post.title);
```

Реализуйте и выведите несколько полей.

---

## Задание 3: Простой REST API сервер

### Описание

Сервер на axum с маршрутами: GET `/` — «Hello», GET `/users/:id` — вернуть JSON `{ "id": id, "name": "User N" }`. Запуск на 127.0.0.1:3000.

### Требования

- Роут `/` с handler, возвращающим текст.
- Роут `/users/:id` с извлечением Path (id) и ответом Json. Использовать extractor `Path<u32>` и `Json(serde_json::json!({ "id": id, "name": format!("User {}", id) }))`.

### Пример

```rust
use axum::{Router, routing::get, extract::Path, Json};

async fn user(Path(id): Path<u32>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "id": id, "name": format!("User {}", id) }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/users/:id", get(user));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Проверьте через curl или браузер.

---

## Задание 4: WebSocket клиент (опционально)

### Описание

Кратко описать или использовать крейт (например, tokio-tungstenite) для подключения к публичному WebSocket echo-серверу, отправить сообщение и вывести ответ. Если не делаете — достаточно комментария «WebSocket требует отдельный крейт и async цикл приёма/отправки».

---

## Итоги практики

- GET-запрос и вывод тела ответа.
- Десериализация JSON в структуры с serde.
- Минимальный axum-сервер с маршрутами и JSON-ответами.

Далее — блок 19 (Async/Await).
