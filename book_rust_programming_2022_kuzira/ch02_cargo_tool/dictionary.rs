// 사전에서 검색

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();   // 벡터로 취득

    if args.len() < 2 {
        println!("찾을 키워드를 지정해 주세요");
        return 
    }

    let dict_filename = "dict.txt";
    let word = &args[1];

    let fp:File = File::open(dict_filename).unwrap();
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        // 주어진 단어 포함 여부 확인
        if line.find(word) == None {
            continue;
        }
        println!("{}", line);
        break;
    }
    
}