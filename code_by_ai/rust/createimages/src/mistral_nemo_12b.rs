use image::Rgba;
use image::{ImageBuffer, RgbaImage};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufWriter, Write};
const SHAPES: [&str; 3] = ["triangle", "square", "circle"];
const COLORS: [(&str, u8); 3] = [("red", 255), ("blue", 0), ("green", 127)];

// not really triangles
// not really outlines, but fully filled

pub fn mistral_nemo_12b() {
    let mut rng = thread_rng();
    let mut csv_file = BufWriter::new(File::create("images.csv").unwrap());

    for i in 0..100 {
        // Generate 100 images
        let shape = SHAPES[rng.gen_range(0..3)];
        let color = COLORS[rng.gen_range(0..3)];

        let mut imgbuf = ImageBuffer::new(64, 64);

        match shape {
            "triangle" => draw_triangle(&mut imgbuf, color.1),
            "square" => draw_square(&mut imgbuf, color.1),
            "circle" => draw_circle(&mut imgbuf, color.1),
            _ => unreachable!(),
        }

        let filename = format!("image_{}.png", i);
        imgbuf.save(&filename).unwrap();

        writeln!(csv_file, "{},{}", shape, color.0,).unwrap();
    }
}

fn draw_triangle(img: &mut RgbaImage, color: u8) {
    img.put_pixel(32, 16, Rgba([color, 0, 0, 255]));
    img.put_pixel(48, 16, Rgba([color, 0, 0, 255]));
    img.put_pixel(40, 48, Rgba([color, 0, 0, 255]));
}

fn draw_square(img: &mut RgbaImage, color: u8) {
    for y in 16..49 {
        for x in 16..49 {
            img.put_pixel(x, y, Rgba([color, 0, 0, 255]));
        }
    }
}

fn draw_circle(img: &mut RgbaImage, color: u8) {
    for y in 0..64 {
        for x in 0..64 {
            let x_square:i32 = (x - 32) * (x - 32);
            let y_square:i32 = (y - 32) * (y - 32);
            if x_square + y_square <= 576 {
                img.put_pixel(x as u32, y as u32, Rgba([color, 0, 0, 255]));
            }
        }
    }
}
