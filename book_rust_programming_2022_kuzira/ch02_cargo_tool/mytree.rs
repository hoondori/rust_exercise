// 지정한 path의 sub directory에서 파일명에 키워드가 들어간 것을 순회 검색

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();   // 벡터로 취득

    if args.len() < 2 {
        println!("mytree (path)");
        return; 
    }

    let target = path::PathBuf::from(&args[1]);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {

    let files = target.read_dir().expect("존재하지 않는 경로");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        // level 만큼 들여쓰기
        for _ in 1..=level {
            print!("|  ");
        }

        // 파일 이름 
        let filename = path.file_name().unwrap().to_string_lossy();

        // 디렉토리라면 재귀적으로 표시
        if path.is_dir() {
            println!("|-- <{}>", filename);
            tree(&path, level+1);
            continue;
        }
        // 파일 이름 표시
        println!("|-- {}", filename);
    }  
}