
fn main() {
    {
        let g1 = String::from("A");
        let g2 = g1; // ownership moved 
        //println!("{}", g1);  // onwership 이동으로 에러 발생 
    } // lifetime 종료로 파괴
    //println!("{}", g2); // not found in this scope

    // primitive type은 stack 영역에서 생성되므로 굳이 ownership 관리를 하지 않고 값 복사로 처리된다. 
    let g1 = 30;
    let g2 = g1;
    println!("{}", g1);
    println!("{}", g2);

    let f1 = String::from("A");
    let f2 = f1.clone(); // 소유권 이전이 아니라 값 복사가 되는 경우
    println!("{}", f1);
    println!("{}", f2);

}