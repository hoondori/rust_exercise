struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age:i32) -> Self {
        Self { name: name.to_string(), age }
    }
}

fn main() {
    let alex = Person::new("Alex", 10);
    let betty = Person {
        name: String::from("Betty"),
        ..alex   // 부분 복사
    };

    println!("{} {}", alex.name, alex.age);
    println!("{} {}", betty.name, betty.age);
}