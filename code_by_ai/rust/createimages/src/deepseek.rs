// deepseek-coder-v2:236b / 235.7B

use image::{Rgb, RgbImage};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use rand::thread_rng;

#[derive(Debug)]
enum Shape {
    Triangle,
    Square,
    Circle,
}

pub
fn deepseek() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let shapes = [Shape::Triangle, Shape::Square, Shape::Circle];
    let colors = ["red", "blue", "green"];

    let mut csv_writer = csv::Writer::from_path("image_descriptions.csv")?;
    csv_writer.write_record(&["filename", "description"])?;

    for i in 0..10 { // Generate 10 images as an example
        let shape = shapes.choose(&mut rng).unwrap();
        let color = colors.choose(&mut rng).unwrap();

        let filename = format!("image_{}.png", i);
        let description = match shape {
            Shape::Triangle => format!("A {} triangle", color),
            Shape::Square => format!("A {} square", color),
            Shape::Circle => format!("A {} circle", color),
        };

       //  draw_shape(&filename, *shape, color)?;

        csv_writer.write_record(&[&filename, &description])?;
    }

    csv_writer.flush()?;
    Ok(())
}

fn draw_shape(filename: &str, shape: Shape, color: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = RgbImage::new(64, 64);
    let rgb_color = match color {
        "red" => Rgb([255, 0, 0]),
        "blue" => Rgb([0, 0, 255]),
        "green" => Rgb([0, 255, 0]),
        _ => Rgb([0, 0, 0]), // Default to black if color is unknown
    };

    // match shape {
    //     Shape::Triangle => {
    //         for x in 16..48 {
    //             img.put_pixel(x, 32 + (x - 32).abs(), rgb_color);
    //         }
    //         for y in 16..48 {
    //             img.put_pixel(32 + (y - 32).abs(), y, rgb_color);
    //         }
    //     },
    //     Shape::Square => {
    //         for x in 16..48 {
    //             img.put_pixel(x, 16, rgb_color);
    //             img.put_pixel(x, 47, rgb_color);
    //         }
    //         for y in 16..48 {
    //             img.put_pixel(16, y, rgb_color);
    //             img.put_pixel(47, y, rgb_color);
    //         }
    //     },
    //     Shape::Circle => {
    //         let radius = 15;
    //         for x in 0..64 {
    //             for y in 0..64 {
    //                 if ((x as i32 - 32).pow(2) + (y as i32 - 32).pow(2)).sqrt() as i32 == radius {
    //                     img.put_pixel(x, y, rgb_color);
    //                 }
    //             }
    //         }
    //     },
    // }
    //
    // let mut file = File::create(filename)?;
    // img.write_to(&mut file, image::ImageFormat::Png)?;
    Ok(())
}

