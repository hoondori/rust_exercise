
struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str, // 판정
}

fn main() {
    let height_cm:f64 = 180.0;
    let weight:f64 = 75.0;
    let height:f64 = height_cm / 100.0;
    let bmi:f64 = weight / height.powf(2.0);

    let bmi_list = vec![ 
        BmiRange{ min:0.0, max:18.5, label:"저체중" }, 
        BmiRange{ min:18.5, max:23.0, label:"정상" } , 
        BmiRange{ min:23.0, max:25.0, label:"비만전단계" }, 
        BmiRange{ min:25.0, max:30.0, label:"비만1단계" }, 
        BmiRange{ min:30.0, max:35.0, label:"비만2단계" }, 
        BmiRange{ min:35.0, max:99.0, label:"비만3단계" }, 
    ];

    // 비만도 판단
    let mut result = "계산 불가";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }

    println!("BMI={:.1}, 비만도={}", bmi, result);
}

