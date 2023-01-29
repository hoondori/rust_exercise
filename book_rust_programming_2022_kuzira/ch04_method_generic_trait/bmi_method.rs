struct Body {
    height: f64, // 키, cm
    weight: f64, // 몸무게, kg
}

impl Body {

    // default constructor
    fn new(height: f64, weight: f64) -> Self {
        Body{ height: height, weight: weight }
    }

    // another constructor : for "weight,height"
    fn new2(expr: &str) -> Self {

        let arr: Vec<&str> = expr.split(",").collect();
        let height = arr[0].to_string().parse::<f64>().unwrap();
        let weight = arr[1].to_string().parse::<f64>().unwrap();

        Body{ height: height, weight: weight }
    }

    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

fn main() {
    let a = Body::new(160.0, 70.0);
    println!("BMI = {:.2}", a.calc_bmi());
    println!("비만율 = {:.1}%", a.calc_per());

    let b = Body::new2("130.0,90.0");
    println!("BMI = {:.2}", b.calc_bmi());
    println!("비만율 = {:.1}%", b.calc_per());
}
