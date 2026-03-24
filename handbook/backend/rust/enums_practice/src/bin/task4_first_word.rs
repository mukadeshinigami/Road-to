/// Task 4: Safe `Option` — first word of a string.

fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

fn main() {
    let cases = [
        "hello world",
        "single",
        "   spaced   words  ",
        "",
        "   ",
    ];

    for s in cases {
        if let Some(word) = first_word(s) {
            println!("{s:?} -> first word: {word:?}");
        } else {
            println!("{s:?} -> None");
        }
    }

    let owned = String::from("hello world");
    if let Some(word) = first_word(&owned) {
        println!("First word: {word}");
    }
}
