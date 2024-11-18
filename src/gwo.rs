use rand::Rng;
use crate::unit::Unit;
use crate::schedule::*;
use crate::solution::*;


const OPERATOR_PROBABILITY: f64 = 1.0;


pub fn gwo(
    units: &[Unit],
    demand: &[f64],
    desc: &[usize],
    asc: &[usize],
    n_pop: usize,
    max_iter: usize,
    dim: usize
) -> Schedule {

    let ub: f64 = 1.0;
    let lb: f64 = 0.0;
    let n_hours: usize = demand.len();
    let n_units: usize = units.len();
    let mut rng = rand::thread_rng();

    let mut best: Schedule = empty_schedule(n_hours, n_units);

    let mut alpha: Solution = Solution::empty(dim, true);
    let mut beta: Solution = Solution::empty(dim, true);
    let mut delta: Solution = Solution::empty(dim, true);

    let mut population: Vec<Solution> = Vec::with_capacity(dim);

    // /* Random solution initialization */
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

    for t in 0..max_iter {
        let a: f64 = 2.0 - t as f64 * 2.0 / max_iter as f64;
        for p in 0..n_pop {

            let mut status: Vec<bool> = vec![false; n_hours * n_units];

            for d in 0..dim {
                let r1: f64 = rng.gen::<f64>();
                let r2: f64 = rng.gen::<f64>();
                let a1: f64 = 2.0 * a * r1 - a;
                let c1 = 2.0 * r2;
                let d_alpha: f64 = f64::abs(c1 * alpha.value[d] - population[p].value[d]);
                let x1: f64 = alpha.value[d] - a1 * d_alpha;
                let r1: f64 = rng.gen::<f64>();
                let r2: f64 = rng.gen::<f64>();
                let a2: f64 = 2.0 * a * r1 - a;
                let c2: f64 = 2.0 * r2;
                let d_beta: f64 = f64::abs(c2 * beta.value[d] - population[p].value[d]);
                let x2: f64 = beta.value[d] - a2 * d_beta;
                let r1: f64 = rng.gen::<f64>();
                let r2: f64 = rng.gen::<f64>();
                let a3: f64 = 2.0 * a * r1 - a;
                let c3: f64 = 2.0 * r2;
                let d_delta: f64 = f64::abs(c3 * delta.value[d] - population[p].value[d]);
                let x3: f64 = delta.value[d] - a3 * d_delta;
                population[p].value[d] = (x1 + x2 + x3) / 3.0;
                population[p].value[d] = f64::max(population[p].value[d], lb);
                population[p].value[d] = f64::min(population[p].value[d], ub);

                status[d] = transfer(population[p].value[d], alpha.value[d], t, max_iter, &mut rng);
            }

            let schedule: Schedule = ucp(&status, n_hours, n_units, units, demand, desc, asc);
            population[p].cost = schedule.cost;

            if population[p].cost < alpha.cost {
                alpha = population[p].clone();
                
                best = schedule;
            } else if schedule.cost > alpha.cost && schedule.cost < beta.cost {
                beta = population[p].clone();
                
            } else if schedule.cost > alpha.cost && schedule.cost > beta.cost && schedule.cost < delta.cost {
                delta = population[p].clone();
            }
        }
        // println!("iteration {} best: {}", t, alpha.cost);

        
        /* Swap window operator */
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
                status_first[d] = transfer(population[index_first].value[d], alpha.value[d], t, max_iter, &mut rng);
                status_second[d] = transfer(population[index_second].value[d], alpha.value[d], t, max_iter, &mut rng);
            }

            let schedule_first: Schedule = ucp(&status_first, n_hours, n_units, units, demand, desc, asc);
            let schedule_second: Schedule = ucp(&status_second, n_hours, n_units, units, demand, desc, asc);
            population[index_first].cost = schedule_first.cost;
            population[index_second].cost = schedule_second.cost;

            if population[index_first].cost < alpha.cost {
                alpha = population[index_first].clone();
                best = schedule_first;
            }

            if population[index_second].cost < alpha.cost {
                alpha = population[index_second].clone();
                best = schedule_second;
            }
        }


        /* Window mutation operator */
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



        if rng.gen::<f64>() < OPERATOR_PROBABILITY {
            let index_selected: usize = rand::thread_rng().gen_range(0..population.len());

            let window_start: usize = rand::thread_rng().gen_range(0..(dim - 1));
            let window_end: usize = rand::thread_rng().gen_range((window_start + 1)..dim);

            for w in window_start..=window_end {
                population[index_selected].value[w] = if population[index_selected].value[w] >= 0.5 {0.0} else {1.0};
            }

            let mut status_selected: Vec<bool> = vec![false; dim];

            for d in 0..dim {
                status_selected[d] = transfer(population[index_selected].value[d], alpha.value[d], t, max_iter, &mut rng);
            }

            let schedule_first: Schedule = ucp(&status_selected, n_hours, n_units, units, demand, desc, asc);
            population[index_selected].cost = schedule_first.cost;

            if population[index_selected].cost < alpha.cost {
                alpha = population[index_selected].clone();
                best = schedule_first;
            }
        }


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




        // println!("{:.2}", alpha.cost);


    }
    best
}
