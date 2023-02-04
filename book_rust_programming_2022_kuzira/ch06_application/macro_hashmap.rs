
// Hashmap 초기화 매크로

macro_rules! map_init {
    ( $($key:expr => $val:expr), *) => {{
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key, $val);
        )*
        tmp // 객체 반환
    }};
}

fn main() {

    let week = map_init![
        "mon" => "월요일",
        "tue" => "화요일"
    ];
    println!("mon={}", week["mon"]);
    println!("tue={}", week["tue"]);
}
