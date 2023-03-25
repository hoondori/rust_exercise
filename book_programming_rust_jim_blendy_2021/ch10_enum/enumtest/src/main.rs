use std::f32::consts::E;


// C-styple의 enum
fn main1() {

    // 값은 정수 형태로 메모리에 저장. 값이 커지면 u8로 안될수도 있다.
    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
    }
    use std::mem::size_of;
    assert_eq!(size_of::<HttpStatus>(), 2);  // 404는 u8에 fit이 되지 않음 

    // 정수를 enum으로 캐스팅할 수 없다. 원하면 직접 변환한다. 
    // enum_primitive 크레이트를 사용하면 자동 변환 매크로를 생성해줌 
    fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::Ok),
            404 => Some(HttpStatus::NotFound),
            _ => None,
        }
    }

    // == 를 쓰려면 derive로 명시가 필요
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum TimeUnit {
        Seconds, Minutes,
    }

    impl TimeUnit {
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::Seconds => "seconds",   // === 사용 
                TimeUnit::Minutes => "minutes",  
            }
        }

        fn singular(self) -> &'static str {
            self.plural().trim_end_matches('s')
        }
    }
}

// 데이터를 갖는 이늄
fn main2() {

    // struct enum
    enum Point3d { X, Y, Z }
    enum Shape {
        Sphere { center: Point3d, radius: f32 },
        Cuboid { corner1: Point3d, corner2: Point3d },
    }

    // unit enum과 tuple enum, struct enum이 공존할 수 있다. 
    enum RelationshipStatus {
        Single, 
        ItsComplecated(Option<String>, Option<String>),
        ItsExteremlyComplicated {
            a: String,
        }
    }

    // json 구조를 표현하기 위한 리치 데이터 구조 
    // hashmap을 box로 감싸안아줌으로 인해서 enum의 메모리 필요량이 줄어든다.  => 4 머신워드
    use std::collections::HashMap;

    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String, Json>>),
    } 

}

// 제네릭 이늄
fn main3() {

    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }
    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    // Tree 만들기
    use BinaryTree::*;
    let a_tree = NonEmpty(Box::new(TreeNode {
        element: "a",
        left: Empty,
        right: Empty,
    }));
    let b_tree = NonEmpty(Box::new(TreeNode {
        element: "b",
        left: Empty,
        right: Empty,
    }));
    let root_tree = NonEmpty(Box::new(TreeNode{
        element: "root",
        left: a_tree,
        right: b_tree,
    }));


}

// 패턴 매칭으로 값에 접근
fn main() {
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum TimeUnit {
        Seconds, Minutes,
    }
    enum RoughTime {
        InThePast(TimeUnit, u32),
        JustNow,
    }
    fn rough_time_to_english(rt: RoughTime) -> String {
        // 표현식은 값을 생산하고 패턴은 값을 소비한다.
        match rt {
            RoughTime::InThePast(units, count) => 
                format!("{} ago", count),
            RoughTime::JustNow =>
                format!("just now"),
        }
    }

    // struct pattern 매칭
    struct Account {
        name: String,
        language: String,
        age: u8,
    }
    fn show_account(a: &Account) { println!("account");}
    fn match_account(a: Account) {
        match a {  // 필요한 필드만 참조하고, 나머지는 .. 처리
            Account{ ref name, ..} =>  (),
        }

        match a {  // ref로 참조해야지 consume 되지 않는다.
            Account { ref name, ref language, ..} => {
                show_account(&a)
            }
        }
    }
}