# Блок 1: Основы Python 3

## Цели урока

- Установить и проверить Python 3
- Запустить программу из файла и в REPL
- Понять отступы, функции и базовые типы
- Создать виртуальное окружение и установить пакет через `pip`

---

## 1. Установка и проверка версии

Рекомендуется **Python 3.11+** (актуальный синтаксис и performance).

```bash
python3 --version
# или на Windows, если установлен launcher:
py -3 --version
```

Если команды нет — установи Python с [python.org](https://www.python.org/downloads/) или через пакетный менеджер дистрибутива / [pyenv](https://github.com/pyenv/pyenv).

---

## 2. REPL и первый скрипт

**REPL** (read-eval-print loop) — интерактивная сессия:

```bash
python3
```

Выйти: `exit()` или `Ctrl+D` (Linux/macOS).

Минимальная программа в файле `hello.py`:

```python
def main() -> None:
    print("Hello, Python!")


if __name__ == "__main__":
    main()
```

Запуск:

```bash
python3 hello.py
```

**Зачем `if __name__ == "__main__"`:** при `import hello` из другого модуля блок под этим условием не выполнится; при прямом запуске файла — выполнится. Это стандартный паттерн entry point.

---

## 3. Отступы и управление потоком

В Python **отступы обязательны** (обычно 4 пробела). Нет фигурных скобок для блоков.

```python
def describe(n: int) -> str:
    if n < 0:
        return "negative"
    if n == 0:
        return "zero"
    return "positive"


for i in range(3):
    print(i)
```

Циклы: `for ... in ...`, `while`. Условия: `if` / `elif` / `else`.

---

## 4. Базовые типы и коллекции

```python
# Primitives
count: int = 42
ratio: float = 3.14
name: str = "Ada"
ok: bool = True

# Collections (mutable vs immutable matters later)
nums: list[int] = [1, 2, 3]
user: dict[str, str | int] = {"name": "Ada", "age": 36}
point: tuple[int, int] = (10, 20)
unique: set[int] = {1, 2, 2}  # {1, 2}
```

**f-strings** (formatted string literals):

```python
x = 10
print(f"value={x}, double={x * 2}")
```

Аннотации в примерах выше — опциональны для интерпретатора (runtime), но полезны для читаемости и инструментов проверки типов.

---

## 5. Виртуальное окружение и pip

Изоляция зависимостей проекта:

```bash
cd your_project
python3 -m venv .venv
source .venv/bin/activate   # Linux/macOS
# .venv\Scripts\activate    # Windows cmd/powershell

python -m pip install --upgrade pip
pip install requests        # example dependency
pip freeze > requirements.txt
```

Деактивировать: `deactivate`.

---

## 6. Аргументы командной строки (минимум)

```python
import sys


def main() -> None:
    print("args:", sys.argv)


if __name__ == "__main__":
    main()
```

Для серьёзного CLI позже изучи модуль [`argparse`](https://docs.python.org/3/library/argparse.html) или библиотеки вроде Click / Typer.

---

## Чеклист после блока

- [ ] `python3` запускается, версия 3.11+ (или осознанно ниже)
- [ ] Скрипт с `main()` и `__name__` guard выполняется
- [ ] Создан `venv`, активирован, пакет установлен через `pip`
- [ ] Написаны простые `if` / `for` и f-string

---

## Дальше

Перейди к **`01-basics-practice.md`** и выполни задания. Следующий блок: **`02-functions-modules.md`**.

## Ссылки

- [Python Tutorial](https://docs.python.org/3/tutorial/index.html)
- [`venv` — документация](https://docs.python.org/3/library/venv.html)
- [`sys.argv`](https://docs.python.org/3/library/sys.html#sys.argv)
