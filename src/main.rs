mod ga;
mod gwo;
mod hbes;
mod sca;
mod schedule;
mod solution;
mod ssa;
mod unit;
mod woa;

use ga::*;
use gwo::*;
use hbes::*;
use sca::*;
use schedule::*;
use ssa::*;
use std::fs;
use std::time::Instant;
use unit::*;
use woa::*;

fn main() {
    let now = Instant::now();

    let n_runs = 30;

    /* Datasets */
    // let (units, demand) = get_four_units();
    // let (units, demand) = get_ten_units();
    // let (units, demand) = get_ten_x_units(2);
    // let (units, demand) = get_ten_x_units(4);
    // let (units, demand) = get_ten_x_units(6);
    // let (units, demand) = get_ten_x_units(8);
    let (units, demand) = get_ten_x_units(10);

    let n_hours: usize = demand.len();
    let n_units: usize = units.len();
    let desc: Vec<usize> = get_sorted_unit_indices(&units, false);
    let asc: Vec<usize> = get_sorted_unit_indices(&units, true);
    let mut scores: Vec<f64> = Vec::with_capacity(n_runs);
    let mut best: Schedule = empty_schedule(n_hours, n_units);

    /* Bald Eagle Search */
    let lm: f64 = 2.0; /* Default: 2.0, ranges from 1.5 to 2.0 */
    let a: f64 = 10.0; /* Default: 10.0, ranges from 5.0 to 10.0 */
    let r: f64 = 2.0; /* Default: 1.5, ranges from 0.5 to 2.0 */
    let n_pop: usize = 80; /* Default: 100 */
    let max_iter: usize = 300; /* Default: 1000 */
    for i in 0..n_runs {
        let schedule: Schedule = hbes(
            lm, a, r, &units, &demand, &desc, &asc, n_hours, n_units, n_pop, max_iter,
        );
        println!("Run: {}, Best sol cost: {:.2}", i + 1, schedule.cost);
        // print_schedule(&schedule, n_hours, n_units, &units, &demand);
        scores.push(schedule.cost);
        if i == 0 {
            best = schedule;
        } else if schedule.cost < best.cost {
            best = schedule;
        }
    }

    // /* Grey Wolf Optimizer */
    // let n_pop: usize = 80; /* Default: 30 */
    // let max_iter: usize = 1000;
    // for i in 0..n_runs {
    //     let schedule: Schedule = gwo(
    //         &units, &demand, &desc, &asc, n_pop, max_iter, n_hours * n_units
    //     );
    //     println!("Run: {}, Best sol cost: {:.2}", i + 1, schedule.cost);
    //     // print_schedule(&best, n_hours, n_units, &units, &demand);
    //     scores.push(schedule.cost);
    //     if i == 0 {
    //         best = schedule;
    //     } else if schedule.cost < best.cost {
    //         best = schedule;
    //     }
    // }

    // /* Whale Optimization Algorithm */
    // let n_search_agents: usize = 80; /* Default: 30 */
    // let max_iter: usize = 1000;
    // for i in 0..n_runs {
    //     let schedule: Schedule = woa(
    //         &units, &demand, &desc, &asc, n_search_agents, max_iter, n_hours * n_units
    //     );
    //     println!("Run: {}, Best sol cost: {:.2}", i + 1, schedule.cost);
    //     // print_schedule(&best, n_hours, n_units, &units, &demand);
    //     scores.push(schedule.cost);
    //     if i == 0 {
    //         best = schedule;
    //     } else if schedule.cost < best.cost {
    //         best = schedule;
    //     }
    // }

    // /* Salp Swarm Algorithm */
    // let n_pop: usize = 80; /* Default: 30 */
    // let max_iter: usize = 1000;
    // for i in 0..n_runs {
    //     let schedule: Schedule = ssa(
    //         &units, &demand, &desc, &asc, n_pop, max_iter, n_hours * n_units
    //     );
    //     println!("Run: {}, Best sol cost: {:.2}", i + 1, schedule.cost);
    //     // print_schedule(&best, n_hours, n_units, &units, &demand);
    //     scores.push(schedule.cost);
    //     if i == 0 {
    //         best = schedule;
    //     } else if schedule.cost < best.cost {
    //         best = schedule;
    //     }
    // }

    // /* Sine Cosine Algorithm */
    // let n_search_agents: usize = 80; /* Default: 30 */
    // let max_iter: usize = 1000;
    // for i in 0..n_runs {
    //     let schedule: Schedule = sine_cosine(
    //         &units, &demand, &desc, &asc, n_search_agents, max_iter, n_hours * n_units
    //     );
    //     println!("Run: {}, Best sol cost: {:.2}", i + 1, schedule.cost);
    //     // print_schedule(&best, n_hours, n_units, &units, &demand);
    //     scores.push(schedule.cost);
    //     if i == 0 {
    //         best = schedule;
    //     } else if schedule.cost < best.cost {
    //         best = schedule;
    //     }
    // }

    // /* Genetic Algorithm */
    // let n_individuals = 80;
    // let n_generations = 1000;
    // let crossover_rate = 0.5;
    // let mutation_rate = 0.03;
    // let elitism_rate = 0.05;
    // let n_genes: usize = n_hours * n_units;
    // for i in 0..n_runs {
    //     let schedule: Schedule = genetic_algorithm(
    //         &units,
    //         &demand,
    //         &desc,
    //         &asc,
    //         n_individuals,
    //         n_genes,
    //         n_generations,
    //         crossover_rate,
    //         mutation_rate,
    //         elitism_rate
    //     );
    //     println!("Run: {}, Best sol cost: {}", i + 1, schedule.cost);
    //     // print_schedule(&best, n_hours, n_units, &units, &demand);
    //     scores.push(schedule.cost);
    //     if i == 0 {
    //         best = schedule;
    //     } else if schedule.cost < best.cost {
    //         best = schedule;
    //     }
    // }

    /* Results */
    let stats: [f64; 3] = mean_best_worst(&scores);
    println!(
        "Mean: {:.2}, Best: {:.2}, Worst: {:.2}",
        stats[0], stats[1], stats[2]
    );
    save_schedule(&best, n_hours, n_units, &demand);
    let mut stats_file = String::with_capacity(64);
    stats_file.push_str("./results/");
    stats_file.push_str(&n_units.to_string());
    stats_file.push_str("_stats.txt");
    let mut stats_data = String::with_capacity(8192);
    for i in 0..n_runs {
        stats_data.push_str(&((scores[i] * 100.0).trunc() / 100.0).to_string());
        stats_data.push('\n');
    }
    fs::write(stats_file, stats_data).expect("Unable to write file");

    /* Print time elapsed */
    println!("This took {:.2?}", now.elapsed());
}

fn mean_best_worst(x: &[f64]) -> [f64; 3] {
    let mut best: f64 = *x.first().unwrap();
    let mut worst: f64 = *x.last().unwrap();
    let mut total: f64 = 0.0;
    for i in x.iter() {
        total += i;
        if best > *i {
            best = *i;
        }
        if worst < *i {
            worst = *i;
        }
    }
    [total as f64 / x.len() as f64, best as f64, worst as f64]
}
