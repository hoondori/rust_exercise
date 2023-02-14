use num::Complex;
use text_colorizer::Colorize;
use std::str::FromStr;

/// c 가 망델로브 집합에 속하는지 검사, 속하지 않으면 Some(탈출시간), 맞으면 None 반환
/// heuristics
///     1. 제한된 시간내에만 
///     2. 반지름 2인 원을 벗어나면 대략 무한 발산으로 간주 
/// 
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re:0.0, im: 0.0 };

    for i in 0..limit {  // 제한 시간 내에만
        if z.norm_sqr() > 4.0 { // 발산  heuristic
            return Some(i);
        }
        z += z*z + c;
    }

    None
}

///'s' 가 구분자로 구분된 좌표쌍이면 이를 파싱한다. 
///   input 형식 : <left><seperator><right>
/// 
/// ex) 400x600 => (400, 600), 1.0,0.5 => (1.0, 0.5)
fn parse_pair<T:FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None, // 구분자가 없는 경우 파싱 불가
        Some(index) => {  // 구분자로 분리했어도 양쪽의 string to numeric 변환이 모두 성공해야 한다.
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

///가 쉼표로 구분된 complex 표현 's'를 부동소수점 복소수로 파싱한다.
///   input 형식 : <real><,><imaginary>
/// 
/// ex) 1.3,-0.4 => Complex(1.3, -0.4)
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

/// 결과 이미지가 주어지면 이에 대응하는 복소수 평면 위의 점을 반환
/// 
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64> {
    
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

/// 직시각형 모양의 망델브르 집합을 픽셀 버퍼에 렌더링한다. 
fn render(pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
    for column in 0..bounds.0 {
        let point = pixel_to_point(bounds, (column, row),
                                    upper_left, lower_right);
        pixels[row * bounds.0 + column] =
            match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
    }
    }
}

use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use std::fs::File;

/// 픽셀 버퍼를 이미지 파일로 저장한다. 
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {

    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8).expect("write_image error");

    Ok(())
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", 'c'), None);
    assert_eq!(parse_pair::<i32>("400x600", 'x'), Some((400,600)));
    assert_eq!(parse_pair::<f64>("1.5,2.3", ','), Some((1.5,2.3)));
    assert_eq!(parse_pair::<f64>("1.5,2.3", '.'), None);
}
#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("13.4,-.5"), Some(Complex{re:13.4, im:-0.5}));
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex { re:0.0, im: 0.0 }, 10), None);
    assert_eq!(escape_time(Complex { re:10.0, im: 0.0 }, 10), Some(1));
    assert_eq!(escape_time(Complex { re:0.3, im: 0.1 }, 10), Some(4));
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });    
}

use std::env;

fn print_usage() {
    eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", "mandelbrot".red());
    eprintln!("Example: {} mandel.png 1000x750 -1.2,0.3 -1,0.2", "mandelbrot".red());
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() !=5 {
        print_usage();
        std::process::exit(1);
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing UPPERLEFT");
    let lower_right = parse_complex(&args[4]).expect("error parsing LOWERRIGHT");
    
    // bounds 만큼 픽셀 버퍼 생성
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG FILE");    

}

