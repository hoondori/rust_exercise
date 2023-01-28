fn main() {
    let points:[i32;5] = [1,2,3,4,5];
    let points_slice = &points[0..3];

    print_array(&points);
    print_array(&points_slice);

    let sum = sum_slice(&points[..]);
    println!("sum={}", sum);
}

fn print_array(arr: &[i32]) {
    println!("{:?}", arr);
    println!("len={}", arr.len());
}

fn sum_slice(items: &[i32]) -> i32 {
    let mut total:i32 = 0;
    for i in items {
        total += i;
    }
    total
}