// linked list를 recursive 구조체로 표현하고 싶다.
pub struct Node {
    data: i64,
    link: Option<Node> // 하지만.. 재귀 타입 구조체는 실제 어느 정도 메모리를 할당해야 할지 알지 못해서 컴파일 에러 유발
                       // error[E0072]: recursive type `Node` has infinite size
}

fn main() {
    let node = Node{data: 30, link: None};
    println!("{}", node);
}