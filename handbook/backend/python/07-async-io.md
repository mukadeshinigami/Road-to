# Блок 7: Async и конкурентность

## Цели урока

- Понимать разницу **concurrency** (много задач чередуются) и **parallelism** (одновременно на нескольких ядрах)
- Писать **`async` / `await`** код на базе `asyncio`
- Знать, когда нужен **thread** или **process** pool вместо async

---

## 1. Зачем asyncio

**Async I/O** хорош там, где много ожидания: сеть, диск (в ограниченной степени), тысячи соединений. Пока одна корутина ждёт I/O, event loop может выполнять другую.

**Не магия:** CPU-bound задачи на одном потоке async **не ускорятся** — нужны процессы или нативные потоки/C-расширения.

---

## 2. Корутины и `asyncio.run`

```python
import asyncio


async def fetch_label(n: int) -> str:
    await asyncio.sleep(0.05)  # simulate I/O
    return f"item-{n}"


async def main() -> None:
    results = await asyncio.gather(fetch_label(1), fetch_label(2))
    print(results)


if __name__ == "__main__":
    asyncio.run(main())
```

`asyncio.run` создаёт loop, запускает корутину `main`, корректно закрывает loop.

---

## 3. Таймауты и отмена

```python
async with asyncio.timeout(5):
    await slow_call()
```

`Task.cancel()` — кооперативная отмена; внутри корутины периодически проверяй `await asyncio.sleep(0)` или используй `asyncio.CancelledError` осознанно.

---

## 4. Асинхронные контекстные менеджеры

```python
class AsyncResource:
    async def __aenter__(self) -> "AsyncResource":
        await asyncio.sleep(0)
        return self

    async def __aexit__(self, exc_type, exc, tb) -> None:
        await asyncio.sleep(0)
```

---

## 5. Блокирующий код в async

Не вызывай в корутине тяжёлый `time.sleep()` или синхронный `requests.get` без ограничений — **блокируешь весь loop**.

Варианты:

- асинхронные клиенты: `httpx.AsyncClient`, `aiohttp`
- обёртка: `await asyncio.to_thread(blocking_func, args...)`

---

## 6. Параллелизм: `concurrent.futures`

```python
from concurrent.futures import ProcessPoolExecutor

# CPU-bound: split work across processes
with ProcessPoolExecutor() as ex:
    results = list(ex.map(heavy_pure_func, data))
```

Для I/O-bound на блокирующем API иногда проще **`ThreadPoolExecutor`**, чем переписывать всё на async.

---

## Чеклист после блока

- [ ] Можешь объяснить, почему `time.sleep(1)` внутри `async def` — плохая идея
- [ ] Напишешь `gather` из нескольких корутин с обработкой исключений (`return_exceptions=True` или своя логика)
- [ ] Понимаешь, когда выбрать threads/processes вместо asyncio

---

## Дальше

`07-async-io-practice.md` — финальные упражнения по треку.

## Ссылки

- [`asyncio` documentation](https://docs.python.org/3/library/asyncio.html)
- [Real Python: asyncio](https://realpython.com/async-io-python/)
- [HTTPX async](https://www.python-httpx.org/async/)
