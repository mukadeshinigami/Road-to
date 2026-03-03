// Exercise 5: Count words in a string
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// Exercise 6: Reverse a string
fn reverse_string(s: &mut String) {
    let reversed: String = s.chars().rev().collect();
    *s = reversed;
}

// Exercise 7: Check if string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let s_lower: String = s.chars().filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let reversed: String = s_lower.chars().rev().collect();
    s_lower == reversed
}

// Exercise 8: Remove all spaces from a string
fn remove_spaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// Exercise 9: Replace all occurrences of a substring
fn replace_all(s: &mut String, old: &str, new: &str) {
    *s = s.replace(old, new);
}

// Exercise 10: Check if string starts with a prefix
fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

fn main() {
    // Exercise 5: Count words
    let s = "hello world rust";
    println!("Количество слов: {}", count_words(s)); // 3
    
    // Exercise 6: Reverse string
    let mut s6 = String::from("hello");
    reverse_string(&mut s6);
    println!("{}", s6); // "olleh"
    
    // Exercise 7: Palindrome check
    println!("{}", is_palindrome("racecar")); // true
    println!("{}", is_palindrome("hello"));   // false
    println!("{}", is_palindrome("A man a plan a canal Panama")); // true (ignoring spaces/case)
    
    // Exercise 8: Remove spaces
    let s8 = "hello world rust";
    let result = remove_spaces(s8);
    println!("{}", result); // "helloworldrust"
    
    // Exercise 9: Replace all
    let mut s9 = String::from("hello world hello");
    replace_all(&mut s9, "hello", "hi");
    println!("{}", s9); // "hi world hi"
    
    // Exercise 10: Check prefix
    println!("{}", starts_with("hello", "he")); // true
    println!("{}", starts_with("hello", "lo"));  // false
}
