use rand::Rng;


pub fn find_points(min: f64, max: f64,  check: fn(f64,f64)->bool) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut outside_points = vec![];
    let mut inside_points = vec![];
    for _ in 0..1_000_000 {
        let x = rand::thread_rng().gen_range(min..=max);
        let y = rand::thread_rng().gen_range(min..=max);
        if check(x,y) {
            inside_points.push((x as f64, y as f64));
        } else {
            outside_points.push((x as f64, y as f64));
        }
    }
    (outside_points, inside_points)
}
