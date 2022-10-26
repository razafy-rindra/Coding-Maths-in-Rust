use burning_ship::Complex;

fn main() {
    ship(String::from("burning-ship.png"));
}

// Creating the fractal
fn ship(outfile: String) {
    let width = 500;
    let height = 938;
    let min_x = -1.8;
    let max_x = -1.7;
    let min_y = -(1.0 / 8.0);
    let max_y = 1.0 / 16.0;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Doing this to scale down and translate (x,y) so that we can zoom in on the ship in the
        // left antenna of the fractal.

        let scale_x = (max_x - min_x) / width as f32;
        let scale_y = (max_y - min_y) / height as f32;

        let x = (x as f32) * scale_x + min_x;
        let y = (y as f32) * scale_y + min_y;

        let c = Complex::new(x, y);
        let mut z = Complex::new(0.0, 0.0);
        let mut shade: u32 = 1000;
        while 0 < shade && z.norm() <= 4.0 {
            shade -= 1;
            let zn = Complex::new(z.re().abs(), z.im().abs());
            z = zn * zn + c;
        }
        let red = 0;
        // Adding some green for esthetic reasons.
        let green = match shade {
            0 => 0,
            _ => 50 as u8,
        };
        // The shade of blue should change based on how fast our point goes to infinity
        let blue = (shade * (1000 / 255)) as u8;

        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(outfile).unwrap();
}
