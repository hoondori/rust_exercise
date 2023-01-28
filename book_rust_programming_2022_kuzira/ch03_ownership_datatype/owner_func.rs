fn main() {
    let g1 = String::from("A");
    show_message(&g1); // A, borrow
    println!("{}", g1);  // A, // 소유권 이전이 없으므로 출력 가능

    let m = generage_message();
    println!("{}", m); // B

    let mut s = String::from("C");
    modify_message(&mut s);
    println!("{}", s);   // _C!
}

fn show_message(message: &String) {
    println!("{}", message);
}

fn generage_message() -> String {
    let msg = String::from("B");
    return msg;
}

fn modify_message(message: &mut String) {
    message.insert(0, '_');
    message.push('!');
}