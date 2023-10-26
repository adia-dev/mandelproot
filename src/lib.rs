extern crate image;
extern crate num_complex;

fn compute_iterations_mandelbrot(complex_x: f32, complex_y: f32, max_iterations: usize) -> usize {
    let c = num_complex::Complex::new(complex_x, complex_y);
    let mut z = num_complex::Complex::new(0f32, 0f32);

    let mut nb_iterations = 0;
    // >= 2.0 we go out of bounds, useless to render
    while nb_iterations < max_iterations && z.norm() < 2.0 {
        z = z * z + c;
        nb_iterations += 1;
    }

    nb_iterations
}

pub fn compute_iterations(
    width: u32,
    height: u32,
    xa: f32,
    xb: f32,
    ya: f32,
    yb: f32,
    max_iterations: usize,
) -> Vec<usize> {
    (0..(width * height))
        .into_iter()
        .map(|offset| {
            let image_x = offset % width;
            let image_y = offset / width;

            let complex_x = (image_x as f32) * (xb - xa) / (width as f32 - 1.0f32) + xa;
            let complex_y = (image_y as f32) * (yb - ya) / (height as f32 - 1.0f32) + ya;

            compute_iterations_mandelbrot(complex_x, complex_y, max_iterations)
        })
        .collect()
}

pub fn save_image(
    nb_iterations: &[usize],
    width: u32,
    height: u32,
    _max_iterations: usize, // I don't need to use it for now
    path: &str,
) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let smooth = (nb_iterations[(y * width + x) as usize]) as f32;

        let red = (255.0 * smooth.sin()).abs() as u8;
        let green = (255.0 * (smooth * 0.5).sin()).abs() as u8;
        let blue = (255.0 * (smooth * 0.8).cos()).abs() as u8;

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(path).unwrap();
}
