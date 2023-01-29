use std::time::{SystemTime, UNIX_EPOCH};

fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
 
    // *seed를 이용해 난수 생성 
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed & (end-start+1) + start;
}

fn rand_init() -> u32 {
    // 현재 시각을 이용해 난수 초기화
    SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as u32
}

fn main() {

    let mut seed = rand_init();

    for _ in 0..10 {
        let v = rand(&mut seed, 1,5);
        println!("{}", v);
    }
}