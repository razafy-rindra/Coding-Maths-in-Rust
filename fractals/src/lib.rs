use rand::Rng;


// Creating the mandelbrot fractal
pub fn mandelbrot(outfile: String) {
    let width = 1000;
    let height = 1000;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Doing this to scale down and translate (x,y)
        //        let test = (x % 20 == 0) && (y % 20 == 0);
        let scale_x = 4.0 / width as f32;
        let scale_y = 4.0 / height as f32;
        let x = (x as f32) * scale_y - 2.0;
        let y = (y as f32) * scale_x - 2.0;
        // Should replace this with my own Complex number struct.
        let c = num_complex::Complex::new(x, y);
        let mut z = num_complex::Complex::new(0 as f32, 0 as f32);
        let mut shade: u32 = 255;
        // Algorithm to see if the point is in the Mandelbrot set or not.
        while 0 < shade && z.norm() <= 2.0 {
            shade -= 1;
            z = z * z + c;
        }
        let red = (shade * 10 % 255) as u8;
        let green = 0;
        let blue = 0;

        *pixel = image::Rgb([red, green, blue]);
    }
    println!("Mandelbrot completed!");
    imgbuf.save(outfile).unwrap();
}

// Creating the Sierpinski triangle, with the Chaos Game
pub fn triangle(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    
    // Vector containing all our visited points.
    let mut visited = vec![];
    
    // Creating the three vertices of our triangle
    let a = (1_000, 1_000);
    let b = (0, 1_000);
    let c = (1_000, 0);

    // Creating starting point:
    let start_x = rand::thread_rng().gen_range(0..1_000);
    let start_y = rand::thread_rng().gen_range(0..1_000);

    // Adding all these starting points visited
    visited.push(a);
    visited.push(b);
    visited.push(c);
    visited.push((start_x,start_y));
    
    // Finding our points.
    
    let mut current = (start_x,start_y);
    for _ in 0..100_000{
        let d = rand::thread_rng().gen_range(0..3);
        current = match d%3 {
            0 => ((current.0+a.0)/2, (current.1+a.1)/2),
            1 =>((current.0+b.0)/2, (current.1+b.1)/2),
            _ =>((current.0+c.0)/2, (current.1+c.1)/2),
        };
        visited.push(current);
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Scaling everything by 2 may be unecessary. Will look more into this.
        let x = 2*x;
        let y = 2*y;


        if visited.contains(&(x,y)){
            *pixel = image::Rgb([255,255,255]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }
    println!("Chaos Game completed!");
    imgbuf.save(outfile).unwrap();
}
