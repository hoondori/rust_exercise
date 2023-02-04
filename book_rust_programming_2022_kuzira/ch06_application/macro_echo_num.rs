
#[macro_export] // 외부에서 매크로 이용할 수 있게
macro_rules! echo_num {
    ($num:expr) => { println!("{}", $num)};
}

#[macro_export]
macro_rules! echo_nums {
    ( $($num:expr), *) => { 
        $(
            print!("{}, ", $num);
        )*
        println!("");
    };
}
fn main() {
    echo_num!(10);
    echo_num!(20);
    echo_nums![10,20,30,40];
}