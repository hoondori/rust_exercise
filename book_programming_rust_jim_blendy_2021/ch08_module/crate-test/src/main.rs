
pub mod world {  // 가장 바깥쪽에 pub되어야 crate 외부에서 접근 가능하다.
    mod Asia {
        pub fn fly() {}  
    }
    pub mod Europe {
        pub(crate) fn eat() {}    // pub(crate)
        fn private_thing() {}
    }
    fn world_something() {
        // self::Europe::private_thing();  <- pub로 오픈하지 않은 자식것을 쓸 수 없다.
        self::Asia::fly();  //pub로 열어주었으므로 부모인 world에서 사용 가능
        self::Europe::eat()
    }
    
}
fn main2() {
    // use world::Asia;    <-- crate 외부로 Asia는 개방되어 있지 않음
    use world::Europe::eat;  // 현재의 crate 내에서는 pub(crate)를 쓸 수 있다.

}



mod A {
    pub mod B {
        pub mod C {
            pub(in crate::A::B) mod D { 
                // 공개하고자 하는 범위를 경로로 지정 
                // 여기서는 B와 그 이하에 공개
            }
        }
        use crate::A::B::C::D; // <-- B에서는 D가 보인다. 
    }
    //use self::B::C::D;  <-- D는 B 이하에게만 공개 
}

pub mod Bank {
    pub struct Account {
        pub name: String,
        passwd: u32  // 비공개
    }

    pub fn create_account(name: String, passwd: u32) -> Account {
        Account { name:name, passwd:passwd }
    }
}

fn main() {
    use Bank::Account;
    // passwd가 비공개 필드라서 비록 Account가 공개되어 있더라도 자체 생성이 불가능하다. 
    // let a = Account { name:"James".to_string(), passwd: 33 }; 

    // 공개된 생성함수를 이용해서 생성 가능
    let a = Bank::create_account("James".to_string(), 33);

    // 하지만 private 필드의 읽기는 불가능
    println!("{}", a.name); 
    //println!("{}", a.passwd); 
}

