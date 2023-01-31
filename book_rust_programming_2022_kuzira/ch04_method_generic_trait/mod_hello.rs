mod hello {
    pub mod world {
        pub fn say() -> String {
            String::from("안녕 세계")
        }
    }

    pub mod korea {
        pub fn say() -> String {
            String::from("안녕 한국")
        }

        pub fn say_to_all() -> String {
            let s1 = super::world::say();
            let s2 = self::say();

            s1 + &s2
        }
    }
}

fn main() {
    println!("{}", hello::world::say());
    println!("{}", hello::korea::say());
    println!("{}", crate::hello::korea::say_to_all());
}
