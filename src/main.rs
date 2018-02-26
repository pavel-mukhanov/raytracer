//! An example of generating julia fractals.
extern crate num_complex;
extern crate image;
extern crate rand;

use std::fs::File;
use image::{ImageBuffer, Rgb};

use num_complex::Complex;

fn render(pixels: &mut Vec<Rgb<u8>>) {
    let max_iterations = 256u16;
    let mut rng = rand::thread_rng();

    for pixel in pixels.iter_mut() {
        *pixel = image::Rgb([0, 255, 0]);
    }
}

fn write_image(pixels:Vec<Rgb<u8>>, imgx:u32, imgy:u32) {
    let mut imgbuf = ImageBuffer::new(imgx as u32, imgy as u32);
 
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = x * imgx as u32 + y;
        *pixel = pixels[index as usize];
    }

    let fout = &mut File::create("render.png").unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}

fn main() {
    let imgx = 4;
    let imgy = 4;
    let mut pixels = vec![Rgb([0, 0, 0]); imgx * imgy];

    render(&mut pixels);
    write_image(pixels, imgx as u32, imgy as u32);
}
