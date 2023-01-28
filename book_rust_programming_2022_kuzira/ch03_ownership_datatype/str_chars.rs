fn main() {
    let p: &str = "구슬이 서 말이라도 꿰어야 보배";
    // 1바이트씩 출력
    for b in p.bytes() {
        print!("{:2x} ", b);
    }
    println!("byte = {}B", p.len());

    // 1자씩 출력
    for c in p.chars() {
        print!("[{}] ", c);
    }
    println!("\n글자수 = {}자", p.chars().count());

    // 1자씩 출력 - Vec<char> 변환후 처리 
    let p_chars: Vec<char> = p.chars().collect();
    println!("Vec<char>: {:?}", p_chars);
    println!("글자수 = {}자", p_chars.len());
}