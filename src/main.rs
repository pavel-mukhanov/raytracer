extern crate image;
extern crate num_complex;
extern crate rand;

use std::fs::File;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rand::distributions::{IndependentSample, Normal};

use num_complex::Complex;

fn render(pixels: &mut Vec<Rgb<u8>>) {
    let max_iterations = 256u16;
    let between = Normal::new(1.0, 1.);
    let mut rng = rand::thread_rng();

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let b = between.ind_sample(&mut rng).abs();
        *pixel = image::Rgb([0, 127 + (b * 10f64) as u8, (i % 255) as u8]);
    }
}

fn write_image(pixels: Vec<Rgb<u8>>, imgx: u32, imgy: u32) {
    let mut imgbuf = ImageBuffer::new(imgx as u32, imgy as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = x * imgx as u32 + y;
        *pixel = pixels[index as usize];
    }

    let fout = &mut File::create("render.png").unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}

fn main() {
    let imgx = 400;
    let imgy = 400;
    let mut pixels = vec![Rgb([0, 0, 0]); imgx * imgy];

    render(&mut pixels);
    write_image(pixels, imgx as u32, imgy as u32);
}
