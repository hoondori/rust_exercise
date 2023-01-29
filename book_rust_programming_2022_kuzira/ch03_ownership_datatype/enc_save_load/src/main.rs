use encoding_rs;
use std::{fs, fs::File, io::Write};

fn main() {
    let filename = "test_euc_kr.txt";
    let text = "맛있게 먹으세요";

    save_euckr(filename, text);

    let s = load_euckr(filename);
    println!("{}", s);
}

fn save_euckr(filename: &str, text: &str) {
    let (enc, _, _) = encoding_rs::EUC_KR.encode(text);
    let buf = enc.into_owned();  // cow로 소유권 이전
    let mut file = File::create(filename).expect("create");
    file.write(&buf[..]).expect("write");
}

fn load_euckr(filename: &str) -> String {
    let buf = fs::read(filename).expect("read");
    let (dec, _, _) = encoding_rs::EUC_KR.decode(&buf);
    return dec.into_owned();
}