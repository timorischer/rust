struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1_name: String = String::from("USERNAME");
    let user1_mail: String = String::from("user@mail.com");

    let user1 = build_user(user1_mail, user1_name);

    let user2 = User {
        email: String::from("another@mail.com"), 
        ..user1
    };

    println!("{}", user1.email);
}
