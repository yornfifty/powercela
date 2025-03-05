use std::collections::{BTreeMap, HashSet};
use std::env;
use std::time::Instant;
use std::fs;
use serde::{Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Serialize)]
struct SimulationResult {
    generations: BTreeMap<usize, usize>,
    #[serde(rename = "stabilizedAt")]
    stabilized_at: Option<usize>,
}

struct GameOfLife {
    live_cells: HashSet<Point>,
    neighbor_offsets: [(i32, i32); 8],
    generation_data: BTreeMap<usize, usize>,
}

impl GameOfLife {
    fn new() -> Self {
        GameOfLife {
            live_cells: HashSet::new(),
            neighbor_offsets: [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1),
            ],
            generation_data: BTreeMap::new(),
        }
    }

    fn initialize_pattern(&mut self, pattern: &[&str], start_x: i32, start_y: i32) {
        for (y, row) in pattern.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '1' {
                    self.live_cells.insert(Point::new(start_x + x as i32, start_y + y as i32));
                }
            }
        }
    }

    fn simulate(&mut self, iterations: usize) -> Option<usize> {
        self.generation_data.insert(0, self.live_cells.len());
        const STABILITY_WINDOW: usize = 50;

        for step in 0..iterations {
            let mut neighbor_counts = std::collections::HashMap::with_capacity(self.live_cells.len() * 8);

            for &cell in &self.live_cells {
                for &(dx, dy) in &self.neighbor_offsets {
                    let neighbor = Point::new(cell.x + dx, cell.y + dy);
                    *neighbor_counts.entry(neighbor).or_insert(0) += 1;
                }
            }

            let mut new_live_cells = HashSet::new();
            for (cell, count) in neighbor_counts {
                if count == 3 || (count == 2 && self.live_cells.contains(&cell)) {
                    new_live_cells.insert(cell);
                }
            }

            self.live_cells = new_live_cells;

            let population = self.live_cells.len();
            let current_gen = step + 1;
            self.generation_data.insert(current_gen, population);

            if current_gen >= STABILITY_WINDOW {
                let start_gen = current_gen - STABILITY_WINDOW + 1;
                let last_50 = self.generation_data.range(start_gen..=current_gen);
                let is_stable = last_50.clone().all(|(_, &pop)| pop == population);

                if is_stable {
                    println!(
                        "Simulation stopped at generation {}: Population stabilized at {} for 50 generations",
                        current_gen-STABILITY_WINDOW+1, population
                    );
                    return Some(current_gen-STABILITY_WINDOW+1);
                }
            }

            if current_gen % 1000 == 0 {
                println!("Generation {}: Population = {}", current_gen, population);
            }
        }

        println!("Simulation completed all {} iterations without stabilizing", iterations);
        None
    }
}

fn split_pattern(pattern: &str, split_amount: usize) -> Vec<String> {
    pattern
        .chars()
        .collect::<Vec<char>>()
        .chunks(split_amount)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <iterations> <split_amount> <pattern_to_split>");
        eprintln!("Example: cellula.exe 10000 9 011000000100100000110000000111010000000000100110000111011100110001111100000011000");
        return;
    }

    let iterations: usize = args[1].parse().expect("Invalid iterations");
    let split_amount: usize = args[2].parse().expect("Invalid split amount");
    let pattern_to_split = &args[3];

    let pattern = split_pattern(pattern_to_split, split_amount);

    println!("Running Conway's Game of Life in Rust");
    println!("Iterations: {}", iterations);
    println!("Split Amount: {}", split_amount);
    println!("Pattern: {:?}", pattern);

    let mut game = GameOfLife::new();
    let start_x = -(split_amount as i32) / 2;
    let start_y = -(pattern.len() as i32) / 2;

    println!("Setting up pattern centered at ({}, {})", start_x, start_y);
    game.initialize_pattern(&pattern.iter().map(|s| s.as_str()).collect::<Vec<_>>(), start_x, start_y);

    println!("Simulating");
    let start_time = Instant::now();
    let stabilized_at = game.simulate(iterations);
    let duration = start_time.elapsed();

    println!("Simulation completed in {:?}", duration);

    fs::create_dir_all("result").unwrap();

    let result = SimulationResult {
        generations: game.generation_data,
        stabilized_at,
    };

    let json_data = serde_json::to_string_pretty(&result).unwrap();
    let output_file = format!("result/{}.json", args.last().unwrap());
    fs::write(&output_file, json_data).unwrap();
    println!("Generation data written to {}", output_file);
}