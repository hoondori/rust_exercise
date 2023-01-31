fn main() {
    // 계산용 스택
    let mut stack: Vec<f64> = vec![];

    let formula: String = String::from(" 2 3 * 5 +");

    let tokens = formula.split_whitespace();
    for tok in tokens {
        let t = tok.trim();

        //  숫자면 스택에 PUSH
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,  // <--- ugly
        };

        // 연산자라면 2번을 POP 하고 계산 후 결과를 PUSH
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a+b),
            "-" => stack.push(a-b),
            "*" => stack.push(a*b),
            "/" => stack.push(a/b),
            _ => panic!("계산이 불가능한 연산자: {}", t),
        }
    }

    // 최종 결과 표시
    println!("result={}", stack.pop().unwrap());

}