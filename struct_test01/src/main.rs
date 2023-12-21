struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("email: {}", user1.email);
    println!("active: {}", if user1.active{"활성"}else{"비활성"});
    println!("username: {}", user1.username);
    println!("sic: {}", user1.sign_in_count);
    let user2 = build_user("proleesh@rust.me".to_string(), "proleesh".to_string());

    println!("email: {}", user2.email);
    println!("username: {}", user2.username);
    println!("active: {}", if user2.active{"활성"}else{"비활성"});
    println!("sic: {}", user2.sign_in_count);
}
fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
