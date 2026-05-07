# Блок 6: Тестирование и упаковка проектов

## Цели урока

- Писать тесты на **`pytest`**: функции, фикстуры, параметризация
- Понимать структуру installable package и **`pyproject.toml`**
- Устанавливать пакет в **editable** режиме для разработки

---

## 1. Зачем pytest

`unittest` из стандартной библиотеки достаточен, но **`pytest`** стал де-факто стандартом: меньше boilerplate, богатые assert'ы, фикстуры, плагины.

```bash
pip install pytest
pytest -q
```

Файлы `test_*.py` или классы `Test*` — см. [convention](https://docs.pytest.org/en/stable/explanation/goodpractices.html).

---

## 2. Простой тест

```python
# tests/test_math_extra.py
from __future__ import annotations

import pytest

from mypkg.math_extra import clamp


def test_clamp_inside_range() -> None:
    assert clamp(5, 0, 10) == 5


def test_clamp_below_min() -> None:
    assert clamp(-1, 0, 10) == 0
```

Запуск: `pytest tests/ -q`.

---

## 3. Фикстуры

```python
import pytest
from pathlib import Path


@pytest.fixture
def tmp_json(tmp_path: Path) -> Path:
    p = tmp_path / "a.json"
    p.write_text('{"x": 1}', encoding="utf-8")
    return p
```

`tmp_path` — встроенная фикстура pytest (временный каталог).

---

## 4. Параметризация

```python
@pytest.mark.parametrize(
    ("value", "low", "high", "expected"),
    [(5, 0, 10, 5), (-1, 0, 10, 0), (15, 0, 10, 10)],
)
def test_clamp_param(value: int, low: int, high: int, expected: int) -> None:
    assert clamp(value, low, high) == expected
```

(предполагается, что `clamp` импортирован из твоего модуля)

---

## 5. `pyproject.toml` и packaging

Минимальный пример проекта (PEP 621):

```toml
[build-system]
requires = ["setuptools>=61"]
build-backend = "setuptools.build_meta"

[project]
name = "mypkg"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = []

[tool.setuptools.packages.find]
where = ["src"]

[project.optional-dependencies]
dev = ["pytest>=8", "mypy>=1.8", "ruff>=0.4"]
```

Структура **src layout**:

```text
pyproject.toml
src/
  mypkg/
    __init__.py
    math_extra.py
tests/
  test_math_extra.py
```

Установка для разработки:

```bash
pip install -e ".[dev]"
```

---

## 6. Coverage (опционально)

```bash
pip install pytest-cov
pytest --cov=mypkg --cov-report=term-missing
```

---

## Чеклист после блока

- [ ] Есть хотя бы один параметризованный тест
- [ ] Использована фикстура `tmp_path` или своя на его основе
- [ ] Пакет ставится `pip install -e .` и импортируется из `src`

---

## Дальше

`06-testing-packaging-practice.md`, затем блок 7 — async.

## Ссылки

- [pytest documentation](https://docs.pytest.org/)
- [Python Packaging User Guide](https://packaging.python.org/en/latest/)
- [PEP 621 — pyproject metadata](https://peps.python.org/pep-0621/)
