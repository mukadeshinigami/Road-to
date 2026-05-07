# Блок 5: Практика — типы и инструменты

---

## Задание 1 — аннотировать модуль

Возьми код из блока 2 (например `merge_config`) и:

- Добавь полные аннотации
- Замени `dict` на `Mapping` там, где объекты только читаются
- Запусти `mypy` и исправь замечания

---

## Задание 2 — `TypedDict` для JSON-конфига

Импорт: `from typing import TypedDict`.

Опиши структуру:

```python
class ServerConfig(TypedDict, total=False):
    host: str
    port: int
    ssl: bool
```

Функция `load_config(path: Path) -> ServerConfig` читает JSON и валидирует обязательное поле `host` (остальные с дефолтами).

---

## Задание 3 — `pyproject.toml`

Создай минимальный `pyproject.toml` для библиотеки:

- `[project]` с `name`, `version`, `requires-python`
- `[tool.ruff]` — line length 100, target py311
- `[tool.mypy]` — `python_version`, `warn_return_any = true`

Проверь, что `ruff check .` и `mypy .` запускаются из корня.

---

## Задание 4 — `Protocol` для storage

```python
class Storage(Protocol):
    def get(self, key: str) -> str | None: ...
    def set(self, key: str, value: str) -> None: ...
```

Реализуй `InMemoryStorage` и `FileStorage` (простой формат не важен — хоть pickle для учебы с оговоркой о безопасности). Функция `greet_user(store: Storage, user_id: str) -> str` должна типизироваться против `Storage`.

---

## Критерии

- Нет неявного `Any` без `# type: ignore` с комментарием причины
- Ruff не ругается на очевидные вещи (unused imports, etc.)
