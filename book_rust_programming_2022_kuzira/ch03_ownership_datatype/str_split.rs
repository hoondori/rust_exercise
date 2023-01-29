fn main() {
    let telno = "955-3658";

    // 슬라이스로 분할
    println!("{} {}", &telno[..3], &telno[4..]);

    // split_at으로 분할
    let (telno1, telno2) = telno.split_at(3);   // 955 -3658
    let (_, telno3) = telno2.split_at(1);  // - 3658
    println!("{} {}", &telno1, &telno3);

    // split_off 로 분할
    let mut telno1 = String::from(telno);
    let mut telno2 = telno1.split_off(3); // 955 -3658
    let telno3 = telno2.split_off(1); // - 3658
    println!("{} {}", &telno1, &telno3);

    // split으로 분한
    let telno_arr: Vec<&str> = telno.split("-").collect();
    println!("{} {}", &telno_arr[0], &telno_arr[1]);
}