
fn get_age_str(age: u8) -> String {
    
    let age_str: &str = match age {
        0 => "유아",
        1..=10 => "어린이",
        v if v >= 11 && v < 18 => "청소년",
        _ => "어른",
    };

    String::from(age_str)
}

fn main() {
    println!("age={} -> {}", 0, get_age_str(0));    
    println!("age={} -> {}", 8, get_age_str(8));
    println!("age={} -> {}", 12, get_age_str(12));
    println!("age={} -> {}", 120, get_age_str(120));
}