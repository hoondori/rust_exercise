
/// 두 수를 **더하는** 기능
/// 
/// 주어진 [`left`](left:usize) 와 [`right`](right:usize) 를 더한다. 
/// 
/// 양쪽 숫자가 같아도 정상적으로 작동한다. 
/// 
///     assert_eq!(doctest::add(2,2),4);
/// 
/// 양쪽 숫자가 달라도 정상적으로 작동한다. 
/// ```
/// assert_eq!(doctest::add(4,6),10);
/// ```
/// 
/// 아래 예제는 테스트 대상에서 제외
/// ```no run
/// assert_eq!(doctest:add(4,6),10);
/// ```
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
}
