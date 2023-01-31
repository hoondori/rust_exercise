//! # RPN Calc
//! Reverse Polish notation(RPN) Calc
//! # Example
//! ```
//!     let src = String::from(" 1 2 + 3 *");
//!     let a = rpn_calc::eval(src).unwrap();
//!     println!("{}", a); // -> 9
//! ```

pub fn eval(formula:String) -> Result<f64, String> {
    // 계산용 스택
    let mut stack: Vec<f64> = vec![];

    let tokens = formula.split_whitespace();
    for tok in tokens {
        let t = tok.trim();
        if t == "" {continue;}

        //  숫자면 스택에 PUSH
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,  // <--- ugly
        };

        // 연산자라면 2번을 POP 하고 계산 후 결과를 PUSH
        let b = stack.pop().unwrap_or(0.0);
        let a = stack.pop().unwrap_or(0.0);
        match t {
            "+" => stack.push(a+b),
            "-" => stack.push(a-b),
            "*" => stack.push(a*b),
            "/" => stack.push(a/b),
            _ => panic!("계산이 불가능한 연산자: {}", t),
        }
    }

    // 결과 반환
    if stack.len() == 0 { return Err(format!("no result"));}
    if stack.len() > 1 {
        return Err(format!("too many value in result"));
    }
    Ok(stack.pop().unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(eval("1 3 +".to_string()), Ok(4.0));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0));
        assert_eq!(eval("6 3 /".to_string()), Ok(2.0));
        assert_eq!(eval("1 2 + 3 *".to_string()), Ok(9.0));
        //assert_eq!(eval("1 2 3".to_string()), Err("too many value in result"));
    }
}
