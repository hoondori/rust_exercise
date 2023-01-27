// 명령줄 인수를 더하는 프로그램
fn main() {

    // 명령줄 인수 취득
    let args = std::env::args();

    let mut total = 0.0;

    for (i, v) in args.enumerate() {
        if i == 0 { continue; }
        let num: f64 = match v.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }

    println!("{}", total);
}