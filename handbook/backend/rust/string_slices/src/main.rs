// Function that returns the first word as a string slice
// Takes &str to work with both String and &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // Find the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    // If no space found, return the whole string
    &s[..]
}

// Function that returns the last word as a string slice
fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // Find the last space by iterating backwards
    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    
    // If no space found, return the whole string
    &s[..]
}

// Additional exercise: get word at specific index
// Returns Option<&str> - Some(word) if found, None otherwise
fn word_at_index(s: &str, index: usize) -> Option<&str> {
    s.split_whitespace().nth(index)
}

// Additional exercise: get all words as a vector of slices
fn all_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    let s = String::from("hello world");
    
    let first = first_word(&s);
    println!("Первое слово: {}", first); // "hello"
    
    let last = last_word(&s);
    println!("Последнее слово: {}", last); // "world"
    
    // s is still valid!
    println!("Вся строка: {}", s);
    
    // Test with string literal (also &str)
    let s2 = "rust programming language";
    let first2 = first_word(s2);
    println!("Первое слово в '{}': '{}'", s2, first2);
    
    // Additional exercises
    let s3 = "hello world rust";
    println!("{:?}", word_at_index(s3, 0)); // Some("hello")
    println!("{:?}", word_at_index(s3, 1)); // Some("world")
    println!("{:?}", word_at_index(s3, 10)); // None
    
    let words = all_words(s3);
    println!("Слова: {:?}", words); // ["hello", "world", "rust"]
}
