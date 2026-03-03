// Function that takes ownership of a String
// After this function, the String is dropped and memory is freed
fn take_string(s: String) {
    println!("Взял владение: {}", s);
} // s goes out of scope here, memory is freed

// Function that takes ownership and returns it back
// This demonstrates how to return ownership
fn return_string(s: String) -> String {
    println!("Получил: {}", s);
    s // Return ownership back to caller
}

// Additional exercise: swap two strings
// Takes ownership of two strings and returns them in reverse order
fn swap_strings(s1: String, s2: String) -> (String, String) {
    (s2, s1) // Return tuple with swapped ownership
}

fn main() {
    let s1 = String::from("hello");
    take_string(s1);
    // println!("{}", s1);  // ERROR! s1 is no longer valid after move

    let s2 = String::from("world");
    let s3 = return_string(s2);
    println!("Вернули: {}", s3); // OK — ownership was returned

    // Additional exercise
    let s4 = String::from("first");
    let s5 = String::from("second");
    let (s6, s7) = swap_strings(s4, s5);
    println!("{} {}", s6, s7); // "second first"
}
