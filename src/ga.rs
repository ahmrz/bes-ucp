use rand::Rng;
use crate::unit::Unit;
use crate::schedule::*;


pub fn genetic_algorithm(
    units: &[Unit],
    demand: &[f64],
    desc: &[usize],
    asc: &[usize],
    n_individuals: usize,
    n_genes: usize,
    n_generations: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    elitism_rate: f64
) -> Schedule {

    let n_hours: usize = demand.len();
    let n_units: usize = units.len();

    /* Generate population */
    let mut population: Vec<Schedule> = Vec::with_capacity(n_individuals);
    for _ in 0..n_individuals {
        let mut status: Vec<bool> = Vec::with_capacity(n_hours * n_units);
        for _ in 0..(n_hours * n_units) {
            status.push(rand::random::<bool>());
        }
        repair(&mut status, n_hours, n_units, units, demand, desc, asc);
        let power: Vec<f64> = lambda_iteration(&status, units, demand, n_hours, n_units);
        let (fuel, startup) = calculate_cost(&power, n_hours, n_units, units);
        let cost: f64 = sum_vec(&fuel) + sum_vec(&startup);

        population.push(
            Schedule {
                status,
                power,
                fuel,
                startup,
                cost
            }
        );
    }
    population.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());

    for i in 1..n_generations + 1 {
        let mut new_population: Vec<Schedule> = recombination(&mut population, n_individuals, n_genes, crossover_rate);
        mutation(&mut new_population, n_individuals, n_genes, mutation_rate);
        update_cost(&mut new_population, n_individuals, units, demand, n_hours, n_units, desc, asc);
        elitism(&mut population, &new_population, n_individuals, elitism_rate);
        update_cost(&mut population, n_individuals, units, demand, n_hours, n_units, desc, asc);
        // println!("Iteration {} Best {}", i, population[0].cost);
    }
    return population.first().unwrap().clone();
    
}

fn roulette_selection(
    p: &[Schedule]
) -> Vec<Schedule> {

    let mut skip: i64 = -1;
    let mut selected: Vec<Schedule> = Vec::with_capacity(2);
    for i in 0..2 {
        let mut tf: f64 = 0.0;
        for j in 0..p.len() {
            if j as i64 == skip {
                continue;
            }
            tf += p[j].cost;
        }
        let mut cf: f64 = 0.0;
        let r: f64 = rand::random();
        for j in 0..p.len() {
            if j as i64 == skip {
                continue;
            }
            if tf == 0.0 {
                cf += 1.0 / (p.len() - i) as f64
            } else {
                cf += p[j].cost as f64 / tf as f64;
            }
            if r < cf {
                skip = j as i64;
                selected.push(p[j].clone());
                break;
            }
        }
    }
    return selected;
}

// fn single_point_crossover(
//     selected: &mut [Schedule],
//     n_genes: usize
// ) {

//     let crossover_point: usize = rand::thread_rng().gen_range(1..n_genes);
//     for i in 0..crossover_point {
//         let temp: bool = selected[0].status[i];
//         selected[0].status[i] = selected[1].status[i];
//         selected[1].status[i] = temp;
//     }
// }

fn multi_point_crossover(
    selected: &mut [Schedule],
    n_genes: usize
) {

    let crossover_point1: usize = rand::thread_rng().gen_range(1..n_genes - 1);
    let crossover_point2: usize = rand::thread_rng().gen_range(crossover_point1 + 1..n_genes);
    for i in 0..crossover_point1 {
        let temp: bool = selected[0].status[i];
        selected[0].status[i] = selected[1].status[i];
        selected[1].status[i] = temp;
    }
    for i in crossover_point1..crossover_point2 {
        let temp: bool = selected[1].status[i];
        selected[1].status[i] = selected[0].status[i];
        selected[0].status[i] = temp;
    }
}

fn recombination(
    p: &mut [Schedule],
    n_individuals: usize,
    n_genes: usize,
    crossover_rate: f64
) -> Vec<Schedule> {

    let worst: f64 = p.last().unwrap().cost;
    for i in 0..p.len() {
        p[i].cost -= worst; 
    }

    let mut new_p: Vec<Schedule> = Vec::with_capacity(n_individuals);

    for i in (0..n_individuals).step_by(2) {
        let mut selected: Vec<Schedule> = roulette_selection(p);
        let r: f64 = rand::random();
        if r < crossover_rate {
            multi_point_crossover(&mut selected, n_genes);
        }
        for j in 0..2 {
            if n_individuals % 2 == 1 && i == n_individuals - 1 && j == 1 {
                break;
            }
            new_p.push(selected[j].clone());
        }
    }
    return new_p;
}

fn mutation(
    p: &mut [Schedule],
    n_individuals: usize,
    n_genes: usize,
    mutation_rate: f64
) {

    for i in 0..n_individuals {
        for j in 0..n_genes {
            let r: f64 = rand::random();
            if r < mutation_rate {
                p[i].status[j] = !p[i].status[j];
            }
        }
    }
}

fn elitism(
    p: &mut [Schedule],
    new_p: &[Schedule],
    n_individuals: usize,
    elitism_rate: f64
) {

    let n_elites: usize = (elitism_rate * n_individuals as f64).ceil() as usize;
    for i in n_elites..n_individuals {
        p[i] = new_p[i - n_elites].clone();
    }
}

fn update_cost(
    p: &mut [Schedule],
    n_individuals: usize,
    units: &[Unit],
    demand: &[f64],
    n_hours: usize,
    n_units: usize,
    desc: &[usize],
    asc: &[usize]
) {

    for i in 0..n_individuals {
        repair(&mut p[i].status, demand.len(), units.len(), units, demand, desc, asc);
        p[i].power = lambda_iteration(& p[i].status, units, demand, n_hours, n_units);
        let (fuel, startup) = calculate_cost(&p[i].power, n_hours, n_units, units);
        p[i].fuel = fuel;
        p[i].startup = startup;
        p[i].cost = sum_vec(&p[i].fuel) + sum_vec(&p[i].startup);
    }
    p.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
}