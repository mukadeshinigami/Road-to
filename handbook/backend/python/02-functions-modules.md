
# Блок 2: Функции, модули и область видимости

## Цели урока

- Писать функции с разными сигнатурами: позиционные и именованные аргументы, значения по умолчанию
- Использовать `*args` и `**kwargs` осознанно
- Понимать правило **LEGB** (local → enclosing → global → builtin) для scope
- Организовывать код в пакеты и модули, импортировать без циклических зависимостей

---

## 1. Функции: `def`, `return`, документация

```python
def add(a: int, b: int) -> int:
    """Return sum of two integers."""
    return a + b
```

Без `return` функция возвращает `None`.

---

## 2. Аргументы: порядок и правила

Почему это важно: сигнатура функции — это контракт API. Правильный порядок параметров делает код читаемым, снижает число ошибок при вызове и облегчает backward compatibility.

Допустимый порядок в сигнатуре (Python 3):

1. positional-only (`/`, PEP 570)
2. positional-or-keyword
3. `*args`
4. keyword-only (после `*` или `*args`)
5. `**kwargs`

Правило легко запомнить так: **сначала "что обязательно по позиции", потом "гибкие параметры", потом "всё именованное", и только в конце "мешок настроек"**.

```python
def connect(
    host: str,
    /,
    port: int = 5432,
    *tags: str,
    timeout: float = 3.0,
    ssl: bool = False,
    **options: object,
) -> None:
    """Demonstrate legal parameter ordering."""
    # host is positional-only
    # port is positional-or-keyword
    # tags collects extra positional arguments
    # timeout and ssl are keyword-only
    # options collects extra keyword arguments
    ...
```

Примеры вызова:

```python
connect("db.local")
connect("db.local", 6432, "read-replica", timeout=5.0, ssl=True)
connect("db.local", ssl=True, retries=3)  # retries goes to options
```

Типичные ошибки:

```python
connect(host="db.local")  # TypeError: host is positional-only
connect("db.local", timeout=5.0, 6432)  # SyntaxError: positional after keyword
```

Распаковка при вызове:

```python
args = ("db.local", 6432, "analytics")
kwargs = {"timeout": 4.5, "ssl": True}
connect(*args, **kwargs)
```

Когда использовать разные типы параметров:

- positional-only — когда имя параметра не должно быть частью публичного API
- keyword-only — когда важна явность (`timeout=...` безопаснее, чем "магическое" третье число)
- `*args`/`**kwargs` — для extensibility, но без злоупотребления: явные параметры обычно лучше для читаемости

Мини-проверка:

1. Почему `timeout` часто делают keyword-only?
2. В каком случае стоит добавить `/` в сигнатуру?
3. Что произойдет, если передать позиционный аргумент после keyword аргумента?

---

## 3. `*args` и `**kwargs`

- `*args` — кортеж позиционных «лишних» аргументов
- `**kwargs` — словарь неожиданных именованных аргументов

```python
def trace(tag: str, *values: object, **extra: object) -> None:
    print(tag, values, extra)


trace("run", 1, 2, sep="---")
# values=(1, 2), extra={'sep': '---'}
```

**Практика:** не злоупотребляй — часто явные параметры читаемее, чем «мешок» kwargs.

---

## 4. Lambda и вложенные функции

`lambda` — анонимная однострочная функция. Удобна в `sorted(..., key=)` и подобном, но для сложной логики лучше именованная `def` внутри функции (**nested function**).

```python
pairs = [("b", 2), ("a", 1)]
pairs.sort(key=lambda p: p[1])
```

---

## 5. LEGB: где видно имя

1. **L**ocal — тело текущей функции
2. **E**nclosing — внешние функции (closures)
3. **G**lobal — модуль
4. **B**uiltin — встроенные имена

Чтение ищет снизу вверх по этой цепочке. **Присваивание** создаёт локальное имя, если не указано `global` / `nonlocal`.

```python
x = 0


def outer() -> None:
    x = 1

    def inner() -> None:
        nonlocal x
        x = 2

    inner()
    assert x == 2


outer()
```

Избегай `global` в больших программах; лучше явно передавать зависимости или использовать классы / DI.

---

## 6. Замыкания (closures)

Внутренняя функция «захватывает» переменные из enclosing scope. Типичный подводный камень — цикл и отложенный вызов: все лямбды могут увидеть **одно** финальное значение счётчика. Решение: default-аргумент `lambda i=i: ...` или `functools.partial`.

---

## 7. Модули и пакеты

- **Модуль** — файл `.py` (или расширение на C и т.д.)
- **Пакет** — каталог с `__init__.py` (в Python 3.3+ может быть **namespace package** без `__init__.py`, но для учебных проектов `__init__.py` остаётся нормой)

Импорты:

```python
import math
from pathlib import Path
from mypkg.utils import helper  # mypkg on PYTHONPATH or installed
```

`from module import *` — избегай в production (засоряет namespace, неочевидно для линтеров).

**Относительный импорт** внутри пакета: `from .sub import x` (только если модуль — часть пакета, не top-level скрипт).

---

## 8. `__name__ == "__main__"` и `-m`

```bash
python -m mypkg.cli
```

Запускает `mypkg/cli.py` как `__main__`, корректно выставляя `sys.path` для пакета.

---

## Чеклист после блока

- [ ] Можешь объяснить разницу между `global` и `nonlocal`
- [ ] Пишешь keyword-only параметры там, где API не должен позволять перепутать порядок
- [ ] Понимаешь, почему циклы + lambda без default-аргументов дают баги
- [ ] Разделяешь код на модули без циклического `import`

---

## Дальше

`02-functions-modules-practice.md`, затем блок 3 — ООП и протоколы.

## Ссылки

- [Defining functions](https://docs.python.org/3/tutorial/controlflow.html#defining-functions)
- [Modules](https://docs.python.org/3/tutorial/modules.html)
- [PEP 570 — positional-only parameters](https://peps.python.org/pep-0570/)
