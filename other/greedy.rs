use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_points_from_file(file_path: &str) -> io::Result<Vec<(f64, f64)>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let num_points: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut points = Vec::with_capacity(num_points);

    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x: f64 = parts[1].parse().unwrap();
        let y: f64 = parts[2].parse().unwrap();
        points.push((x, y));
    }

    Ok(points)
}

fn distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn shortest_path(points: &[(f64, f64)]) -> (Vec<usize>, f64) {
    let n = points.len();
    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);
    let mut total_distance = 0.0;

    let mut current_point = 0;
    path.push(current_point);
    visited[current_point] = true;

    for _ in 0..n - 1 {
        let mut nearest_point = None;
        let mut min_distance = f64::INFINITY;

        for i in 0..n {
            if !visited[i] {
                let dist = distance(points[current_point], points[i]);
                if dist < min_distance {
                    min_distance = dist;
                    nearest_point = Some(i);
                }
            }
        }

        if let Some(nearest) = nearest_point {
            visited[nearest] = true;
            path.push(nearest);
            total_distance += min_distance;
            current_point = nearest;
        }
    }

    // Return to the starting point
    total_distance += distance(points[current_point], points[0]);
    path.push(0);

    (path, total_distance)
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let points = read_points_from_file("../data/dane.txt.txt")?;
    let (path, total_distance) = shortest_path(&points);

    let duration = start_time.elapsed();

    println!("Path: {:?}", path);
    println!("Total distance: {}", total_distance);
    println!("Execution time: {:?}", duration);

    Ok(())
}