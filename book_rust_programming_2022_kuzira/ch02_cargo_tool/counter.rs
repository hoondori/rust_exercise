
use std::collections::HashMap;

const V_DATA: &str = "C,C,A,A,B,C,C,B,B,C,A,A,B,A,B,C";

fn main() {
    // 집계용 hashmap 작성
    let mut c_map: HashMap<&str, u8> = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    // 투표 데이터 집계 
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
    }

    // 집계 결과 표시
    for (k, v) in c_map.iter() {
        println!("{}: {:>2}", k, v);
    }

    match c_map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("D는 존재하지 않음"),
    }

}