fn main() {
    let s1 = "안녕하 세요";
    println!("{}", &s1[0..3]);  // 한글은 3 byte씩 하나의 char 형성
    println!("{}", &s1[3..6]);
    println!("{}", &s1[6..9]);
    println!("{}", &s1[10..13]);  // 9..10 구간은 whitespace

    // &str 과 String 의 상호변환
    let ss: &str = "베풀면 반드시 돌아온다";
    
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();

    let ss2: &str = &so1;
    let ss3: &str = so2.as_str();

    println!("{}, {}, {}, {}", so1, so2, ss2, ss3);
    println!("{:p}, {:p}, {:p}",ss, ss2, ss3);
}