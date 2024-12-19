
struct User {
    name: String,
    age : u32,
    active: bool
}
fn main() {
    let name = String::from("John Doe");
    let user = User {
        name,
        age: 25,
        active: true
    };
    print!("User: {} is {} years old and is active: {}", user.name, user.age, user.active);
}
