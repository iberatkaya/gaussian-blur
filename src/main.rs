extern crate image;
use image::open;
mod blur;
use blur::gaussian::gaussian_blur;

fn main() {
    let image = open("./images/pika.png").unwrap();
    gaussian_blur(image, 9, 5.0, "./images/blurredpikaf9s5.png");
}
