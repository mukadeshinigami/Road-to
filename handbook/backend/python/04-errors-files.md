# Блок 4: Ошибки и работа с файлами

## Цели урока

- Разделять **ошибки программирования** (bugs) и **ожидаемые сбои** (recoverable errors)
- Писать `try` / `except` / `else` / `finally` без «глушения» исключений
- Использовать **`pathlib.Path`** вместо строковых путей
- Читать и писать текстовые и бинарные файлы без утечек дескрипторов

---

## 1. Исключения: базовый синтаксис

```python
try:
    value = int(raw)
except ValueError as e:
    print("not an int", e)
else:
    # runs if no exception in try
    print("parsed", value)
finally:
    # always runs (cleanup)
    ...
```

Лови **конкретные** типы (`ValueError`, `OSError`), а не «голый» `except:` или `except Exception:` без перевыброса/логирования.

---

## 2. Цепочка и `raise ... from`

```python
try:
    data = parse_json(s)
except json.JSONDecodeError as e:
    raise ConfigError("invalid config") from e
```

`from` сохраняет **cause** — полезно для трассировок.

---

## 3. Свои исключения

```python
class AppError(Exception):
    """Base for application errors."""


class NotFoundError(AppError):
    def __init__(self, resource_id: str) -> None:
        super().__init__(f"not found: {resource_id}")
        self.resource_id = resource_id
```

Не злоупотребляй глубокой иерархией; 2–3 уровня часто достаточно.

---

## 4. `pathlib` — объектный API для путей

```python
from pathlib import Path

root = Path("data") / "users" / "ada.json"
text = root.read_text(encoding="utf-8")
root.write_text(text, encoding="utf-8")
```

- `Path.exists()`, `is_file()`, `mkdir(parents=True)`
- Итерировать каталог: `for p in Path("src").rglob("*.py"): ...`

---

## 5. Файлы и контекст `with`

```python
with Path("log.txt").open("a", encoding="utf-8") as f:
    f.write("line\n")
```

`with` гарантирует закрытие файла. Для бинарных данных режим `"rb"` / `"wb"`.

---

## 6. Временные файлы

Для тестов и скриптов: [`tempfile`](https://docs.python.org/3/library/tempfile.html) — `TemporaryDirectory`, `NamedTemporaryFile`.

---

## Чеклист после блока

- [ ] Знаешь, зачем `else` у `try`, и чем он отличается от кода после блока
- [ ] Не используешь bare `except:`
- [ ] Пути строишь через `Path`, кодировку указываешь явно (`encoding="utf-8"`)

---

## Дальше

`04-errors-files-practice.md`, затем блок 5 — type hints и качество.

## Ссылки

- [Errors and exceptions](https://docs.python.org/3/tutorial/errors.html)
- [`pathlib`](https://docs.python.org/3/library/pathlib.html)
- [Built-in exceptions](https://docs.python.org/3/library/exceptions.html)
