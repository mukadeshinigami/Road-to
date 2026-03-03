// Basic version: find first word by searching for first space
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // Search for the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    // If no space found, return the whole string
    &s[..]
}

// Improved version using string methods
// This handles multiple spaces better
fn first_word_improved(s: &str) -> &str {
    s.split_whitespace()
        .next()
        .unwrap_or("")
}

// Additional exercise: get all words as a vector of slices
fn all_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    let s1 = String::from("hello world");
    let word1 = first_word(&s1);
    println!("Первое слово в '{}': '{}'", s1, word1);
    
    let s2 = "rust programming";
    let word2 = first_word(s2);
    println!("Первое слово в '{}': '{}'", s2, word2);
    
    let s3 = "norust";
    let word3 = first_word(s3);
    println!("Первое слово в '{}': '{}'", s3, word3);
    
    // s1 is still valid!
    println!("Исходная строка: {}", s1);
    
    // Test improved version
    let s4 = String::from("   hello   world   ");
    let first = first_word_improved(&s4.trim());
    println!("Первое слово (improved): '{}'", first);
    
    // Additional exercise
    let s5 = "hello world rust";
    let words = all_words(s5);
    println!("Слова: {:?}", words); // ["hello", "world", "rust"]
}
