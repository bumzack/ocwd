use image::{DynamicImage, GenericImageView, Rgba};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use rand::prelude::IndexedRandom;


pub
fn qwen25coder() -> io::Result<()> {
    let shapes = ["triangle", "square", "circle"];
    let colors = ["red", "blue", "green"];

    let mut rng = rand::thread_rng();

    let mut writer = csv::Writer::from_path("image_descriptions.csv")?;

    for i in 0..10 { // Generate 10 images as an example
        let shape = *shapes.choose(&mut rng).unwrap();
        let color = *colors.choose(&mut rng).unwrap();

        let img = create_image(shape, color);

        let filename = format!("image_{}_{}.png", shape, color);
        img.save(&filename).expect("file save error");

        writer.write_record(&[&filename, &format!("A 64x64 pixel image containing a {} outline in {}", shape, color)])?;
    }

    writer.flush()?;
    Ok(())
}

fn create_image(shape: &str, color: &str) -> DynamicImage {
    let mut img = image::DynamicImage::new_rgba8(64, 64).to_rgba8();

    let draw_color = match color {
        "red" => Rgba([255, 0, 0, 255]),
        "blue" => Rgba([0, 0, 255, 255]),
        "green" => Rgba([0, 255, 0, 255]),
        _ => panic!("Unknown color"),
    };

    match shape {
        "triangle" => draw_triangle(&mut img, draw_color),
        "square" => draw_square(&mut img, draw_color),
        "circle" => draw_circle(&mut img, draw_color),
        _ => panic!("Unknown shape"),
    }

    DynamicImage::ImageRgba8(img)
}

fn draw_triangle(img: &mut image::RgbaImage, color: Rgba<u8>) {
    let points = [(10, 54), (32, 10), (54, 54)];
    for y in 0..img.height() {
        for x in 0..img.width() {
            if is_point_in_triangle((x as i32, y as i32), points) {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn draw_square(img: &mut image::RgbaImage, color: Rgba<u8>) {
    for y in 10..54 {
        for x in 10..54 {
            img.put_pixel(x, y, color);
        }
    }
}

fn draw_circle(img: &mut image::RgbaImage, color: Rgba<u8>) {
    let center = (32, 32);
    let radius = 20;
    for y in 0..img.height() {
        for x in 0..img.width() {
            if is_point_in_circle((x as i32, y as i32), center, radius) {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn is_point_in_triangle(point: (i32, i32), triangle: [(i32, i32); 3]) -> bool {
    let (x, y) = point;
    let (ax, ay) = triangle[0];
    let (bx, by) = triangle[1];
    let (cx, cy) = triangle[2];

    let area = |(x1, y1), (x2, y2), (x3, y3)| -> i32 {
        x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
    };

    let area_main = area((ax, ay), (bx, by), (cx, cy)).abs();
    let area1 = area((x, y), (bx, by), (cx, cy)).abs();
    let area2 = area((ax, ay), (x, y), (cx, cy)).abs();
    let area3 = area((ax, ay), (bx, by), (x, y)).abs();

    area_main == area1 + area2 + area3
}

fn is_point_in_circle(point: (i32, i32), center: (i32, i32), radius: i32) -> bool {
    let (x, y) = point;
    let (cx, cy) = center;
    ((x - cx).pow(2) + (y - cy).pow(2)) <= radius.pow(2)
}