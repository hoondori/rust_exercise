pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    // unused_must_use 는 사용되지 않는 것에 warning뜨는 것을 묵음처리
    // unconditional_panic은 항상 발생하는 panic이므로 컴파일 에러 방지
    // should_panic은 test 결과에 반드시 오류가 발생해야 함을 암시
    #[test]
    #[allow(unconditional_panic, unused_must_use)]
    #[should_panic(expected="divide by zero")]
    fn test_divide_by_zero() {
        1/0;
    }
}
