fn main10() {

    struct Person { name: String, birth: i32 }

    // new()함수 내부에서 만들어진 vec![]는 move되서 compser가 소유권을 갖는다.
    let mut composers = Vec::new();  

    // to_string() 은 String 객체로 변환된 후에 소유권을 Person 객체에 넘긴다. 
    // 생성된 Person 객체는 push 메소드에 값으로 전달되고(moved) 
    // 이후의 소유권은 벡터배열이 갖게 된다. 
    // 벡터 배열의 소유권은 composers 에 있다. 
    composers.push( Person { name: "kim".to_string(), birth: 15 } );

}

fn main9() {

    fn f(v: Vec<u32>) { println!("{:?}", v);}
    fn g(v: Vec<u32>) { println!("{:?}", v);}
    fn h(v: Vec<u32>) { println!("{:?}", v);}

    fn test(c: bool) {

        let x = vec![10,20,30];

        if c {  
            f(x) // x 가 여기서 move될 수도 있고
        } else {
            g(x) // x 가 여기서 move될 수도 있고
        }
        
        // 어느 경우던지 x는 move 된 것이 확실한 상황 
        //h(x)   <-- 여기서 컴파일 에러
    }

}

fn main8() {

    fn f(v: Vec<u32>) { println!("{:?}", v);}

    let x = vec![10,20,30];
    loop {
        // f(x) // x 가 여기서 이동  
        // error[E0382]: use of moved value: `x`
        //  value moved here, in previous iteration of loop
    }
}


fn main7() {

    let x = vec!["hello".to_string(), "world".to_string()];

    // collection 내의 특정 요소만 쏘옥 move할 수 없다. 
    // 이를 허용하면 collection은 어떤 index가 move된 것인지를 일일이 기억해야 하기 때문에 곤란
    //let third = x[2];   
    //   error[E0507]: cannot move out of index of `Vec<String>`
    //   move occurs because value has type `String`, which does not implement the `Copy` trait
    //   help: consider borrowing here: `&x[2]`
}


fn main6() {

    let mut x = vec!["hello".to_string(), "world".to_string()];

    // 끝에서 꺼내는 거라면 move 허용
    let popped = x.pop().expect("vector empty!");

    // 대신해서 채워넣을 것이 있으면 move 허용
    let a = x.swap_remove(1); // index로 준 위치를 꺼내고 마지막 요소를 그 자리에 replace
    let b = std::mem::replace(&mut x[2], "substitude".to_string()); // 꺼내려는 값을 다른 값과 맞바꾼다. 
}

fn main5() {

    let v1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];

    let mut v2:Vec<String> = vec![];

    for mut s in v1 {  // iteration에서 끄낼 때부터 원래 주인으로부터 이동(moved)
        s.push('!'); //  문자열 변경 
        v2.push(s); // 이젠 소유권이 v2로 이전된다. 
    }

    //println!("{:?}", v1);   // iter() 호출로 인해서 개별 요소들의 소유권을 잃어버릴 가능성이 있으므로 컴파일 에러 
                            // 설사 실제 이동을 한 요소라도 안했더라도 컴파일 에러임
    println!("{:?}", v2);
}


fn main() {

    struct A { number: u32 }
    let a1 = A { number: 1};
    let a2 = a1;   //  Copy가 아니라 Move
    // println!("{}", a1.number);   //  소유권을 잃어버려서 쓸 수 없다. 

    #[derive(Copy, Clone)]
    struct B { number: u32 }
    let b1 = B { number: 1};
    let b2 = b1;   //  Move가 아니라 Copy
    println!("{}", b1.number);   //  소유권을 각각 가지고 있으므로 출력 가능 

    // error[E0204]: the trait `Copy` may not be implemented for this type
    // #[derive(Copy, Clone)]    <-- String 은 Copy 불가
    struct C { name:String }
    

}