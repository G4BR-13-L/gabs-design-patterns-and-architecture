#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    email: Option<String>,
    age: Option<u32>,
}

struct UserBuilder {
    name: String,
    email: Option<String>,
    age: Option<u32>,
}

impl UserBuilder {
    fn new(name: &str) -> Self {
        UserBuilder {
            name: name.to_string(),
            email: None,
            age: None,
        }
    }

    fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    fn build(self) -> User {
        User {
            name: self.name,
            email: self.email,
            age: self.age,
        }
    }
}

fn main() {
    let user1 = UserBuilder::new("Gabriel")
        .email("gabrieluser@email.com")
        .age(23)
        .build();

    let user2 = UserBuilder::new("Lucas").build();

    println!("{:?}", user1);
    println!("{:?}", user2);
}
