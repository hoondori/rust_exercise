// 거스름돈 계산 - brute-force으로 가능한 거스름돈 조합 모두 출력
// - 500원짜리 10개, 100원짜리 3개, 50원짜리 10개 있음
fn main() {
    let price: i64 = 3950;  // 거스름돈

    // 각 동전이 몇 개 있는지
    let count500:i64 = 10;
    let count100:i64 = 3;
    let count50:i64 = 10;
    
    for i500 in 0..(count500+1) {
        for i100 in 0..(count100+1) {
            for i50 in 0..(count50+1) {
                let total:i64 = i50 * 50 + i100 * 100 + i500 * 500;
                if price == total {
                    println!("500원x{} + 100원x{} + 50원x{} = {}원", i500, i100, i50, price);
                }
            }
        }
    }
}