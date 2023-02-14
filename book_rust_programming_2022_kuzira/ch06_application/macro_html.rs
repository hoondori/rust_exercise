
// 재귀적으로 매크로 호출
macro_rules! out_html {
    // 인수가 없을 때, 재귀호출시 base
    () => {()};  

    // 인수가 1개일때, ex) ["This is test"], , 재귀호출시 base
    ($e:tt) => {print!("{}", $e);};

    // 태그[안쪽]을 계속 지정하는 경우 
    // html [ head, rest* ]
    ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html!( $($inner)* );
        println!("</{}>", stringify!($tag));
        out_html!( $($rest)* );
    }}
}

fn main() {
    // 매크로를 이용해 HTML 구조 출력
    out_html!(
        html [
            head[title["test"]]
            body[
                hi["test"]
                p ["This is test"]
            ]
        ]
    )
}