# Блок 22: Веб-фреймворки (Web Frameworks)

## Цель

Создавать **веб-приложения** на **axum**: роутинг, middleware, extractors (Path, Query, Json), состояние приложения, обработка ошибок, валидация, шаблоны.

---

## 1. axum — современный async-фреймворк

**axum** строится на hyper и tokio. Маршрутизация, извлечение данных и ответы встроены. Состояние приложения передаётся через типы и не глобальное.

---

## 2. Роутинг

Дерево маршрутов собирается из Router:

```rust
let app = Router::new()
    .route("/", get(home))
    .route("/users/:id", get(get_user))
    .route("/users", post(create_user))
    .nest("/api", api_routes());
```

Параметры пути: `:id`. Вложенность: `nest("/prefix", router)`. Методы: get, post, put, delete и т.д.

---

## 3. Middleware

Middleware — слой между запросом и handler’ом (логирование, аутентификация, CORS). В axum используют tower (и совместимые сервисы). Подключение: `.layer(Logger::new().layer())` или кастомный Layer. Обработка ошибок из middleware и преобразование в ответ — через Service/layer.

---

## 4. Extractors (Path, Query, Json)

Извлечение данных из запроса объявляется в сигнатуре handler’а:

- **Path<T>** — параметры пути (/users/:id).
- **Query<T>** — query-строка (?page=1).
- **Json<T>** — тело запроса как JSON (требует serde).
- **State<S>** — состояние приложения.
- **HeaderMap**, **Method**, **Uri** и др.

```rust
async fn get_user(Path(id): Path<u32>, State(db): State<Db>) -> Json<User> {
    let user = db.get_user(id).await?;
    Json(user)
}
```

Несколько extractors в одном handler; порядок не важен. Ошибки извлечения можно маппить в ответ через IntoResponse.

---

## 5. State management

Состояние создаётся один раз и передаётся в Router:

```rust
let app_state = AppState { pool, config };
let app = Router::new()
    .route("/", get(handler))
    .with_state(app_state);
```

В handler: `State<AppState>`. Для разделяемого изменяемого состояния внутри State хранят Arc<Mutex<...>> или каналы. Клонирование State при добавлении маршрутов — только Arc-подобное, без копирования данных.

---

## 6. Error handling в веб-контексте

Handler может возвращать `Result<Response, E>`, где E реализует IntoResponse (маппинг в статус и тело). Или свой тип ошибки с impl IntoResponse для разных кодов (400, 404, 500). Оператор ? в handler’е требует, чтобы ошибка реализовывала IntoResponse.

---

## 7. Validation: validator

Крейт **validator** — атрибуты для валидации полей (length, range, email). Структура с валидацией передаётся в Json; вызов `validate()` перед использованием. При ошибке — ответ 400 с описанием полей.

---

## 8. Templates: askama, tera

- **askama** — шаблоны, компилируемые в Rust (типобезопасность).
- **tera** — Jinja2-подобные шаблоны в рантайме.

Рендер в handler и возврат Html(response) или встроенный в тип ответа. Состояние для шаблонов (база, пользователь) передаётся при рендере.

---

## 9. Итоги

- axum: Router, route, nest; get/post и др.
- Extractors: Path, Query, Json, State.
- Состояние через with_state; ошибки через IntoResponse.
- Middleware, валидация, шаблоны — по необходимости.

---

## 10. Что дальше?

Блок 25 — финальный проект (REST API, БД, тесты, развёртывание).

---

## 11. Ресурсы

- [axum](https://docs.rs/axum/)
- [tower](https://docs.rs/tower/)
