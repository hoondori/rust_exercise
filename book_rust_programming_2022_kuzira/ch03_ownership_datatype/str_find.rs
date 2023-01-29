
fn main() {
    let s = "There is more happiness in giving";

    // matcher로 검색
    match s.find("happiness") {
        Some(idx) => println!(" at {}", idx),
        None => println!("no happiness"),
    }

    // 클로져로 검색 
    let res = s.find(|c:char| c == 'h');
    match res {
        Some(idx) => println!(" at {}", idx),
        None => println!("no h"),
    }

    // 문자열 치환 (with variable shadowning)
    let s = s.replace("happiness", "sadness");
    println!("{}", s);

    
}