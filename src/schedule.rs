use crate::unit::Unit;
use std::fs;
// use rand::Rng;

const SPINNING_RESERVE: f64 = 1.1;

#[derive(Clone)]
pub struct Schedule {
    pub status: Vec<bool>,
    pub power: Vec<f64>,
    pub fuel: Vec<f64>,
    pub startup: Vec<f64>,
    pub cost: f64,
}

pub fn empty_schedule(n_hours: usize, n_units: usize) -> Schedule {
    Schedule {
        status: vec![false; n_hours * n_units],
        power: vec![0.0; n_hours * n_units],
        fuel: vec![0.0; n_hours],
        startup: vec![0.0; n_hours],
        cost: f64::INFINITY,
    }
}

// pub fn new_schedule(
//     n_hours: usize,
//     n_units: usize,
//     units: &[Unit],
//     demand: &[f64]
// ) -> Schedule {

//     let mut status: Vec<bool> = vec![false; n_hours * n_units];

//     for i in 0..n_hours {
//         for j in 0..n_units {
//             status[i * n_units + j] = random::<bool>();
//         }
//     }

//     repair(&mut status, n_hours, n_units, units, demand);
//     let power: Vec<f64> = lambda_iteration(&status, units, demand, n_hours, n_units);
//     let (fuel, startup) = calculate_cost(&power, n_hours, n_units, units);
//     let cost: f64 = sum_vec(&fuel) + sum_vec(&startup);

//     Schedule {
//         status,
//         power,
//         fuel,
//         startup,
//         cost
//     }
// }

pub fn ucp(
    input: &[bool],
    n_hours: usize,
    n_units: usize,
    units: &[Unit],
    demand: &[f64],
    desc: &[usize],
    asc: &[usize],
) -> Schedule {
    let mut status: Vec<bool> = input.to_vec();
    repair(&mut status, n_hours, n_units, units, demand, desc, asc);
    let power: Vec<f64> = lambda_iteration(&status, units, demand, n_hours, n_units);
    let (fuel, startup) = calculate_cost(&power, n_hours, n_units, units);
    let cost: f64 = sum_vec(&fuel) + sum_vec(&startup);

    Schedule {
        status,
        power,
        fuel,
        startup,
        cost,
    }
}

pub fn calculate_cost(
    power: &[f64],
    n_hours: usize,
    n_units: usize,
    units: &[Unit],
) -> (Vec<f64>, Vec<f64>) {
    let mut fuel: Vec<f64> = vec![0.0; n_hours];
    let mut startup: Vec<f64> = vec![0.0; n_hours];
    // let mut cost: f64 = 0.0;
    // let mut fuel_cost: f64 = 0.0;
    // let mut startup_cost: f64 = 0.0;
    let mut time: Vec<isize> = vec![0; n_hours * n_units];

    for i in 0..n_hours {
        for j in 0..n_units {
            /* Calculate fuel cost. */
            if power[i * n_units + j] > 0.0 {
                fuel[i] += units[j].a
                    + power[i * n_units + j] * (units[j].b + units[j].c * power[i * n_units + j]);
                // cost += fuel[i];
            }

            /* Calculate startup cost. */
            if i == 0 {
                /* If unit was previously on and is currently on. */
                if units[j].initial_status > 0 && power[i * n_units + j] > 0.0 {
                    time[i * n_units + j] = units[j].initial_status + 1;

                /* If unit was previously on and is currently off. */
                } else if units[j].initial_status > 0 && power[i * n_units + j] == 0.0 {
                    time[i * n_units + j] = -1;

                /* If unit was previously off and is currently on. */
                } else if units[j].initial_status < 1 && power[i * n_units + j] > 0.0 {
                    if (units[j].min_downtime <= units[j].initial_status.abs())
                        && (units[j].initial_status.abs()
                            <= (units[j].min_downtime + units[j].cold_start_hours))
                    {
                        startup[i] += units[j].hot_start_cost;
                        // cost += startup[i];
                        // startup_cost += units[j].hot_start_cost;
                    } else if units[j].initial_status.abs()
                        > (units[j].min_downtime + units[j].cold_start_hours)
                    {
                        startup[i] += units[j].cold_start_cost;
                        // cost += startup[i];
                        // startup_cost += units[j].cold_start_cost;
                    }
                    time[i * n_units + j] = 1;

                /* If unit was previously off and is currently off. */
                } else if units[j].initial_status < 1 && power[i * n_units + j] == 0.0 {
                    time[i * n_units + j] = units[j].initial_status - 1;
                }
            } else {
                /* If unit was previously on and is currently on. */
                if time[(i - 1) * n_units + j] > 0 && power[i * n_units + j] > 0.0 {
                    time[i * n_units + j] = time[(i - 1) * n_units + j] + 1;

                /* If unit was previously on and is currently off. */
                } else if time[(i - 1) * n_units + j] > 0 && power[i * n_units + j] == 0.0 {
                    time[i * n_units + j] = -1;

                /* If unit was previously off and is currently on. */
                } else if time[(i - 1) * n_units + j] < 1 && power[i * n_units + j] > 0.0 {
                    if (units[j].min_downtime <= time[(i - 1) * n_units + j].abs())
                        && (time[(i - 1) * n_units + j].abs()
                            <= (units[j].min_downtime + units[j].cold_start_hours))
                    {
                        startup[i] += units[j].hot_start_cost;
                        // cost += startup[i];
                        // startup_cost += units[j].hot_start_cost;
                    } else if time[(i - 1) * n_units + j].abs()
                        > (units[j].min_downtime + units[j].cold_start_hours)
                    {
                        startup[i] += units[j].cold_start_cost;
                        // cost += startup[i];
                        // startup_cost += units[j].cold_start_cost;
                    }
                    time[i * n_units + j] = 1;

                /* If unit was previously off and is currently off. */
                } else if time[(i - 1) * n_units + j] < 1 && power[i * n_units + j] == 0.0 {
                    time[i * n_units + j] = time[(i - 1) * n_units + j] - 1;
                }
            }
        }
    }

    (fuel, startup)
}

pub fn repair(
    status: &mut [bool],
    n_hours: usize,
    n_units: usize,
    units: &[Unit],
    demand: &[f64],
    desc: &[usize],
    asc: &[usize],
) {
    let mut time: Vec<isize> = vec![0; n_hours * n_units];
    let mut max_hourly_load: Vec<f64> = vec![0.0; n_hours * n_units];
    // let desc: Vec<usize> = get_sorted_unit_indices(units, false);
    // let asc: Vec<usize> = get_sorted_unit_indices(units, true);

    for i in 0..n_hours {
        for j in 0..n_units {
            if i == 0 {
                /* If unit was previously on and is currently on. */
                if units[j].initial_status > 0 && status[i * n_units + j] {
                    time[i * n_units + j] = units[j].initial_status + 1;

                /* If unit was previously on and is currently off. */
                } else if units[j].initial_status > 0 && !status[i * n_units + j] {
                    if units[j].initial_status.abs() < units[j].min_uptime {
                        status[i * n_units + j] = true;
                        time[i * n_units + j] = units[j].initial_status + 1;
                    } else {
                        time[i * n_units + j] = -1;
                    }

                /* If unit was previously off and is currently on. */
                } else if units[j].initial_status < 1 && status[i * n_units + j] {
                    if units[j].initial_status.abs() < units[j].min_downtime {
                        status[i * n_units + j] = false;
                        time[i * n_units + j] = units[j].initial_status - 1;
                    } else {
                        time[i * n_units + j] = 1;
                    }

                /* If unit was previously off and is currently off. */
                } else if units[j].initial_status < 1 && !status[i * n_units + j] {
                    time[i * n_units + j] = units[j].initial_status - 1;
                }
            } else {
                /* If unit was previously on and is currently on. */
                if time[(i - 1) * n_units + j] > 0 && status[i * n_units + j] {
                    time[i * n_units + j] = time[(i - 1) * n_units + j] + 1;

                /* If unit was previously on and is currently off. */
                } else if time[(i - 1) * n_units + j] > 0 && !status[i * n_units + j] {
                    if time[(i - 1) * n_units + j].abs() < units[j].min_uptime {
                        status[i * n_units + j] = true;
                        time[i * n_units + j] = time[(i - 1) * n_units + j] + 1;
                    } else {
                        time[i * n_units + j] = -1;
                    }

                /* If unit was previously off and is currently on. */
                } else if time[(i - 1) * n_units + j] < 1 && status[i * n_units + j] {
                    if time[(i - 1) * n_units + j].abs() < units[j].min_downtime {
                        status[i * n_units + j] = false;
                        time[i * n_units + j] = time[(i - 1) * n_units + j] - 1;
                    } else {
                        time[i * n_units + j] = 1;
                    }

                /* If unit was previously off and is currently off. */
                } else if time[(i - 1) * n_units + j] < 1 && !status[i * n_units + j] {
                    time[i * n_units + j] = time[(i - 1) * n_units + j] - 1;
                }
            }

            if status[i * n_units + j] {
                max_hourly_load[i] += units[j].max_power;
            }
        }

        if max_hourly_load[i] < (demand[i] * SPINNING_RESERVE) {
            for j in 0..n_units {
                if i == 0 {
                    /* If unit was previously on and is currently off. */
                    if units[desc[j]].initial_status > 0 && !status[i * n_units + desc[j]] {
                        status[i * n_units + desc[j]] = true;
                        time[i * n_units + desc[j]] = units[desc[j]].initial_status + 1;
                        max_hourly_load[i] += units[desc[j]].max_power;

                    /* If unit was previously off and is currently off. */
                    } else if units[desc[j]].initial_status < 1 && !status[i * n_units + desc[j]] {
                        if units[desc[j]].initial_status.abs() >= units[desc[j]].min_downtime {
                            status[i * n_units + desc[j]] = true;
                            time[i * n_units + desc[j]] = 1;
                            max_hourly_load[i] += units[desc[j]].max_power;
                        }
                    }
                } else {
                    /* If unit was previously on and is currently off. */
                    if time[(i - 1) * n_units + desc[j]] > 0 && !status[i * n_units + desc[j]] {
                        status[i * n_units + desc[j]] = true;
                        time[i * n_units + desc[j]] = time[(i - 1) * n_units + desc[j]] + 1;
                        max_hourly_load[i] += units[desc[j]].max_power;

                    /* If unit was previously off and is currently off. */
                    } else if time[(i - 1) * n_units + desc[j]] < 1
                        && !status[i * n_units + desc[j]]
                    {
                        if time[(i - 1) * n_units + desc[j]].abs() >= units[desc[j]].min_downtime {
                            status[i * n_units + desc[j]] = true;
                            time[i * n_units + desc[j]] = 1;
                            max_hourly_load[i] += units[desc[j]].max_power;
                        } else {
                            for k in (i + 1 - time[i * n_units + desc[j]].abs() as usize)..=i {
                                status[k * n_units + desc[j]] = true;
                                if k == (i + 1 - time[i * n_units + desc[j]].abs() as usize) {
                                    time[k * n_units + desc[j]] = 1;
                                } else {
                                    time[k * n_units + desc[j]] =
                                        time[(k - 1) * n_units + desc[j]] + 1;
                                }
                                max_hourly_load[k] += units[desc[j]].max_power;
                            }
                        }
                    }
                }

                if max_hourly_load[i] >= (demand[i] * SPINNING_RESERVE) {
                    break;
                }
            }
        }

        if max_hourly_load[i] > (demand[i] * SPINNING_RESERVE) {
            for j in 0..n_units {
                if i == 0 {
                    if units[asc[j]].initial_status > 0 && status[i * n_units + asc[j]] {
                        if units[asc[j]].initial_status.abs() >= units[asc[j]].min_uptime {
                            if (max_hourly_load[i] - units[asc[j]].max_power)
                                >= (demand[i] * SPINNING_RESERVE)
                            {
                                status[i * n_units + asc[j]] = false;
                                time[i * n_units + asc[j]] = -1;
                                max_hourly_load[i] -= units[asc[j]].max_power;
                            }
                        }
                    } else if units[asc[j]].initial_status < 1 && status[i * n_units + asc[j]] {
                        if (max_hourly_load[i] - units[asc[j]].max_power)
                            >= (demand[i] * SPINNING_RESERVE)
                        {
                            status[i * n_units + asc[j]] = false;
                            time[i * n_units + asc[j]] = units[asc[j]].initial_status - 1;
                            max_hourly_load[i] -= units[asc[j]].max_power;
                        }
                    }
                } else {
                    if time[(i - 1) * n_units + asc[j]] > 0 && status[i * n_units + asc[j]] {
                        if time[(i - 1) * n_units + asc[j]].abs() >= units[asc[j]].min_uptime {
                            if (max_hourly_load[i] - units[asc[j]].max_power)
                                >= (demand[i] * SPINNING_RESERVE)
                            {
                                status[i * n_units + asc[j]] = false;
                                time[i * n_units + asc[j]] = -1;
                                max_hourly_load[i] -= units[asc[j]].max_power;
                            }
                        }
                    } else if time[(i - 1) * n_units + asc[j]] < 1 && status[i * n_units + asc[j]] {
                        if (max_hourly_load[i] - units[asc[j]].max_power)
                            >= (demand[i] * SPINNING_RESERVE)
                        {
                            status[i * n_units + asc[j]] = false;
                            time[i * n_units + asc[j]] = time[(i - 1) * n_units + asc[j]] - 1;
                            max_hourly_load[i] -= units[asc[j]].max_power;
                        }
                    }
                }
            }
        }
    }
}

pub fn get_sorted_unit_indices(units: &[Unit], is_ascending: bool) -> Vec<usize> {
    let mut indices = (0..units.len()).collect::<Vec<_>>();

    let mut fc: Vec<f64> = Vec::with_capacity(units.len());
    for u in 0..units.len() {
        fc.push(units[u].a / units[u].max_power + units[u].b + units[u].c * units[u].max_power);
    }

    if is_ascending {
        indices.sort_by(|&a, &b| fc[b].partial_cmp(&fc[a]).unwrap());
    } else {
        indices.sort_by(|&a, &b| fc[a].partial_cmp(&fc[b]).unwrap());
    }

    // if is_ascending {
    //     indices.sort_by(|&a, &b| units[a].max_power.partial_cmp(&units[b].max_power).unwrap());
    // } else {
    //     indices.sort_by(|&a, &b| units[b].max_power.partial_cmp(&units[a].max_power).unwrap());
    // }
    indices
}

pub fn lambda_iteration(
    status: &[bool],
    units: &[Unit],
    demand: &[f64],
    n_hours: usize,
    n_units: usize,
) -> Vec<f64> {
    let mut power: Vec<f64> = vec![0.0; n_hours * n_units];

    for h in 0..n_hours {
        let mut power_demand: f64 = demand[h];

        let mut lambda: f64 = 0.0;
        for u in 0..n_units {
            if lambda < units[u].b && status[h * n_units + u] {
                lambda = units[u].b;
            }
        }

        while f64::abs(power_demand) > 0.001 {
            let mut multiplier: Vec<f64> = vec![0.0; n_units];
            let mut sum_p: f64 = 0.0;
            let mut sum_c: f64 = 0.0;

            for u in 0..n_units {
                if status[h * n_units + u] {
                    multiplier[u] = (lambda - units[u].b) / 2.0;
                    power[h * n_units + u] = multiplier[u] / units[u].c;
                    power[h * n_units + u] = f64::min(power[h * n_units + u], units[u].max_power);
                    power[h * n_units + u] = f64::max(power[h * n_units + u], units[u].min_power);
                    sum_p += power[h * n_units + u];
                    sum_c += 1.0 / units[u].c;
                }
            }

            power_demand = demand[h] - sum_p;
            lambda = lambda + power_demand * 2.0 / sum_c;
        }
    }

    power
}

pub fn save_schedule(schedule: &Schedule, n_hours: usize, n_units: usize, demand: &[f64]) {
    let mut state = String::with_capacity(5000);
    for j in 0..n_units + 1 {
        if j == 0 {
            state.push_str("Hour,");
            // state.push(',');
        } else {
            state.push_str("Unit ");
            state.push_str(&j.to_string());
            if j == n_units {
                state.push('\n')
            } else {
                state.push(',')
            };
        }
    }
    for i in 0..n_hours {
        state.push_str(&(i + 1).to_string());
        state.push(',');
        for j in 0..n_units {
            if schedule.status[i * n_units + j] {
                state.push('1')
            } else {
                state.push('0')
            };
            if j == n_units - 1 {
                state.push('\n')
            } else {
                state.push(',')
            };
        }
    }

    let mut status_file = String::with_capacity(64);
    status_file.push_str("./results/");
    status_file.push_str(&n_units.to_string());
    status_file.push_str("_status.csv");
    fs::write(status_file, state).expect("Unable to write file");

    let mut power = String::with_capacity(50000);
    for j in 0..n_units + 1 {
        if j == 0 {
            power.push_str("Hour,");
            // power.push(',');
        } else {
            power.push_str("Unit ");
            power.push_str(&j.to_string());
            power.push(',');
        }
    }
    power.push_str("Load (MW),Demand (MW),Fuel ($/h),Startup ($/h),Total ($/h)\n");

    for i in 0..n_hours {
        power.push_str(&(i + 1).to_string());
        power.push(',');
        for j in 0..n_units {
            power
                .push_str(&((schedule.power[i * n_units + j] * 100.0).trunc() / 100.0).to_string());
            power.push(',');
        }
        power.push_str(
            &((sum_vec(&schedule.power[(i * n_units)..(i * n_units + n_units)]) * 100.0).trunc()
                / 100.0)
                .to_string(),
        );
        power.push(',');
        power.push_str(&((demand[i] * 100.0).trunc() / 100.0).to_string());
        power.push(',');
        power.push_str(&((schedule.fuel[i] * 100.0).trunc() / 100.0).to_string());
        power.push(',');
        power.push_str(&((schedule.startup[i] * 100.0).trunc() / 100.0).to_string());
        power.push(',');
        power.push_str(
            &(((schedule.fuel[i] + schedule.startup[i]) * 100.0).trunc() / 100.0).to_string(),
        );
        power.push('\n');
    }

    let mut power_file = String::with_capacity(64);
    power_file.push_str("./results/");
    power_file.push_str(&n_units.to_string());
    power_file.push_str("_power.csv");
    fs::write(power_file, power).expect("Unable to write file");
}

pub fn print_schedule(
    schedule: &Schedule,
    n_hours: usize,
    n_units: usize,
    units: &[Unit],
    demand: &[f64],
) {
    print!("----");
    for _ in 0..(n_units + 3) {
        print!("---------");
    }
    println!();
    print!(" S |");
    for j in 0..n_units {
        print!("{:9}", j);
    }
    println!("       CR       SR  Satisfy");
    print!("----");
    for _ in 0..(n_units + 3) {
        print!("---------");
    }
    println!();
    for i in 0..n_hours {
        print!("{:2} |", i);
        let mut sum: f64 = 0.0;
        for j in 0..n_units {
            sum += if schedule.status[i * n_units + j] {
                units[j].max_power
            } else {
                0.0
            };
            print!(
                "{:9}",
                if schedule.status[i * n_units + j] {
                    1
                } else {
                    0
                }
            );
        }
        println!(
            "{:9.2}{:9.2}{:9}",
            sum,
            demand[i] * SPINNING_RESERVE,
            if sum >= demand[i] * SPINNING_RESERVE {
                1
            } else {
                0
            }
        );
    }

    print!("----");
    for _ in 0..(n_units + 5) {
        print!("---------");
    }
    println!();
    print!(" P |");
    for j in 0..n_units {
        print!("{:9}", j);
    }
    println!("     Load   Demand     Fuel  Startup    Total");
    print!("----");
    for _ in 0..(n_units + 5) {
        print!("---------");
    }
    println!();
    for i in 0..n_hours {
        print!("{:2} |", i);
        for j in 0..n_units {
            print!("{:9.2}", schedule.power[i * n_units + j]);
        }
        println!(
            "{:9.2}{:9.2}{:9.2}{:9.2}{:9.2}",
            sum_vec(&schedule.power[(i * n_units)..(i * n_units + n_units)]),
            demand[i],
            schedule.fuel[i],
            schedule.startup[i],
            schedule.fuel[i] + schedule.startup[i]
        );
    }

    print!("----");
    for _ in 0..(n_units + 5) {
        print!("---------");
    }
    println!();
    println!(
        "Fuel: {:.2}, Startup: {:.2}, Total Cost: {:.2}",
        sum_vec(&schedule.fuel),
        sum_vec(&schedule.startup),
        schedule.cost
    );
    print!("----");
    for _ in 0..(n_units + 5) {
        print!("---------");
    }
    println!();
}

pub fn sum_vec(input: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;
    for i in input.iter() {
        sum += i;
    }
    sum
}
