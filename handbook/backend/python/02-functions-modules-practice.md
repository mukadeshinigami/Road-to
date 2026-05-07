# Блок 2: Практика — функции, модули, scope

---

## Задание 1 — `merge_config`

Напиши функцию:

```python
def merge_config(
    defaults: dict[str, object],
    overrides: dict[str, object],
    *,
    deep: bool = False,
) -> dict[str, object]:
    ...
```

- Если `deep is False`: поверхностное объединение — ключи из `overrides` перекрывают `defaults` (достаточно копии и обновления).
- Если `deep is True`: для вложенных `dict` рекурсивно мержи (остальные типы — как в `overrides`).

Покрой граничные случаи: пустые словари, отсутствие вложенности.

---

## Задание 2 — CLI-обёртка над `argparse`

Скрипт `stats.py` с подкомандами:

- `stats sum 1 2 3` → сумма
- `stats mean 1 2 3` → среднее

Используй `argparse` с `subparsers`. Вынеси логику суммы/среднего в отдельный модуль `stats_ops.py`, импортируй в `stats.py`.

---

## Задание 3 — замыкание и счётчик

Импорт: `from collections.abc import Callable`.

Реализуй фабрику:

```python
def make_counter(step: int = 1) -> Callable[[], int]:
    ...
```

Каждый вызов возвращённой функции увеличивает внутреннее состояние на `step` и возвращает новое значение (старт с `0` до первого вызова или с `0` после первого — зафиксируй в docstring и придерживайся).

---

## Задание 4 — пакет `textkit`

Структура:

```text
textkit/
  __init__.py   # re-export: slugify, word_count
  slugify.py
  counts.py
```

`slugify(s: str) -> str` — нижний регистр, пробелы → `-`, удалить символы не `[a-z0-9-]`.

`word_count(s: str) -> dict[str, int]` — частоты слов по `\w+`.

Установи в editable mode (`pip install -e .` с минимальным `pyproject.toml`) или добавь родительский каталог в `PYTHONPATH` и проверь `python -c "from textkit import slugify"`.

---

## Критерии

- Типы в сигнатурах там, где это уместно
- Нет циклических импортов между `stats.py` и `stats_ops.py`
- Краткий docstring у публичных функций
