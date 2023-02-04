use image;

fn main() {
 
    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 0, 0]);

    let w = 64;

    let draw = |x, y| {
        let (xi, yi) = (x/w, y/w);
        match (xi % 2, yi % 2) {
            (0, 0) | (1, 1) => white,
            (1, 0) | (0, 1) => red,
            (_, _) => panic!("error"),
        }
    };

    let img = image::ImageBuffer::from_fn(512, 512, draw);
    img.save("checkerboard.png").unwrap();
}
