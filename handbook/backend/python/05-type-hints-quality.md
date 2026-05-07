# Блок 5: Type hints и качество кода

## Цели урока

- Аннотировать публичные API: аргументы, возвраты, атрибуты классов
- Использовать современный синтаксис (`list[str]`, `X | Y`) и `typing` там, где нужно
- Запускать **static type checker** (`mypy` / `pyright`) и **линтер** (`ruff`)

---

## 1. Зачем типы в динамическом Python

Аннотации **не проверяются интерпретатором** в runtime (до PEP 649 и подобных нюансов — по умолчанию игнорируются для логики). Их читают:

- IDE и автодополнение
- `mypy`, `pyright`, PyCharm
- Документация для команды

---

## 2. Базовый синтаксис (Python 3.10+)

```python
def first(items: list[str]) -> str | None:
    return items[0] if items else None
```

Раньше: `Optional[str]` = `Union[str, None]`; сейчас предпочтительно `str | None`.

---

## 3. Generics и TypeVars

```python
from collections.abc import Sequence
from typing import TypeVar

T = TypeVar("T")


def head(items: Sequence[T]) -> T | None:
    return items[0] if items else None
```

Для dict/list с произвольными ключами/значениями — `Mapping[K, V]`, `Iterable[T]` из `collections.abc`.

---

## 4. `Protocol`, `Callable`, `TypedDict`

- **`Protocol`** — структурная типизация (см. блок 3)
- **`Callable[[int, int], bool]`** — функция от двух int к bool
- **`TypedDict`** — ключи словаря с именами и типами полей

---

## 5. Проверка: mypy / pyright

```bash
pip install mypy
mypy src/
```

Строгие флаги постепенно: `--strict` может быть слишком жёстко для старого кода. Начни с дефолта, добавь `disallow_untyped_defs` для новых модулей.

---

## 6. Ruff: линт + format

[Ruff](https://docs.astral.sh/ruff/) — быстрый линтер (замена части flake8-плагинов) и форматтер, совместимый по стилю с Black.

```bash
pip install ruff
ruff check .
ruff format .
```

Настрой `pyproject.toml` — секции `[tool.ruff]`, `[tool.ruff.lint]`.

---

## 7. Pre-commit (опционально)

[pre-commit](https://pre-commit.com/) — хуки перед коммитом: `ruff`, `mypy`, trailing whitespace.

---

## Чеклист после блока

- [ ] Проект проходит `ruff check` без критичных предупреждений
- [ ] `mypy` (или pyright) на твоём `src/` без неожиданных `Any`
- [ ] Понимаешь разницу между `Sequence`, `list` и `Iterable`

---

## Дальше

`05-type-hints-quality-practice.md`, затем блок 6 — тесты и пакеты.

## Ссылки

- [typing module](https://docs.python.org/3/library/typing.html)
- [mypy documentation](https://mypy.readthedocs.io/)
- [Ruff](https://docs.astral.sh/ruff/)
