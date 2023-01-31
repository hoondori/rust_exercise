fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string()
    ];

    for a in array.iter() {   // 그냥 in array 를 하면 소유권이 이전되는 side-effect 발생
        println!("{}", a);
    }

    println!("len={}", array.len());
}