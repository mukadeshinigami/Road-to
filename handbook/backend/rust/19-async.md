# Блок 19: Async/Await и асинхронное программирование

## Цель

Писать **эффективный асинхронный код**: async fn, Future, .await, tokio::spawn, select!, join!, отмена, различие blocking vs async, обходы для async в трейтах.

---

## 1. async fn и Future

`async fn` возвращает значение, реализующее трейт **Future**. Future — отложенное вычисление; оно выполняется до завершения при опросе (poll) runtime’ом. Само по себе объявление async не запускает код; нужно передать future в executor (например, tokio) через .await или spawn.

```rust
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}
```

---

## 2. .await

`.await` приостанавливает текущую async-функцию, пока данное Future не завершится. Управление возвращается runtime’у; поток не блокируется. После завершения Future выполнение продолжается. .await можно использовать только внутри async-контекста.

```rust
let body = fetch_url("https://example.com").await?;
```

---

## 3. tokio::spawn

Запуск новой задачи (task) в runtime: `tokio::spawn(async move { ... })`. Возвращает `JoinHandle`; на нём можно .await, чтобы дождаться результата. Задача может выполняться на другом потоке пула.

```rust
let handle = tokio::spawn(async move {
    fetch_url("https://a.com").await
});
let result = handle.await??;
```

---

## 4. tokio::select!

Выбор первого завершившегося из нескольких future’ов. Полезно для таймаутов и отмены.

```rust
tokio::select! {
    res = async_op() => { ... }
    _ = tokio::time::sleep(Duration::from_secs(5)) => { ... }
}
```

---

## 5. tokio::join!

Параллельное выполнение нескольких future’ов; ждём завершения всех. Результаты — кортеж.

```rust
let (a, b) = tokio::join!(fetch_url(url1), fetch_url(url2));
```

---

## 6. Cancellation

В Rust отмена — это прекращение опроса Future. Если future не опрашивают (например, после select! выбралась другая ветка), он просто перестаёт выполняться. Код внутри async-блока должен быть готов к «отмене» (например, не оставлять неконсистентное состояние).

---

## 7. Blocking vs async

Блокирующий код (файлы, CPU, синхронный I/O) не должен выполняться внутри async-функции без выноса в отдельный поток (например, `tokio::task::spawn_blocking`), иначе он блокирует поток пула и вредит масштабированию.

---

## 8. Async в трейтах (workarounds)

Трейты не могут требовать `async fn` в стабильном Rust напрямую (нет async fn in traits в стабильном виде на момент многих гайдов). Обходы: возвращать `Pin<Box<dyn Future + Send>>`, использовать крейты вроде async-trait (макрос), либо описывать синхронные интерфейсы. С появлением стабилизации async fn in traits подход может измениться.

---

## 9. Итоги

- async fn возвращает Future; .await приостанавливает до завершения Future.
- tokio::spawn — запуск задачи; join! и select! — ожидание и выбор.
- Отмена через прекращение опроса; не блокировать executor.
- Async в трейтах — через обёртки или крейты.

---

## 10. Что дальше?

Блок 20 — **CLI приложения** (clap, subcommands, вывод, конфиг).

---

## 11. Ресурсы

- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
