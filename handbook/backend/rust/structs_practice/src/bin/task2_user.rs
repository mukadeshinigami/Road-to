/// Task 2: User with validation.

struct User {
    username: String,
    email: String,
}

impl User {
    /// Constructs a new `User` from owned strings.
    fn new(username: String, email: String) -> Self {
        Self { username, email }
    }

    /// Checks that `email` contains `@`.
    fn is_valid_email(&self) -> bool {
        // TODO: Implement the email validation logic.
        let _ = &self.email;
        false
    }

    /// Checks that the username is not empty and not longer than 20 characters.
    fn is_valid_username(&self) -> bool {
        // TODO: Implement the username validation logic.
        let _ = &self.username;
        false
    }
}

fn main() {
    let user_valid = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    let user_invalid = User::new(
        String::from(""),
        String::from("not-an-email"),
    );

    println!(
        "valid user: email_ok={}, username_ok={}",
        user_valid.is_valid_email(),
        user_valid.is_valid_username()
    );
    println!(
        "invalid user: email_ok={}, username_ok={}",
        user_invalid.is_valid_email(),
        user_invalid.is_valid_username()
    );
}

