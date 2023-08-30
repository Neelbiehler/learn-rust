use std::io;

fn passwort_input() {
    let mut finish = false;

    while !finish {
        println!("Enter your password: ");
        let mut password = String::new();
        for _ in 0..3 {
            io::stdin().read_line(&mut password).expect("Failed to read line");
            validate_password(&password);
            println!("Do you want to try again? (y/n)");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read line");
            if answer.trim() == "n" {
                break;
            }
            password.clear();
        }
        io::stdin().read_line(&mut password).expect("Failed to read line");
        validate_password(&password);
        println!("Do you want to try again? (y/n)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer == "n" {
            finish = true;
        }
    }
}

fn validate_password(password: &str) {
    if password.len() < 8 {
        println!("Password must be at least 8 characters long");
    }
    if !password.contains(char::is_uppercase) {
        println!("Password must contain at least one uppercase letter");
    }
    if !password.contains(char::is_lowercase) {
        println!("Password must contain at least one lowercase letter");
    }
    if !password.contains(char::is_numeric) {
        println!("Password must contain at least one number");
    }
    if !password.contains(char::is_alphanumeric) {
        println!("Password must contain at least one special character");
    }
}

pub fn password_input() {
    passwort_input();
}
