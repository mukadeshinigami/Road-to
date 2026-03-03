# Блок 13: Конкурентность (Concurrency)

## Цель

Писать **многопоточный код** безопасно: потоки (thread::spawn), join handles, move-замыкания, каналы (mpsc), разделяемое состояние (Mutex, Arc), базовое понимание deadlock и трейтов Send/Sync.

---

## 1. Потоки: std::thread::spawn

Создание нового потока ОС:

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    thread::sleep(Duration::from_millis(100));
    println!("from thread");
});

handle.join().unwrap();  // дождаться завершения
```

Замыкание должно быть `'static` (не держать ссылки на стек вызывающего) и `Send` (передаваться в другой поток). Часто используют `move`, чтобы передать владение данными в поток.

---

## 2. join() и handle

`thread::spawn` возвращает `JoinHandle<T>`. Вызов `handle.join()` блокирует текущий поток до завершения порождённого и возвращает `Result<T, JoinError>`. Игнорирование join не «убивает» поток — он продолжит работу; но обычно join нужен для синхронизации и получения результата.

---

## 3. move-замыкания для потоков

Данные из текущего стека нельзя передать по ссылке в поток (время жизни). Передают по владению:

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);
});
// v здесь уже перемещён
handle.join().unwrap();
```

Без `move` замыкание захватило бы `v` по ссылке, и компилятор выдаст ошибку: ссылка может пережить текущий поток.

---

## 4. Каналы (mpsc)

Модуль `std::sync::mpsc` — multiple producer, single consumer. Передача сообщений между потоками без общих переменных.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();
});

let received = rx.recv().unwrap();  // 42
```

- `send(T)` — отправить; при закрытом канале вернёт ошибку.
- `recv()` — блокирующее получение; при закрытом канале вернёт ошибку.
- `try_recv()` — неблокирующее.

Клонирование `tx` (передача в несколько потоков) даёт несколько производителей.

---

## 5. Разделяемое состояние: Mutex<T>, Arc<Mutex<T>>

Когда нужно общее изменяемое состояние между потоками:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let m = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..2 {
    let m = Arc::clone(&m);
    handles.push(thread::spawn(move || {
        *m.lock().unwrap() += 1;
    }));
}

for h in handles {
    h.join().unwrap();
}
println!("{}", *m.lock().unwrap());
```

`Mutex::lock()` возвращает guard; при drop guard блокировка снимается. `Arc` даёт общее владение между потоками (счётчик атомарный). Poisoning: если поток паникует, держа lock, Mutex помечается «отравленным»; следующий lock вернёт Err (можно обработать или unwrap).

---

## 6. Deadlocks

Deadlock — два потока ждут друг друга (например, захватили два Mutex в разном порядке). Избегание: единый порядок захвата блокировок, по возможности уменьшать время удержания lock, использовать каналы вместо общих Mutex где возможно.

---

## 7. Send и Sync

- **Send** — тип можно безопасно передать в другой поток (владение).
- **Sync** — ссылку `&T` можно безопасно передать в другой поток (т.е. `T` можно разделять по ссылке между потоками).

Большинство стандартных типов — Send и/или Sync. Rc не Send (счётчик не атомарный). RefCell не Sync (нет атомарности при borrow). Arc и Mutex реализованы так, чтобы быть Send/Sync при подходящем T.

---

## 8. Best practices

- Предпочитать каналы для передачи данных между потоками; Mutex — когда действительно нужно общее состояние.
- Держать lock минимально; не вызывать неизвестный код под lock (риск deadlock и блокировок).
- Использовать типы из стандартной библиотеки (Arc, Mutex, mpsc), а не писать свои примитивы синхронизации.

---

## 9. Итоги

- Потоки: `thread::spawn`, move-замыкания, join.
- Каналы: mpsc::channel(), send/recv, несколько производителей через клонирование tx.
- Разделяемое состояние: Arc<Mutex<T>>; понимание Send/Sync и deadlock.

---

## 10. Что дальше?

Блок 14 — **модули и организация кода**: mod, pub, use, файловая структура, workspaces.

---

## 11. Ресурсы

- [The Book: Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust by Example: Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
