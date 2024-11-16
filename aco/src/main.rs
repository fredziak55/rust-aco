use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use rand::Rng;
use rayon::prelude::*;

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

struct ACO {
    alpha: f64,
    beta: f64,
    evaporation_rate: f64,
    pheromone: Vec<Vec<f64>>,
    distances: Vec<Vec<f64>>,
}

impl ACO {
    fn new(num_points: usize, alpha: f64, beta: f64, evaporation_rate: f64) -> Self {
        let pheromone = vec![vec![1.0; num_points]; num_points];
        let distances = vec![vec![0.0; num_points]; num_points];
        ACO {
            alpha,
            beta,
            evaporation_rate,
            pheromone,
            distances,
        }
    }

    fn initialize_distances(&mut self, points: &[(f64, f64)]) {
        for i in 0..points.len() {
            for j in 0..points.len() {
                self.distances[i][j] = distance(points[i], points[j]);
            }
        }
    }

    fn update_pheromones(&mut self, paths: &[(Vec<usize>, f64)]) {
        for i in 0..self.pheromone.len() {
            for j in 0..self.pheromone.len() {
                self.pheromone[i][j] *= 1.0 - self.evaporation_rate;
            }
        }

        for (path, path_distance) in paths {
            for k in 0..path.len() - 1 {
                let i = path[k];
                let j = path[k + 1];
                self.pheromone[i][j] += 1.0 / path_distance;
                self.pheromone[j][i] += 1.0 / path_distance;
            }
        }
    }

    fn construct_solution(&self, points: &[(f64, f64)]) -> (Vec<usize>, f64) {
        let mut rng = rand::thread_rng();
        let mut visited = vec![false; points.len()];
        let mut path = Vec::with_capacity(points.len());
        let mut total_distance = 0.0;

        let mut current_point = rng.gen_range(0..points.len());
        path.push(current_point);
        visited[current_point] = true;

        for _ in 1..points.len() {
            let mut probabilities = vec![0.0; points.len()];
            let mut sum_probabilities = 0.0;

            for j in 0..points.len() {
                if !visited[j] {
                    probabilities[j] = self.pheromone[current_point][j].powf(self.alpha)
                        * (1.0_f64 / self.distances[current_point][j]).powf(self.beta);
                    sum_probabilities += probabilities[j];
                }
            }

            let mut cumulative_probability = 0.0;
            let random_value = rng.gen::<f64>() * sum_probabilities;
            let mut next_point = 0;

            for j in 0..points.len() {
                if !visited[j] {
                    cumulative_probability += probabilities[j];
                    if cumulative_probability >= random_value {
                        next_point = j;
                        break;
                    }
                }
            }

            visited[next_point] = true;
            path.push(next_point);
            total_distance += self.distances[current_point][next_point];
            current_point = next_point;
        }

        total_distance += self.distances[current_point][path[0]];
        path.push(path[0]);

        (path, total_distance)
    }
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let points = read_points_from_file("../data/dane.txt")?;
    let num_points = points.len();
    let mut aco = ACO::new(num_points, 1.0, 5.0, 0.5);
    aco.initialize_distances(&points);

    let num_ants = 10;
    let num_iterations = 50; // Reduced number of iterations
    let mut best_path = Vec::new();
    let mut best_distance = f64::INFINITY;

    for _ in 0..num_iterations {
        let paths: Vec<(Vec<usize>, f64)> = (0..num_ants).into_par_iter().map(|_| {
            aco.construct_solution(&points)
        }).collect();

        for (path, distance) in &paths {
            if *distance < best_distance {
                best_distance = *distance;
                best_path = path.clone();
            }
        }

        aco.update_pheromones(&paths);
    }

    let duration = start_time.elapsed();

    println!("Best path: {:?}", best_path);
    println!("Best distance: {}", best_distance);
    println!("Execution time: {:?}", duration);

    Ok(())
}