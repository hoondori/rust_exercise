use std::{rc::Rc, cell::RefCell};
fn main() {
    let a_rc = Rc::new(1000);
    {
        let b_rc = Rc::clone(&a_rc); // 공동 소유
        println!("{}", b_rc);
        println!("참조 카운트 = {}", Rc::strong_count(&a_rc));   // 소유자 2명
    } // b 에 대한 소유권은 소멸하지만 a에 대한 소유권은 살아있다.
    println!("{}", a_rc); // 사용 가능 
    println!("참조 카운트 = {}", Rc::strong_count(&a_rc));   // 소유자 1명
}

// 양방향 연결 리스트 정의 (순환 참조 발생)
// b는 a에 의해, c 에 의해 복수 소유권 필요 => Rc<T> 사용
// 하지만  a->b, b->a 라는 순환 참조 발생
//     a -> b -> c 
//     c -> b -> a 
struct Node {
    data: isize,
    prev: Option<Weak<RefCell<Nonde>>>,              
    next: Option<Rc<RefCell<Nonde>>>,              
}