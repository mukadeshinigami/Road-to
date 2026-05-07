# Блок 3: Практика — ООП и протоколы

---

## Задание 1 — `BankAccount`

Класс с инвариантом «баланс не отрицательный»:

- `deposit(amount: Decimal)`
- `withdraw(amount: Decimal)` — бросай `ValueError` при недостатке средств
- `balance` — только чтение снаружи (используй `@property` или private-конвенцию `_balance`)

Используй `Decimal` из `decimal` для денег.

---

## Задание 2 — `Vector2D` и операторы

Реализуй `@dataclass` `Vector2D(x, y)` с:

- `__add__`, `__sub__`, скалярное умножение `__mul__(self, k: float)`
- `__repr__` в виде `Vector2D(1.0, 2.0)`
- `__abs__` — длина вектора

---

## Задание 3 — итератор диапазона

Класс `StepRange(start, stop, step)` — итерация как у встроенного `range`, но с произвольным `float` `step` (осторожно с ошибками float; для учебы допустимо).

Реализуй через генератор в `__iter__` **или** класс с `__next__`.

---

## Задание 4 — `@contextmanager` для временного cwd

Функция `temporary_cwd(path: Path)`:

- при входе в `with` — `os.chdir(path)`
- при выходе — восстанови прежний каталог (включая исключения)

Используй `contextlib.contextmanager` и `yield`.

---

## Критерии

- Docstrings на публичные классы/методы
- Тесты вручную в `if __name__ == "__main__"` или мини-проверки `assert`
