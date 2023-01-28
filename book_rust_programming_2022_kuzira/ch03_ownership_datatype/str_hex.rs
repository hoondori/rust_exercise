fn main() {
    hex_dump("성공하는 사람은 송곳처럼 어느 한 점을 향하여 일한다.");
}

fn hex_dump(s: &str) {
    
    for (i, b) in s.bytes().enumerate() {
        if i % 16 == 0 { print!("{:08x}|", i); }  // 주소 표시
        if i % 4 == 3 { // 4자리씩 끊어 문자를 표시
            print!("{:02x}|", b);   // 끝에 도달시
        } else {
            print!("{:02x} ", b);
        }
        if i % 16 == 15 { println!(""); }  // 줄바꿈
    }
}