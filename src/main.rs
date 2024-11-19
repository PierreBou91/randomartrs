// use num::Num;

fn main() {
    let width = 800;
    let height = 800;

    let mut buf = image::ImageBuffer::new(width, height);

    for (x, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = compute_pixel(x as f32, 0 as f32, width as f32, height as f32);
    }

    buf.save("image.png").unwrap();
}

fn rescale_value(value: f32, min: f32, max: f32, new_min: f32, new_max: f32) -> f32 {
    (value - min) / (max - min) * (new_max - new_min) + new_min
}

fn compute_pixel(x: f32, y: f32, width: f32, height: f32) -> image::Rgb<u8> {
    let r = rescale_value(x, 0 as f32, width, 0 as f32, 255 as f32);
    let g = rescale_value(y, 0 as f32, height, 0 as f32, 255 as f32);
    let b = rescale_value(x, 0 as f32, width, 0 as f32, 255 as f32);
    image::Rgb([r as u8, g as u8, b as u8])
}

#[allow(dead_code)]
fn generate_fractal() {
    //! An example of generating julia fractals.
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

#[allow(dead_code)]
fn image_from_raw_buffer() {
    let buffer: &[u8] = &[125u8; 1440000];
    image::save_buffer(
        "image.png",
        buffer,
        800,
        600,
        image::ExtendedColorType::Rgb8,
    )
    .unwrap();
}
