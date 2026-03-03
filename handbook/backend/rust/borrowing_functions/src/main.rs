// Function that borrows a String (immutable reference)
// The String remains valid after this function call
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but nothing is dropped (it's just a reference)

// Function that borrows a String mutably
// This allows modifying the String without taking ownership
fn append_text(s: &mut String, text: &str) {
    s.push_str(text);
}

// Additional exercise: capitalize first letter
// This is tricky because String stores UTF-8 bytes
fn capitalize_first_letter(s: &mut String) {
    if s.is_empty() {
        return;
    }
    
    // Get the first character
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        let capitalized = first_char.to_uppercase().collect::<String>();
        let rest: String = chars.collect();
        *s = format!("{}{}", capitalized, rest);
    }
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Длина '{}' равна {}", s1, len); // s1 is still valid!
    
    let mut s2 = String::from("hello");
    append_text(&mut s2, ", world");
    println!("{}", s2); // "hello, world"
    
    // Can have multiple immutable references
    let s3 = String::from("test");
    let r1 = &s3;
    let r2 = &s3;
    println!("{}, {}", r1, r2); // OK
    
    // Additional exercise
    let mut s4 = String::from("hello");
    capitalize_first_letter(&mut s4);
    println!("{}", s4); // "Hello"
}
