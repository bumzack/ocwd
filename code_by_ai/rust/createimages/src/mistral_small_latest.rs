pub fn mistral_small_latest() {
    let shapes = ["triangle", "square", "circle"];
    let colors = ["red", "blue", "green"];

    // let mut rng = rand::thread_rng();
    // for _ in 0..10 {
    //     // Generate 10 images for demonstration
    //     let shape = shapes.choose(&mut rng).unwrap();
    //     let color = colors.choose(&mut rng).unwrap();
    //     let mut img = ImageBuffer::new(64, 64);
    //     match shape {
    //        & "triangle" => draw_triangle(&mut img),
    //        & "square" => draw_square(&mut img),
    //        & "circle" => draw_circle(&mut img),
    //         _ => panic!("Unknown shape"),
    //     }
    //
    //     let filename = format!("{}_{}.png", shape, color);
    //     img.save(filename).unwrap();
    //
    //     // Write description and filename to CSV
    //     write_to_csv(shape, color, &filename);
    // }
}
//
// fn draw_triangle(img: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
//     let mut rng = rand::thread_rng();
//     let (x1, y1) = (rng.gen_range(0..64), rng.gen_range(0..64));
//     let (x2, y2) = (rng.gen_range(0..64), rng.gen_range(0..64));
//     let (x3, y3) = (rng.gen_range(0..64), rng.gen_range(0..64));
//
//     img.line((x1, y1), (x2, y2), image::Rgba([255, 0, 0, 255]));
//     img.line((x2, y2), (x3, y3), image::Rgba([255, 0, 0, 255]));
//     img.line((x3, y3), (x1, y1), image::Rgba([255, 0, 0, 255]));
// }
//
// fn draw_square(img: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
//     let mut rng = rand::thread_rng();
//     let (x1, y1) = (rng.gen_range(0..64), rng.gen_range(0..64));
//     let (x2, y2) = (rng.gen_range(0..64), rng.gen_range(0..64));
//
//     img.line((x1, y1), (x2, y1), image::Rgba([0, 0, 255, 255]));
//     img.line((x2, y1), (x2, y2), image::Rgba([0, 0, 255, 255]));
//     img.line((x2, y2), (x1, y2), image::Rgba([0, 0, 255, 255]));
//     img.line((x1, y2), (x1, y1), image::Rgba([0, 0, 255, 255]));
// }
//
// fn draw_circle(img: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
//     let mut rng = rand::thread_rng();
//     let (x, y) = (rng.gen_range(0..64), rng.gen_range(0..64));
//     let radius = 32;
//
//     for i in -radius..=radius {
//         for j in -radius..=radius {
//             if i * i + j * j <= radius * radius {
//                 img.put_pixel((x as i32 + i, y as i32 + j), image::Rgba([0, 255, 0, 255]));
//             }
//         }
//     }
// }
//
// fn write_to_csv(shape: &str, color: &str, filename: &str) {
//     let mut wtr = csv::Writer::from_writer(File::create("output.csv").unwrap());
//     wtr.write_record(&[shape, color, filename]).unwrap();
//     wtr.flush().unwrap();
// }
