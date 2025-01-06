struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.username = String::from("anotherusername123");
    user1.email = String::from("anotheremail@example.com");

    build_user(String::from("email@42.fr"), String::from("42"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // The syntax .. specifies that the remaining fields not explicitly
                // set should have the same value
                // as the fields in the given instance.
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email, // we only need to write email rather than email: email
        sign_in_count: 1,
    }
}
