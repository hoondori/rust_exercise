
fn encrypt(text: &str, shift: i16) -> String {
    // 'A' 와 'Z' 의 문자코드를 i16 타입으로 취득

    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    let mut result = String::new();

    // 한 글자씩 읽어가면서 shift만큼 code를 이동
    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }

    return result;
}

fn main() {

    let shift = 3;
    let message = "I LOVE RUST";
    let enc = encrypt(message, shift);
    let dec = encrypt(&enc, -shift);
    println!("original message = {}", message);
    println!("enc message = {}", enc);
    println!("dec message = {}", dec);
}
