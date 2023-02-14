// linked list를 recursive 구조체로 표현하고 싶다.
pub struct Node {
    data: i64,
    link: Option<Box<Node>>, // Box<T> 로 힙에 생성
}

// 단방향 연결 리스트를 생성
fn node(v:i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node{data:v, link:link}))
}

fn main() {
    let my_list = node(10, node(20, node(30, None))).unwrap();
    //println!("{:?}", my_list);   // 안됨

    // list 출력 : None이 나올 때까지 각 요소의 link를 따라가기
    let mut p = &my_list;
    loop {
        println!("{}", p.data);

        match p.link {
            None => break, // 리스트의 끝 도달
            Some(ref link) => p = &link,   // ref 로 접근
        }
    }
}