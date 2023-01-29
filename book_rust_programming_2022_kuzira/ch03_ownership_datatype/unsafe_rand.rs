use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        // 현재 시각을 이용해 난수 초기화
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoch.as_millis() as u32;
    }

    // SEED를 이용해 난수 생성 
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED & (end-start+1) + start;
}

fn main() {
    for _ in 0..100 {
        unsafe { // error[E0133]: call to unsafe function is unsafe
            let v = rand_global(1,5);
            println!("{}", v);
        }
    }
}