
use rand::Rng;
use rand::seq::SliceRandom;
use crate::unit::Unit;
use crate::schedule::*;
use crate::solution::*;

const OPERATOR_PROBABILITY: f64 = 1.0;


/* Hybrid Bald Eagle Search */
pub fn hbes(
    lm: f64,
    a: f64,
    r: f64,
    units: &[Unit],
    demand: &[f64],
    desc: &[usize],
    asc: &[usize],
    n_hours: usize,
    n_units: usize,
    n_pop: usize,
    max_iter: usize
) -> Schedule {

    let mut rng = rand::thread_rng();
    let dim: usize = n_hours * n_units;

    let mut best: Schedule = empty_schedule(n_hours, n_units);

    let mut alpha: Solution = Solution::empty(dim, true);
    let mut beta: Solution = Solution::empty(dim, true);
    let mut delta: Solution = Solution::empty(dim, true);
    let mut population: Vec<Solution> = Vec::with_capacity(n_pop);


    /* Initialize random solution. */
    for _ in 0..n_pop {
        
        let mut solution: Solution = Solution::empty(dim, false);
        let mut status: Vec<bool> = vec![false; dim];

        for d in 0..dim {
            solution.value[d] = rng.gen::<f64>();
            status[d] = transfer(solution.value[d], alpha.value[d], 0, max_iter, &mut rng);

        }

        let schedule: Schedule = ucp(&status, n_hours, n_units, units, demand, desc, asc);
        solution.cost = schedule.cost;
        
        if solution.cost < alpha.cost {
            alpha = solution.clone();
            best = schedule;
        }
        if solution.cost > alpha.cost && solution.cost < beta.cost {
            beta = solution.clone();
        }
        if solution.cost > alpha.cost && solution.cost > beta.cost && solution.cost < delta.cost {
            delta = solution.clone();
        }

        population.push(solution);
    }

    // println!("0, Best Cost: {:.2}", alpha.cost);


    for i in 0..max_iter {

        /* Improved BESO */
        // let lm: f64 = 1.5 * (max_iter - i + 1) as f64 / max_iter as f64;

        /* ---------------------------------------------------------------------------------------------------- */
        /* Select Space. */
        /* ---------------------------------------------------------------------------------------------------- */
        let pop_mean: Vec<f64> = get_population_mean(&population, n_pop, n_hours * n_units);
        for p in 0..n_pop {

            let mut solution: Solution = Solution::empty(dim, false);
            let mut status: Vec<bool> = vec![false; dim];

            for d in 0..dim {
                /* Original BES */
                solution.value[d] = alpha.value[d] + lm * rng.gen::<f64>() * (pop_mean[d] - population[p].value[d]);


                // /* Modified with sine cosine algorithm */
                // let b: f64 = 2.0;
                // let r1: f64 = b - i as f64 * (b / max_iter as f64);
                // let r2: f64 = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
                // let r3: f64 = 2.0 * rng.gen::<f64>();
                // let r4: f64 = rng.gen::<f64>();
                // if r4 < 0.5 {
                //     solution.value[d] = population[p].value[d] + r1 * r2.sin() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // } else {
                //     solution.value[d] = population[p].value[d] + r1 * r2.cos() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // }

                
                // /* Modified with grey wolf optimizer. */
                // let z: f64 = 2.0 - i as f64 * 2.0 / max_iter as f64;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a1: f64 = 2.0 * z * r1 - z;
                // let c1 = 2.0 * r2;
                // let d_alpha: f64 = f64::abs(c1 * alpha.value[d] - population[p].value[d]);
                // let x1: f64 = alpha.value[d] - a1 * d_alpha;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a2: f64 = 2.0 * z * r1 - z;
                // let c2: f64 = 2.0 * r2;
                // let d_beta: f64 = f64::abs(c2 * beta.value[d] - population[p].value[d]);
                // let x2: f64 = beta.value[d] - a2 * d_beta;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * z * r1 - z;
                // let c3: f64 = 2.0 * r2;
                // let d_delta: f64 = f64::abs(c3 * delta.value[d] - population[p].value[d]);
                // let x3: f64 = delta.value[d] - a3 * d_delta;
                // solution.value[d] = (x1 + x2 + x3) / 3.0;


                // /* Modified with whale optimization algorithm. */
                // let a1: f64 = 2.0 - i as f64 * (2.0 / max_iter as f64);
                // let a2: f64 = -1.0 + i as f64 * (-1.0 / max_iter as f64);
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let r3: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * a1 * r1 - a1;
                // let b1: f64 = 1.0;
                // let c2: f64 = 2.0 * r2;
                // let l1: f64 = (a2 - 1.0) * rng.gen::<f64>() + 1.0;
                // if r3 < 0.5 {
                //     if a3.abs() >= 1.0 {
                //         let rand_alpha_index: usize = rng.gen_range(0..n_pop);
                //         let d_x_rand: f64 = f64::abs(c2 * population[rand_alpha_index].value[d] - population[p].value[d]);
                //         solution.value[d] = population[rand_alpha_index].value[d] - a3 * d_x_rand;
                //     } else {
                //         let d_alpha: f64 = f64::abs(c2 * alpha.value[d] - population[p].value[d]);
                //         solution.value[d] = alpha.value[d] - a3 * d_alpha;
                //     }
                // } else {
                //     let distance_2_alpha: f64 = f64::abs(alpha.value[d] - population[p].value[d]);
                //     solution.value[d] = distance_2_alpha * f64::exp(b1 * l1) * f64::cos(l1 * 2.0 * std::f64::consts::PI) + alpha.value[d];
                // }




                status[d] = transfer(solution.value[d], alpha.value[d], i, max_iter, &mut rng);

            }

            let schedule: Schedule = ucp(&status, n_hours, n_units, units, demand, desc, asc);
            solution.cost = schedule.cost;


            if solution.cost < population[p].cost {
                population[p] = solution;
                
                if population[p].cost < alpha.cost {
                    alpha = population[p].clone();
                    best = schedule;
                }
                if population[p].cost > alpha.cost && population[p].cost < beta.cost {
                    beta = population[p].clone();
                }
                if population[p].cost > alpha.cost && population[p].cost > beta.cost && population[p].cost < delta.cost {
                    delta = population[p].clone();
                }
            }
        }


        /* ---------------------------------------------------------------------------------------------------- */
        /* Search Space. */
        /* ---------------------------------------------------------------------------------------------------- */

        let pop_mean: Vec<f64> = get_population_mean(&population, n_pop, n_hours * n_units);

        for p in 0..(n_pop - 1) {
            shuffle_population(&mut population, n_pop, &mut rng);
            let (x, y) = polr(a, r, n_pop, &mut rng);

            let mut step1: Vec<f64> = vec![0.0; dim];
            let mut step2: Vec<f64> = vec![0.0; dim];

            let mut solution: Solution = Solution::empty(dim, false);
            let mut status: Vec<bool> = vec![false; dim];

            for d in 0..dim {
                /* Original BES */
                step1[d] = population[p].value[d] - population[p + 1].value[d];
                step2[d] = population[p].value[d] - pop_mean[d];
                solution.value[d] = population[p].value[d] + y[p] * step1[d] + x[p] * step2[d];


                // /* Modified with grey wolf optimizer. */
                // let z: f64 = 2.0 - i as f64 * 2.0 / max_iter as f64;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a1: f64 = 2.0 * z * r1 - z;
                // let c1 = 2.0 * r2;
                // let d_alpha: f64 = f64::abs(c1 * alpha.value[d] - population[p].value[d]);
                // let x1: f64 = alpha.value[d] - a1 * d_alpha;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a2: f64 = 2.0 * z * r1 - z;
                // let c2: f64 = 2.0 * r2;
                // let d_beta: f64 = f64::abs(c2 * beta.value[d] - population[p].value[d]);
                // let x2: f64 = beta.value[d] - a2 * d_beta;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * z * r1 - z;
                // let c3: f64 = 2.0 * r2;
                // let d_delta: f64 = f64::abs(c3 * delta.value[d] - population[p].value[d]);
                // let x3: f64 = delta.value[d] - a3 * d_delta;
                // solution.value[d] = (x1 + x2 + x3) / 3.0;

                // /* Modified with sine cosine algorithm */
                // let b: f64 = 2.0;
                // let r1: f64 = b - i as f64 * (b / max_iter as f64);
                // let r2: f64 = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
                // let r3: f64 = 2.0 * rng.gen::<f64>();
                // let r4: f64 = rng.gen::<f64>();
                // if r4 < 0.5 {
                //     solution.value[d] = population[p].value[d] + r1 * r2.sin() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // } else {
                //     solution.value[d] = population[p].value[d] + r1 * r2.cos() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // }


                // /* Modified with whale optimization algorithm. */
                // let a1: f64 = 2.0 - i as f64 * (2.0 / max_iter as f64);
                // let a2: f64 = -1.0 + i as f64 * (-1.0 / max_iter as f64);
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let r3: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * a1 * r1 - a1;
                // let b1: f64 = 1.0;
                // let c2: f64 = 2.0 * r2;
                // let l1: f64 = (a2 - 1.0) * rng.gen::<f64>() + 1.0;
                // if r3 < 0.5 {
                //     if a3.abs() >= 1.0 {
                //         let rand_alpha_index: usize = rng.gen_range(0..n_pop);
                //         let d_x_rand: f64 = f64::abs(c2 * population[rand_alpha_index].value[d] - population[p].value[d]);
                //         solution.value[d] = population[rand_alpha_index].value[d] - a3 * d_x_rand;
                //     } else {
                //         let d_alpha: f64 = f64::abs(c2 * alpha.value[d] - population[p].value[d]);
                //         solution.value[d] = alpha.value[d] - a3 * d_alpha;
                //     }
                // } else {
                //     let distance_2_alpha: f64 = f64::abs(alpha.value[d] - population[p].value[d]);
                //     solution.value[d] = distance_2_alpha * f64::exp(b1 * l1) * f64::cos(l1 * 2.0 * std::f64::consts::PI) + alpha.value[d];
                // }
                


                status[d] = transfer(solution.value[d], alpha.value[d], i, max_iter, &mut rng);

            }

            let schedule: Schedule = ucp(&status, n_hours, n_units, units, demand, desc, asc);
            solution.cost = schedule.cost;

            if solution.cost < population[p].cost {
                population[p] = solution;
                
                if population[p].cost < alpha.cost {
                    alpha = population[p].clone();
                    best = schedule;
                }
                if population[p].cost > alpha.cost && population[p].cost < beta.cost {
                    beta = population[p].clone();
                }
                if population[p].cost > alpha.cost && population[p].cost > beta.cost && population[p].cost < delta.cost {
                    delta = population[p].clone();
                }
            }
        }


        /* ---------------------------------------------------------------------------------------------------- */
        /* Swoop. */
        /* ---------------------------------------------------------------------------------------------------- */

        let pop_mean: Vec<f64> = get_population_mean(&population, n_pop, n_hours * n_units);

        for p in 0..n_pop {
            shuffle_population(&mut population, n_pop, &mut rng);
            let (x, y) = swoo_p(a, n_pop, &mut rng);

            let mut step1: Vec<f64> = vec![0.0; dim];
            let mut step2: Vec<f64> = vec![0.0; dim];

            let mut solution: Solution = Solution::empty(dim, false);
            let mut status: Vec<bool> = vec![false; dim];

            for d in 0..dim {

                /* Original BES */
                step1[d] = population[p].value[d] - 2.0 * pop_mean[d];
                step2[d] = population[p].value[d] - 2.0 * alpha.value[d];
                solution.value[d] = rng.gen::<f64>() * alpha.value[d] + x[p] * step1[d] + y[p] * step2[d];

                // /* Modified with grey wolf optimizer. */
                // let z: f64 = 2.0 - i as f64 * 2.0 / max_iter as f64;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a1: f64 = 2.0 * z * r1 - z;
                // let c1 = 2.0 * r2;
                // let d_alpha: f64 = f64::abs(c1 * alpha.value[d] - population[p].value[d]);
                // let x1: f64 = alpha.value[d] - a1 * d_alpha;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a2: f64 = 2.0 * z * r1 - z;
                // let c2: f64 = 2.0 * r2;
                // let d_beta: f64 = f64::abs(c2 * beta.value[d] - population[p].value[d]);
                // let x2: f64 = beta.value[d] - a2 * d_beta;
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * z * r1 - z;
                // let c3: f64 = 2.0 * r2;
                // let d_delta: f64 = f64::abs(c3 * delta.value[d] - population[p].value[d]);
                // let x3: f64 = delta.value[d] - a3 * d_delta;
                // solution.value[d] = (x1 + x2 + x3) / 3.0;

                // /* Modified with sine cosine algorithm */
                // let b: f64 = 2.0;
                // let r1: f64 = b - i as f64 * (b / max_iter as f64);
                // let r2: f64 = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
                // let r3: f64 = 2.0 * rng.gen::<f64>();
                // let r4: f64 = rng.gen::<f64>();
                // if r4 < 0.5 {
                //     solution.value[d] = population[p].value[d] + r1 * r2.sin() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // } else {
                //     solution.value[d] = population[p].value[d] + r1 * r2.cos() * (r3 * alpha.value[d] - population[p].value[d]).abs();
                // }

                // /* Modified with whale optimization algorithm. */
                // let a1: f64 = 2.0 - i as f64 * (2.0 / max_iter as f64);
                // let a2: f64 = -1.0 + i as f64 * (-1.0 / max_iter as f64);
                // let r1: f64 = rng.gen::<f64>();
                // let r2: f64 = rng.gen::<f64>();
                // let r3: f64 = rng.gen::<f64>();
                // let a3: f64 = 2.0 * a1 * r1 - a1;
                // let b1: f64 = 1.0;
                // let c2: f64 = 2.0 * r2;
                // let l1: f64 = (a2 - 1.0) * rng.gen::<f64>() + 1.0;
                // if r3 < 0.5 {
                //     if a3.abs() >= 1.0 {
                //         let rand_alpha_index: usize = rng.gen_range(0..n_pop);
                //         let d_x_rand: f64 = f64::abs(c2 * population[rand_alpha_index].value[d] - population[p].value[d]);
                //         solution.value[d] = population[rand_alpha_index].value[d] - a3 * d_x_rand;

                //     } else {
                //         let d_alpha: f64 = f64::abs(c2 * alpha.value[d] - population[p].value[d]);
                //         solution.value[d] = alpha.value[d] - a3 * d_alpha;
                //     }

                // } else {
                //     let distance_2_alpha: f64 = f64::abs(alpha.value[d] - population[p].value[d]);
                //     solution.value[d] = distance_2_alpha * f64::exp(b1 * l1) * f64::cos(l1 * 2.0 * std::f64::consts::PI) + alpha.value[d];
                // }


                status[d] = transfer(solution.value[d], alpha.value[d], i, max_iter, &mut rng);
            }


            let schedule: Schedule = ucp(&status, n_hours, n_units, units, demand, desc, asc);
            solution.cost = schedule.cost;

            if solution.cost < population[p].cost {
                population[p] = solution;
                
                if population[p].cost < alpha.cost {
                    alpha = population[p].clone();
                    best = schedule;
                }
                if population[p].cost > alpha.cost && population[p].cost < beta.cost {
                    beta = population[p].clone();
                }
                if population[p].cost > alpha.cost && population[p].cost > beta.cost && population[p].cost < delta.cost {
                    delta = population[p].clone();
                }
            }
        }


        /* Swap window operator */
        if rng.gen::<f64>() < OPERATOR_PROBABILITY {
            let index_first: usize = rng.gen_range(0..population.len());
            let mut index_second: usize = rng.gen_range(0..population.len());
            while index_first == index_second {
                index_second = rng.gen_range(0..population.len());
            }

            let window_start: usize = rng.gen_range(0..(dim - 1));
            let window_end: usize = rng.gen_range((window_start + 1)..dim);

            for w in window_start..=window_end {
                let temp: f64 = population[index_first].value[w];
                population[index_first].value[w] = population[index_second].value[w];
                population[index_second].value[w] = temp;
            }

            let mut status_first: Vec<bool> = vec![false; dim];
            let mut status_second: Vec<bool> = vec![false; dim];

            for d in 0..dim {
                status_first[d] = transfer(population[index_first].value[d], alpha.value[d], i, max_iter, &mut rng);
                status_second[d] = transfer(population[index_second].value[d], alpha.value[d], i, max_iter, &mut rng);
            }

            let schedule_first: Schedule = ucp(&status_first, n_hours, n_units, units, demand, desc, asc);
            let schedule_second: Schedule = ucp(&status_second, n_hours, n_units, units, demand, desc, asc);
            population[index_first].cost = schedule_first.cost;
            population[index_second].cost = schedule_second.cost;

            if population[index_first].cost < alpha.cost {
                alpha = population[index_first].clone();
                best = schedule_first;
            }
            if population[index_first].cost > alpha.cost && population[index_first].cost < beta.cost {
                beta = population[index_first].clone();
            }
            if population[index_first].cost > alpha.cost && population[index_first].cost > beta.cost && population[index_first].cost < delta.cost {
                delta = population[index_first].clone();
            }

            if population[index_second].cost < alpha.cost {
                alpha = population[index_second].clone();
                best = schedule_second;
            }
            if population[index_second].cost > alpha.cost && population[index_second].cost < beta.cost {
                beta = population[index_second].clone();
            }
            if population[index_second].cost > alpha.cost && population[index_second].cost > beta.cost && population[index_second].cost < delta.cost {
                delta = population[index_second].clone();
            }
        }


        /* Window mutation operator */
        if rng.gen::<f64>() < OPERATOR_PROBABILITY {
            let index_selected: usize = rand::thread_rng().gen_range(0..population.len());

            let window_start: usize = rand::thread_rng().gen_range(0..(dim - 1));
            let window_end: usize = rand::thread_rng().gen_range((window_start + 1)..dim);

            for w in window_start..=window_end {
                population[index_selected].value[w] = if population[index_selected].value[w] >= 0.5 {0.0} else {1.0};
            }

            let mut status_selected: Vec<bool> = vec![false; dim];

            for d in 0..dim {
                status_selected[d] = transfer(population[index_selected].value[d], alpha.value[d], i, max_iter, &mut rng);
            }

            let schedule_first: Schedule = ucp(&status_selected, n_hours, n_units, units, demand, desc, asc);
            population[index_selected].cost = schedule_first.cost;

            if population[index_selected].cost < alpha.cost {
                alpha = population[index_selected].clone();
                best = schedule_first;
            } else if population[index_selected].cost > alpha.cost && population[index_selected].cost < beta.cost {
                beta = population[index_selected].clone();
            } else if population[index_selected].cost > alpha.cost && population[index_selected].cost > beta.cost && population[index_selected].cost < delta.cost {
                delta = population[index_selected].clone();
            }
        }






        /* Old */
        // for p in 0..n_pop {
        //     if rng.gen::<f64>() < OPERATOR_PROBABILITY {
        //         let unit_index_first: usize = rng.gen_range(0..n_units);
        //         let mut unit_index_second: usize = rng.gen_range(0..n_units);
        //         while unit_index_first == unit_index_second {
        //             unit_index_second = rng.gen_range(0..n_units);
        //         }

        //         let window_width: usize = rng.gen_range(0..n_hours);
        //         let window_position: usize = rng.gen_range(0..(n_hours - window_width));

        //         for h in window_position..(window_position + window_width) {
        //             let temp: f64 = population[p].value[h * n_units + unit_index_first];
        //             population[p].value[h * n_units + unit_index_first] = population[p].value[h * n_units + unit_index_second];
        //             population[p].value[h * n_units + unit_index_second] = temp;
        //         }

        //         let mut status_first: Vec<bool> = vec![false; dim];

        //         for d in 0..dim {
        //             status_first[d] = transfer(population[p].value[d], alpha.value[d], i, max_iter, &mut rng);
        //         }

        //         let schedule_first: Schedule = ucp(&status_first, n_hours, n_units, units, demand, desc, asc);
        //         population[p].cost = schedule_first.cost;


        //         if population[p].cost < alpha.cost {
        //             alpha = population[p].clone();
        //             best = schedule_first;
        //         } else if population[p].cost > alpha.cost && population[p].cost < beta.cost {
        //             beta = population[p].clone();
        //         } else if population[p].cost > alpha.cost && population[p].cost > beta.cost && population[p].cost < delta.cost {
        //             delta = population[p].clone();
        //         }
        //     }
        // }


        


        // for p in 0..n_pop {
        //     if rng.gen::<f64>() < OPERATOR_PROBABILITY {
        //         let unit_index: usize = rng.gen_range(0..n_units);

        //         let window_width: usize = rng.gen_range(0..n_hours);
        //         let window_position: usize = rng.gen_range(0..(n_hours - window_width));

        //         // let rand_1: f64 = rng.gen::<f64>();

        //         for h in window_position..(window_position + window_width) {
        //             population[p].value[h * n_units + unit_index] = if population[p].value[h * n_units + unit_index] >= 0.5 {0.0} else {1.0};

        //             // population[p].value[h * n_units + unit_index] = if rand_1 >= 0.5 {0.0} else {1.0};
        //         }

        //         let mut status_selected: Vec<bool> = vec![false; dim];

        //         for d in 0..dim {
        //             status_selected[d] = transfer(population[p].value[d], alpha.value[d], i, max_iter, &mut rng);
        //         }

        //         let schedule_first: Schedule = ucp(&status_selected, n_hours, n_units, units, demand, desc, asc);
        //         population[p].cost = schedule_first.cost;

        //         if population[p].cost < alpha.cost {
        //             alpha = population[p].clone();
        //             best = schedule_first;
        //         }
        //         if population[p].cost > alpha.cost && population[p].cost < beta.cost {
        //             beta = population[p].clone();
        //         }
        //         if population[p].cost > alpha.cost && population[p].cost > beta.cost && population[p].cost < delta.cost {
        //             delta = population[p].clone();
        //         }
        //     }
        // }



        


        // /* Swap mutation operator */
        // {
        //     let mut selected_first: Solution = alpha.clone();
        //     let mut selected_second: Solution = beta.clone();

        //     let window_start: usize = rng.gen_range(0..(dim - 1));
        //     let window_end: usize = rng.gen_range((window_start + 1)..dim);

        //     for w in window_start..=window_end {
        //         let temp: f64 = selected_first.value[w];
        //         selected_first.value[w] = selected_second.value[w];
        //         selected_second.value[w] = temp;
        //     }

        //     let mut status_first: Vec<bool> = vec![false; dim];
        //     let mut status_second: Vec<bool> = vec![false; dim];

        //     for d in 0..dim {
        //         status_first[d] = transfer(selected_first.value[d], alpha.value[d], i, max_iter, &mut rng);
        //         status_second[d] = transfer(selected_second.value[d], alpha.value[d], i, max_iter, &mut rng);
        //     }

        //     let schedule_first: Schedule = ucp(&status_first, n_hours, n_units, units, demand, desc, asc);
        //     let schedule_second: Schedule = ucp(&status_second, n_hours, n_units, units, demand, desc, asc);
        //     selected_first.cost = schedule_first.cost;
        //     selected_second.cost = schedule_second.cost;

        //     if selected_first.cost < alpha.cost {
        //         alpha = selected_first;
        //         best = schedule_first;
        //     } else if selected_first.cost > alpha.cost && selected_first.cost < beta.cost {
        //         beta = selected_first;
        //     } else if selected_first.cost > alpha.cost && selected_first.cost > beta.cost && selected_first.cost < delta.cost {
        //         delta = selected_first;
        //     }

        //     if selected_second.cost < alpha.cost {
        //         alpha = selected_second;
        //         best = schedule_second;
        //     } else if selected_second.cost > alpha.cost && selected_second.cost < beta.cost {
        //         beta = selected_second;
        //     } else if selected_second.cost > alpha.cost && selected_second.cost > beta.cost && selected_second.cost < delta.cost {
        //         delta = selected_second.clone();
        //     }
        // }
        



        // /* Swap window hill climb operator */
        // if rng.gen::<f64>() < OPERATOR_PROBABILITY {

        //     let window_width: usize = rng.gen_range(0..dim);
        //     // let window_position: usize = rng.gen_range(0..(dim - window_width));
        //     let steps: usize = dim - window_width;

        //     for s in 0..=steps {

        //         let mut selected_first: Solution = alpha.clone();
        //         let mut selected_second: Solution = beta.clone();

        //         for d in s..(window_width + s) {
        //             let temp: f64 = selected_first.value[d];
        //             selected_first.value[d] = selected_second.value[d];
        //             selected_second.value[d] = temp;
        //         }

        //         let mut status_first: Vec<bool> = vec![false; dim];
        //         let mut status_second: Vec<bool> = vec![false; dim];

        //         for d in 0..dim {
        //             status_first[d] = transfer(selected_first.value[d], alpha.value[d], i, max_iter, &mut rng);
        //             status_second[d] = transfer(selected_second.value[d], alpha.value[d], i, max_iter, &mut rng);
        //         }

        //         let schedule_first: Schedule = ucp(&status_first, n_hours, n_units, units, demand, desc, asc);
        //         let schedule_second: Schedule = ucp(&status_second, n_hours, n_units, units, demand, desc, asc);
        //         selected_first.cost = schedule_first.cost;
        //         selected_second.cost = schedule_second.cost;


        //         if selected_first.cost < alpha.cost {
        //             alpha = selected_first;
        //             best = schedule_first;
        //         } else if selected_first.cost > alpha.cost && selected_first.cost < beta.cost {
        //             beta = selected_first;
        //         } else if selected_first.cost > alpha.cost && selected_first.cost > beta.cost && selected_first.cost < delta.cost {
        //             delta = selected_first;
        //         }

        //         if selected_second.cost < alpha.cost {
        //             alpha = selected_second;
        //             best = schedule_second;
        //         } else if selected_second.cost > alpha.cost && selected_second.cost < beta.cost {
        //             beta = selected_second;
        //         } else if selected_second.cost > alpha.cost && selected_second.cost > beta.cost && selected_second.cost < delta.cost {
        //             delta = selected_second;
        //         }
        //     }
        // }


        
        // println!("{}, Best Cost: {:.2}", i + 1, alpha.cost);
        // println!("{:.2}", alpha.cost);
    }

    best
}


fn get_population_mean(
    population: &[Solution],
    rows: usize,
    columns: usize
) -> Vec<f64> {

    let mut sum: Vec<f64> = vec![0.0; columns];
    for i in 0..rows {
        for j in 0..columns {
            sum[j] += population[i].value[j];
        }
    }
    for i in 0..columns {
        sum[i] /= rows as f64;
    }
    sum
}


fn polr(
    a: f64,
    r: f64,
    n: usize,
    rng: &mut rand::rngs::ThreadRng
) -> (Vec<f64>, Vec<f64>) {

    // Set parameters
    let mut th: Vec<f64> = vec![0.0; n];
    let mut rr: Vec<f64> = vec![0.0; n];
    let mut xr: Vec<f64> = vec![0.0; n];
    let mut yr: Vec<f64> = vec![0.0; n];

    let mut xr_max: f64 = 0.0;
    let mut yr_max: f64 = 0.0;

    for i in 0..n {
        th[i] = a * std::f64::consts::PI * rng.gen::<f64>();
        rr[i] = th[i] + r * rng.gen::<f64>();
        xr[i] = rr[i] * f64::sin(th[i]);
        yr[i] = rr[i] * f64::cos(th[i]);

        let xr_abs: f64 = f64::abs(xr[i]);
        let yr_abs: f64 = f64::abs(yr[i]);

        if xr_abs > xr_max {
            xr_max = xr_abs;
        }
        if yr_abs > yr_max {
            yr_max = yr_abs;
        }
    }

    for i in 0..n {
        xr[i] /= xr_max;
        yr[i] /= yr_max;
    }

    (xr, yr)
}

fn swoo_p(
    a: f64,
    n: usize,
    rng: &mut rand::rngs::ThreadRng
) -> (Vec<f64>, Vec<f64>) {

    // Set parameters
    let mut th: Vec<f64> = vec![0.0; n];
    let mut xr: Vec<f64> = vec![0.0; n];
    let mut yr: Vec<f64> = vec![0.0; n];

    let mut xr_max: f64 = 0.0;
    let mut yr_max: f64 = 0.0;

    for i in 0..n {
        th[i] = a * std::f64::consts::PI * f64::exp(rng.gen::<f64>());
        xr[i] = th[i] * f64::sinh(th[i]);
        yr[i] = th[i] * f64::cosh(th[i]);

        let xr_abs: f64 = f64::abs(xr[i]);
        let yr_abs: f64 = f64::abs(yr[i]);

        if xr_abs > xr_max {
            xr_max = xr_abs;
        }
        if yr_abs > yr_max {
            yr_max = yr_abs;
        }
    }

    for i in 0..n {
        xr[i] /= xr_max;
        yr[i] /= yr_max;
    }

    (xr, yr)
}


fn shuffle_population(
    population: &mut [Solution],
    // population_cost: &mut [f64],
    n_pop: usize,
    rng: &mut rand::rngs::ThreadRng
) {

    let mut shuffled_index: Vec<usize> = (0..n_pop).collect();
    shuffled_index.shuffle(rng);

    let mut temp_population: Vec<Solution> = Vec::with_capacity(n_pop);
    for p in 0..n_pop {
        temp_population.push(population[p].clone());
    }
    // let temp_cost = population_cost.to_vec();
    for i in 0..n_pop {
        population[i] = temp_population[shuffled_index[i]].clone();
        // population_cost[i] = temp_cost[shuffled_index[i]].clone();
    }
}
