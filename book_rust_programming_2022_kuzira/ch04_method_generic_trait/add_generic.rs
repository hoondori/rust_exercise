
fn add<T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
    a + b
}

fn doubling <T> (a:T) -> T 
    where T: std::ops::Add<Output=T> + Copy
{
    a + a
}

fn main() {
    println!("{}", add(10, 25));
    //println!("{}", add(10, 25.0));   // 두 인자가 같은 타입이어야 한다.
    //println!("{}", add(10.0, 25));   // 두 인자가 같은 타입이어야 한다.
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25));
    //println!("{}", add::<char>('a', 'a'));

    println!("{}", doubling::<i32>(10));
    println!("{}", doubling::<f64>(21.1));
}