mod backgrounds;
mod shapes;
mod generator;

use image::{ImageBuffer, Rgb, RgbImage};
use simple_logger::SimpleLogger;
use log::debug;

fn main() {
    SimpleLogger::new().init().unwrap();
    debug!("Starting...");

    let mut image = RgbImage::new(32, 32);

    // set a central pixel to white
    for i in 1..17 {
        for j in 1..3 {
            image.put_pixel(i, j, Rgb([255, 255, 255]));
        }
    }

    image = image::imageops::rotate90(&image);

    image.save("output.png").unwrap();
}