static mut TAX: f32 = 0.1;

fn main() {
    unsafe {   // due to use of mutable staic
        println!("{}", TAX*300.0);
        TAX = 0.2;
        println!("{}", TAX*300.0);
    }

}