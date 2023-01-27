// 지정한 path의 sub directory에서 파일명에 키워드가 들어간 것을 순회 검색

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();   // 벡터로 취득

    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return; 
    }

    let target = path::PathBuf::from(&args[1]);
    let keyword = &args[2];    
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {

    let files = target.read_dir().expect("존재하지 않는 경로");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        // 파일 이름 
        let filename = path.file_name().unwrap().to_string_lossy();
        // 검색어 포함여부 
        if filename.find(keyword) == None { continue; }
        // 검색어를 포함하는 경로를 표시
        println!("{}", path.to_string_lossy());

    }  
}