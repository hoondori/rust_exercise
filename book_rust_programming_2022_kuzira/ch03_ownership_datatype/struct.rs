struct CarSpec {
    model: i32,
    color: i32,
}

fn main() {
    let car1 = CarSpec { model:300, color:0xFF0000, };
    let car2 = CarSpec { model:200, color:0x00FF00, };
    println!("{}, {:06x}", car1.model, car1.color);
    println!("{}, {:06x}", car2.model, car2.color);
}