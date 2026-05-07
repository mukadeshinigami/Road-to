# Python — Роадмап для изучения

## Введение

Python — высокоуровневый язык с динамической типизацией, сильным стандартной библиотекой и экосистемой для web, data science, автоматизации и скриптов. Официальная документация: [Python Documentation](https://docs.python.org/3/).

**Цели обучения:**

- Уверенно писать читаемый и идиоматичный Python 3
- Понимать модель выполнения, scope, imports и виртуальные окружения (virtual environments)
- Использовать стандартную библиотеку и базовые сторонние пакеты
- Освоить ООП, ошибки и файлы, типизацию, тесты, упаковку и async

---

## Текущий прогресс (что пройдено)

_Отмечай чекбоксы по мере прохождения._

- [ ] Блок 1: Основы (`01-basics.md`, `01-basics-practice.md`)
- [ ] Блок 2: Функции и модули (`02-functions-modules.md`, `02-functions-modules-practice.md`)
- [ ] Блок 3: ООП и протоколы (`03-oop-protocols.md`, `03-oop-protocols-practice.md`)
- [ ] Блок 4: Ошибки и файлы (`04-errors-files.md`, `04-errors-files-practice.md`)
- [ ] Блок 5: Типы и качество (`05-type-hints-quality.md`, `05-type-hints-quality-practice.md`)
- [ ] Блок 6: Тесты и пакеты (`06-testing-packaging.md`, `06-testing-packaging-practice.md`)
- [ ] Блок 7: Async (`07-async-io.md`, `07-async-io-practice.md`)

---

## Рекомендуемый роадмап (блоки и файлы)

### Блок 1: Основы синтаксиса и первая программа

**Файлы:** `01-basics.md` + `01-basics-practice.md` · пример: `examples/hello.py`

**Цель:** Установить интерпретатор, запустить код, понять базовый синтаксис и `venv`.

**Темы:** установка, REPL, `__main__`, отступы, базовые типы, f-strings, `pip`, `sys.argv`.

---

### Блок 2: Функции, модули и область видимости

**Файлы:** `02-functions-modules.md` + `02-functions-modules-practice.md`

**Цель:** Сильные функции, осознанные импорты, LEGB и замыкания.

**Темы:** `def`, default и keyword-only, `*args` / `**kwargs`, `lambda`, `global` / `nonlocal`, пакеты, `argparse`, `-m`.

**Практика (кратко):** merge dict, CLI subcommands, counter factory, пакет `textkit`.

---

### Блок 3: ООП и протоколы

**Файлы:** `03-oop-protocols.md` + `03-oop-protocols-practice.md`

**Цель:** Классы, dataclass, dunder-методы, итераторы, context managers, Protocol/ABC.

**Практика:** банковский счёт, `Vector2D`, `StepRange`, временный `cwd`.

---

### Блок 4: Ошибки и файлы

**Файлы:** `04-errors-files.md` + `04-errors-files-practice.md`

**Цель:** Исключения без анти-паттернов, `pathlib`, безопасная работа с файлами.

**Практика:** типизированные ошибки, копирование дерева, JSONL, atomic write.

---

### Блок 5: Type hints и качество кода

**Файлы:** `05-type-hints-quality.md` + `05-type-hints-quality-practice.md`

**Цель:** Аннотации, generics, Protocol, mypy/pyright, ruff.

**Практика:** типизация модуля, TypedDict config, `pyproject.toml`, Storage protocol.

---

### Блок 6: Тестирование и упаковка

**Файлы:** `06-testing-packaging.md` + `06-testing-packaging-practice.md`

**Цель:** pytest, фикстуры, параметризация, src layout, editable install.

**Практика:** `clamp` + raises, mock HTTP, мини-проект, coverage (опционально).

---

### Блок 7: Async и I/O

**Файлы:** `07-async-io.md` + `07-async-io-practice.md`

**Цель:** `asyncio`, `gather`, семафоры, `to_thread`, понимание лимитов async.

**Практика:** параллельные fake_fetch, semaphore, blocking в thread, echo server (опционально).

---

## Полезные ссылки

- [The Python Tutorial](https://docs.python.org/3/tutorial/index.html)
- [PEP 8 — Style Guide](https://peps.python.org/pep-0008/)
- [Real Python](https://realpython.com/)
- [Python Packaging User Guide](https://packaging.python.org/)
