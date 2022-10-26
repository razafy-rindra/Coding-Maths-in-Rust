use monte_carlo_integration::find_points;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "monte_carlo_integration.png";

fn mandelbrot(x: f64, y: f64) -> bool {
    let c = num_complex::Complex::new(x, y);
    let mut z = num_complex::Complex::new(0 as f64, 0 as f64);
    for _ in 0..255 {
        z = z * z + c;
        if z.norm() > 2.0 {
            return false;
        }
    }
    true
}

fn circle(x: f64, y: f64) -> bool {
    x * x + y * y <= 1.0
}

fn triangle(x: f64, y: f64) -> bool {
    let coeff = 3.0_f64.sqrt();
    y <= coeff * x && y <= coeff * (1.0 - x)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() > 2 {
        print_help();
        std::process::exit(-1);
    }
    let subcommand = args.remove(0);
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (points, min, max, message) = match subcommand.as_str() {
        "p" => (find_points(-1.0, 1.0, circle), -1.0, 1.0, "Pi "),
        "m" => (
            find_points(-2.0, 2.0,mandelbrot),-2.0,2.0,
            "Area of the Mandelbrot set ",
        ),
        "t" => (find_points(0.0, 1.0, triangle),0.0,1.0, "Area of the Triange "),
        _ => {
            print_help();
            std::process::exit(-1);
        }
    };

    let outside_points = points.0;
    let inside_points = points.1;
    let area = (max-min)*(max-min)
        * ((inside_points.len() as f64) / ((outside_points.len() + inside_points.len()) as f64));
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} may be {}", message, area), ("sans-serif", 25))
        .build_cartesian_2d((min-0.5)..(max+0.5), (min-0.5)..(max+0.5))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    chart.draw_series(
        inside_points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
    )?;

    chart.draw_series(
        outside_points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, RED.filled())),
    )?;
    root.present()?;
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

fn print_help() {
    println!("I will use Monte Carlo Integration to approximately calculate the following: ");
    println!("Press p for π");
    println!("Press m for the area of the Mandelbrot set");
    println!("Press t for the area of the equilateral triangle of side length 1");
}
