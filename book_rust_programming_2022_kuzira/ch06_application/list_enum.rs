// 열거형을 이용해서 Option 대체
enum Node {
    Empty,
    Cons(i64, Box<Node>),
}
use Node::{Empty, Cons};

// 단방향 연결 리스트를 생성
fn node(v:i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}

fn main() {
    // unwrap이 더 이상 필요없음
    let my_list: Box<Node> = node(10, node(20, node(30, Box::new(Empty)))); 

    // list 출력 : None이 나올 때까지 각 요소의 link를 따라가기
    let mut ptr = &my_list;
    loop {

        ///////  UnBoxing 해서 내용물(T)에 대한 패턴 매칭 준비
        // *ptr 로 &Box<T> 에 대한 deref => Box<T>
        // **ptr 로 Box<T> 에 대한 deref => T
        // &**ptr 로 T에 대한 ref
        let cur_node = &**ptr; 

        match cur_node {
            Empty => break, // 리스트의 끝 도달
            Cons(v, link) => { 
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}