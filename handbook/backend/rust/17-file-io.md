# Блок 17: Файловый ввод-вывод (File I/O)

## Цель

Работать с **файлами и директориями**: открытие, чтение, запись, буферизация, пути, обход директорий, метаданные и обработка ошибок.

---

## 1. std::fs::File

Файл открывается через `File::open(path)` (чтение) и `File::create(path)` (запись, перезаписывает). Оба возвращают `Result<File, io::Error>`.

```rust
use std::fs::File;
use std::io::Read;

let mut f = File::open("hello.txt")?;
let mut buf = String::new();
f.read_to_string(&mut buf)?;
```

Опции: `OpenOptions::new().read(true).write(true).append(true).open(path)` для тонкой настройки.

---

## 2. Чтение: read_to_string, BufReader

- **read_to_string** — прочитать весь файл в строку (удобно для небольших файлов). Есть в `std::fs::read_to_string(path)` без явного открытия File.
- **BufReader** — обёртка над `Read`, буферизует ввод; уменьшает число системных вызовов при построчном или побайтовом чтении.

```rust
use std::io::BufRead;
let f = File::open("log.txt")?;
let reader = std::io::BufReader::new(f);
for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}
```

---

## 3. Запись: write, BufWriter

- **write**, **write_all** — запись в файл. `std::fs::write(path, bytes)` — записать срез байтов целиком (перезаписывает).
- **BufWriter** — буферизует вывод; сбрасывает при drop или явном `flush()`.

```rust
use std::io::Write;
let mut f = File::create("out.txt")?;
f.write_all(b"hello")?;
f.flush()?;
```

---

## 4. Создание и удаление файлов/директорий

- **create_dir**, **create_dir_all** — создать директорию (all создаёт вложенные).
- **remove_file** — удалить файл.
- **remove_dir**, **remove_dir_all** — удалить директорию (all рекурсивно).

Все в `std::fs`, возвращают `Result`.

---

## 5. Работа с путями: Path, PathBuf

- **Path** — срез, неизменяемое представление пути (аналогично str для String).
- **PathBuf** — владеющий, изменяемый путь (push, pop, join).

```rust
use std::path::Path;

let p = Path::new("/home/user/file.txt");
p.file_name();   // Some(OsStr("file.txt"))
p.extension();   // Some(OsStr("txt"))
p.parent();      // Some(Path("/home/user"))
```

Сборка пути: `path.join("subdir").join("file.txt")`; для кроссплатформенности лучше использовать Path/PathBuf, а не конкатенацию строк.

---

## 6. Обход директорий: read_dir

`std::fs::read_dir(path)` возвращает итератор по `Result<DirEntry, io::Error>`. У `DirEntry`: `path()`, `file_name()`, `file_type()`. Рекурсивный обход — самостоятельно обходить поддиректории (или использовать крейт walkdir).

```rust
for entry in std::fs::read_dir(".")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}
```

---

## 7. Метаданные файлов

`std::fs::metadata(path)` и `Path::metadata()` возвращают `Metadata`: размер (`len()`), это файл или директория (`is_file()`, `is_dir()`), права, время модификации (на некоторых платформах) и т.д.

---

## 8. Обработка ошибок

Файловые операции возвращают `io::Error`. Использовать `?` для проброса, конвертировать в свой тип ошибки через `From` или `map_err`. Проверять «файл не найден», «нет прав», «это директория» по `kind()` (например, `ErrorKind::NotFound`).

---

## 9. Итоги

- File::open/create, read_to_string, BufReader для чтения; write/write_all, BufWriter для записи.
- fs::write, fs::read_to_string — быстрые хелперы для небольших файлов.
- Path/PathBuf для путей; read_dir для обхода; metadata для сведений о файле.
- Все операции через Result; обрабатывать io::Error.

---

## 10. Что дальше?

Блок 18 — **сети и HTTP**: reqwest, tokio, простой сервер, JSON.

---

## 11. Ресурсы

- [The Book: I/O](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) (и std::fs, std::io)
- [std::fs](https://doc.rust-lang.org/std/fs/)
- [std::path](https://doc.rust-lang.org/std/path/)
