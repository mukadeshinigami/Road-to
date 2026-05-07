# Блок 7: Практика — async

---

## Задание 1 — параллельные «запросы»

Напиши `async def fake_fetch(url: str) -> str`:

- `await asyncio.sleep(0.1)`
- верни строку `f"OK:{url}"`

Функция `async def fetch_all(urls: list[str]) -> list[str]` должна обработать 20 URL **быстрее**, чем 20 × 0.1 сек последовательно (используй `asyncio.gather`).

Замерь время через `time.perf_counter()`.

---

## Задание 2 — семафор (ограничение конкуренции)

То же 20 URL, но одновременно не более **3** активных `fake_fetch` — используй `asyncio.Semaphore`.

---

## Задание 3 — обёртка блокирующей функции

Дана синхронная функция:

```python
def blocking_sum(n: int) -> int:
    total = 0
    for i in range(n):
        total += i * i
    return total
```

Вызови её из `async def main()` для большого `n` через `asyncio.to_thread`, не блокируя event loop. Сравни поведение с прямым вызовом (наблюдай за другими корутинами в том же loop).

---

## Задание 4 — мини TCP echo (опционально)

Используй [`asyncio.start_server`](https://docs.python.org/3/library/asyncio-stream.html#asyncio.start_server): сервер читает строку и отправляет её обратно с префиксом `ECHO: `.

Напиши клиент на `asyncio.open_connection` и один интеграционный тест (можно ручной скрипт с `asyncio.run`).

---

## Критерии

- Нет блокирующих sleep/HTTP в корутинах без `to_thread` или async-аналога
- Код запускается на Python 3.11+

---

После этого блока трек «от основ до async» в handbook считается пройденным по материалам; дальше — углубление в web (FastAPI/Django), data science или свои проекты.
