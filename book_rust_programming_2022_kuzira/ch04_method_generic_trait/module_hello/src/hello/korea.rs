pub fn say() -> String {
    String::from("안녕 한국")
}

pub fn say_to_all() -> String {
    let s1 = super::world::say();
    let s2 = self::say();

    s1 + &s2
}
