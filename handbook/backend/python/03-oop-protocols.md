# Блок 3: ООП и протоколы (data model)

## Цели урока

- Проектировать классы с понятным состоянием и инвариантами
- Использовать `@dataclass` для DTO и простых моделей
- Понимать **dunder-методы** (`__str__`, `__repr__`, `__eq__`, арифметика)
- Реализовывать **итераторы** и **context managers** через протоколы

---

## 1. Класс: атрибуты и методы

```python
class Point:
    def __init__(self, x: float, y: float) -> None:
        self.x = x
        self.y = y

    def distance_from_origin(self) -> float:
        return (self.x**2 + self.y**2) ** 0.5
```

Первый параметр экземплярных методов — `self` (имя по соглашению).

**classmethod** получает класс как первый аргумент (`cls`), **staticmethod** — без неявного первого аргумента.

---

## 2. Наследование и `super()`

```python
class Shape:
    def area(self) -> float:
        raise NotImplementedError


class Rectangle(Shape):
    def __init__(self, w: float, h: float) -> None:
        self.w = w
        self.h = h

    def area(self) -> float:
        return self.w * self.h
```

`super()` разрешает вызов метода следующего класса в MRO (method resolution order). В множественном наследовании изучи **C3 linearization** или упрощай иерархию.

---

## 3. `@dataclass` (Python 3.7+)

```python
from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class User:
    id: int
    name: str
    active: bool = True
```

- `frozen=True` — неизменяемый объект (как правило, хэшируемый, если все поля хэшируемы)
- `slots=True` — экономия памяти и быстрее доступ к атрибутам (Python 3.10+)

Для сложной валидации используй `__post_init__`.

---

## 4. Dunder-методы и «протокол» объекта

Часто используемые:

| Метод            | Назначение                          |
|------------------|-------------------------------------|
| `__repr__`      | однозначное представление для dev   |
| `__str__`       | человекочитаемое для `print`       |
| `__eq__`        | равенство                           |
| `__hash__`      | для `set`/`dict` ключей (осторожно) |

Операторы: `__add__`, `__len__`, `__getitem__` и т.д. — см. [Data model](https://docs.python.org/3/reference/datamodel.html).

---

## 5. Итератор: `__iter__` / `__next__`

Или проще — **генератор**:

```python
def countdown(n: int):
    while n >= 0:
        yield n
        n -= 1
```

Протокол итератора: объект с `__iter__` возвращающим объект с `__next__`, выбрасывающим `StopIteration` в конце.

---

## 6. Context manager: `with`

```python
class timed_block:
    def __enter__(self) -> "timed_block":
        import time

        self._t0 = time.perf_counter()
        return self

    def __exit__(self, exc_type, exc, tb) -> None:
        import time

        print("elapsed", time.perf_counter() - self._t0)
```

Часто удобнее **`contextlib.contextmanager`** + один `yield` вместо класса.

---

## 7. Абстракции: `Protocol` и ABC

- **`typing.Protocol`** (structural subtyping) — «утиная типизация» с проверкой статически: класс подходит, если есть нужные методы
- **`abc.ABC`** — запрет инстанцирования базы, `@abstractmethod`

---

## Чеклист после блока

- [ ] Отличие `__str__` и `__repr__` можешь объяснить на примере
- [ ] Знаешь, когда `dataclass` лучше ручного `__init__`
- [ ] Можешь написать простой контекстный менеджер для ресурса (файл, таймер, lock)

---

## Дальше

`03-oop-protocols-practice.md`, затем блок 4 — ошибки и файлы.

## Ссылки

- [Data model](https://docs.python.org/3/reference/datamodel.html)
- [`dataclasses`](https://docs.python.org/3/library/dataclasses.html)
- [PEP 544 — Protocol](https://peps.python.org/pep-0544/)
