# Блок 6: Практика — тесты и пакет

---

## Задание 1 — `clamp` и pytest

Реализуй `clamp(value: float, low: float, high: float) -> float` в `src/lesson06/mylimits.py`.

Напиши тесты:

- нормальные значения внутри диапазона
- границы `low` и `high`
- `low > high` → `ValueError` (и тест на это через `pytest.raises`)

---

## Задание 2 — фикстура клиента API (mock)

Функция `fetch_status(base_url: str) -> int` делает `urllib.request.urlopen(f"{base_url}/status")` и возвращает HTTP-код.

В тестах **не ходи в сеть**: используй `urllib.request.build_opener` + custom handler или `unittest.mock.patch` / `pytest-httpserver` (на выбор).

Проверь коды 200 и 500.

---

## Задание 3 — полный src-layout проект

Создай каталог `lesson06_pkg/` с:

- `pyproject.toml` (имя произвольное, `requires-python >= 3.11`)
- `src/<имя>/__init__.py` и модуль с одной функцией
- `tests/` с минимум тремя тестами

Убедись: `pip install -e ".[dev]"` и `pytest` из корня этого подпроекта проходят.

---

## Задание 4 — coverage gate (опционально)

Добавь `pytest-cov` и добейся покрытия модуля с `clamp` не ниже 90% для учебного задания.

---

## Критерии

- Тесты детерминированы (без реальной сети)
- README в `lesson06_pkg` с одной строкой «how to run tests»
