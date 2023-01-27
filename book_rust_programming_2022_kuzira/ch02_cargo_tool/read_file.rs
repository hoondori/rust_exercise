// 명령행 인자로 지정한 파일의 내용을 읽어서 표기

fn main() {
    let args: Vec<String> = std::env::args().collect();   // 벡터로 취득

    if args.len() < 2 {
        println!("읽을 파일을 지정해 주세요");
        return 
    }

    let filename = &args[1];

    let text: String = std::fs::read_to_string(filename).unwrap();   
    println!("{}", text);
}