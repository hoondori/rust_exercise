use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    
    let in_file = "original.jpg".to_string();
    let filename: Vec<&str> = in_file.split(".").collect();
    let out_file = format!("{}-out.jpg", filename[0]);

    let mut img = image::open(in_file).expect("no file");

    let (w, h) = img.dimensions();

    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);

            // 색상 반전: alpha만 그대로
            let c = Rgba([255-c[0], 255-c[1], 255-c[2], c[3]]);
            img.put_pixel(x, y, c);
        }
    }

    img.save(out_file).unwrap();
}
