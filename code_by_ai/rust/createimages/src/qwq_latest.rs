// // qwq latest
//
// use raqote::{DrawTarget, DrawOptions, PathBuilder, Source, AntialiasMode};
//
// use image::{ImageBuffer, RgbaImage};
//
// use rand::prelude::*;
//
// use csv::{Writer, StringRecord};
// use rand::thread_rng;
//
// enum Shape {
//
//     Triangle,
//
//     Square,
//
//     Circle,
//
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//
//     let width = 100;
//
//     let height = 100;
//
//     let shapes = [Shape::Triangle, Shape::Square, Shape::Circle];
//
//     let color_names = ["red", "blue", "green"];
//
//     let colors = [
//
//         Srr::new(255, 0, 0, 255),
//
//         SrrgbaU8::new(0, 0, 255, 255),
//
//         SrrgbaU8::new(0, 255, 0, 255),
//
//     ];
//
//     let mut rng = thread_rng();
//
//     let num_images = 10; // or any desired number
//
//     let mut wtr = Writer::from_path("output.csv")?;
//
//     wtr.write_header(vec!["description", "filename"])?;
//
//     for i in 1..=num_images {
//
//         let random_shape = shapes.choose(&mut rng).unwrap();
//
//         let random_color = colors.choose(&mut rng).unwrap();
//
//         let color_name = color_names[colors.iter().position(|c| c == random_color).unwrap()];
//
//         let filename = format!("shape_{:03}.png", i);
//
//         let mut dt = DrawTarget::new(width as i32, height as i32);
//
//         let mut options = DrawOptions::new();
//
//         options.antialias = AntialiasMode::Gray;
//
//         options.fill_source = Source::Solid(*random_color);
//
//         match random_shape {
//
//             Shape::Triangle => draw_triangle(&mut dt, &options),
//
//             Shape::Square => draw_square(&mut dt, &options),
//
//             Shape::Circle => draw_circle(&mut dt, &options),
//
//         }
//
//         let surface_pixels = dt.read_pixels(0, 0, width as i32, height as i32);
//
//         let rgba_data: Vec<u8> = surface_pixels.iter().flat_map(|&pixel| {
//
//             let bytes = pixel.to_be_bytes();
//
//             vec![bytes[0], bytes[1], bytes[2], bytes[3]]
//
//         }).collect();
//
//         let img = ImageBuffer::from_raw(width, height, rgba_data).unwrap();
//
//         img.save(&filename)?;
//
//         wtr.write(Record::from(vec![color_name.to_string() + " " + random_shape_variant_name(random_shape), filename]))?;
//
//     }
//
//     Ok(())
//
// }
//
// fn draw_triangle(dt: &mut DrawTarget, options: &DrawOptions) {
//
//     let mut pb = PathBuilder::new();
//
//     pb.move_to(50.0, 10.0);
//
//     pb.line_to(10.0, 90.0);
//
//     pb.line_to(90.0, 90.0);
//
//     pb.close();
//
//     dt.fill_path(&pb.finish(), options, &DrawOptions::new());
//
// }
//
// fn draw_square(dt: &mut DrawTarget, options: &DrawOptions) {
//
//     let rect = raqote::Rect::from_xywh(25.0, 25.0, 50.0, 50.0);
//
//     dt.fill_rect(rect, options, &DrawOptions::new());
//
// }
//
// fn draw_circle(dt: &mut DrawTarget, options: &DrawOptions) {
//
//     let center_x = 50.0;
//
//     let center_y = 50.0;
//
//     let radius = 40.0;
//
//     let mut pb = PathBuilder::new();
//
//     pb.arc(center_x, center_y, radius, 0.0, 2.0 * std::f32::consts::PI, false);
//
//     dt.fill_path(&pb.finish(), options, &DrawOptions::new());
//
// }
//
// fn random_shape_variant_name(shape: &Shape) -> &'static str {
//
//     match shape {
//
//         Shape::Triangle => "triangle",
//
//         Shape::Square => "square",
//
//         Shape::Circle => "circle",
//
//     }
//
// }
