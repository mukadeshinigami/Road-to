# Блок 21: Базы данных (Databases)

## Цель

Работать с **SQL и NoSQL**: sqlx для SQL (PostgreSQL, MySQL, SQLite), проверка запросов на этапе компиляции, миграции, пулы соединений, кратко ORM (diesel), клиенты Redis и MongoDB.

---

## 1. sqlx для SQL

Крейт **sqlx** — асинхронный драйвер для PostgreSQL, MySQL, SQLite и др. Поддерживает compile-time проверку запросов (при подключённой БД в момент сборки) и runtime-режим. Запросы выполняются через методы `query`, `query_as`, `execute` и т.д.; результаты маппятся на структуры с derive.

```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
```

```rust
let row = sqlx::query_as::<_, (i32, String)>("SELECT id, name FROM users WHERE id = ?")
    .bind(1)
    .fetch_one(&pool)
    .await?;
```

---

## 2. Compile-time проверка запросов

При переменной окружения `DATABASE_URL` и `sqlx prepare` во время сборки sqlx может проверять запросы против реальной БД и кешировать метаданные. Тогда неверный SQL или несовпадение типов обнаружится при компиляции. В CI и у разработчиков часто используют `cargo sqlx prepare` и офлайн-режим.

---

## 3. Миграции

sqlx предоставляет миграции в виде SQL-файлов в каталоге (например, `migrations/`). Команда `sqlx migrate run` применяет их к базе по порядку. В коде можно вызывать `sqlx::migrate!("./migrations").run(&pool).await`. Версионирование схемы хранится в таблице миграций.

---

## 4. Connection pooling

Пул соединений (PgPool, SqlitePool и т.д.) создаётся один раз и передаётся в handlers. sqlx сам управляет числом соединений. В axum состояние приложения часто содержит `Arc<Pool>`. Создание: `SqlitePool::connect(&url).await` или с опциями (min/max connections).

---

## 5. ORM: diesel (кратко)

**Diesel** — синхронный ORM: типизированные запросы, схемы из миграций, модели. Подходит для блокирующего кода. В async-стеке чаще используют sqlx; diesel — когда нужна строгая типизация запросов и синхронный доступ.

---

## 6. Redis клиент

Крейты **redis** или **deadpool-redis** (с tokio) — подключение к Redis, команды GET/SET, пулы. Типичное использование: кеш, сессии, очереди. API через методы типа `get`, `set`, `set_ex` и т.д.

---

## 7. MongoDB клиент

Официальный **mongodb** — асинхронный драйвер. Подключение к кластеру, выбор базы и коллекции, вставка/поиск документов (BSON). Часто в паре с serde для сериализации структур в документы.

---

## 8. Итоги

- sqlx: async SQL, пулы, миграции, опционально compile-time проверка.
- diesel: синхронный ORM при необходимости.
- Redis и MongoDB — отдельные крейты для кеша и документных БД.

---

## 9. Что дальше?

Блок 22 — веб-фреймворки (axum, состояние, БД в API). Блок 25 — финальный проект.

---

## 10. Ресурсы

- [sqlx](https://docs.rs/sqlx/)
- [Redis crate](https://docs.rs/redis/)
- [MongoDB Rust driver](https://docs.rs/mongodb/)
