#[derive(Clone)]
pub struct Unit {
    pub max_power: f64,
    pub min_power: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub min_uptime: isize,
    pub min_downtime: isize,
    pub hot_start_cost: f64,
    pub cold_start_cost: f64,
    pub cold_start_hours: isize,
    pub initial_status: isize
}

pub fn get_four_units() -> (Vec<Unit>, Vec<f64>) {
    (
        vec![
            Unit {
                max_power: 300.0,
                min_power: 75.0,
                a: 648.74,
                b: 16.83,
                c: 0.0021,
                min_uptime: 4,
                min_downtime: 5,
                hot_start_cost: 500.0,
                cold_start_cost: 1100.0,
                cold_start_hours: 5,
                initial_status: 8
            },
            Unit {
                max_power: 250.0,
                min_power: 60.0,
                a: 585.62,
                b: 16.95,
                c: 0.0042,
                min_uptime: 3,
                min_downtime: 5,
                hot_start_cost: 170.0,
                cold_start_cost: 400.0,
                cold_start_hours: 5,
                initial_status: 8
            },
            Unit {
                max_power: 80.0,
                min_power: 25.0,
                a: 213.0,
                b: 20.74,
                c: 0.0018,
                min_uptime: 2,
                min_downtime: 4,
                hot_start_cost: 150.0,
                cold_start_cost: 350.0,
                cold_start_hours: 4,
                initial_status: -5
            },
            Unit {
                max_power: 60.0,
                min_power: 20.0,
                a: 252.0,
                b: 23.6,
                c: 0.0034,
                min_uptime: 1,
                min_downtime: 1,
                hot_start_cost: 0.0,
                cold_start_cost: 0.02,
                cold_start_hours: 0,
                initial_status: -6
            },
        ],
        vec![450.0, 530.0, 600.0, 540.0, 400.0, 280.0, 290.0, 500.0],
    )
}

pub fn get_ten_units() -> (Vec<Unit>, Vec<f64>) {
    (
        vec![
            Unit {
                // Unit 1
                max_power: 455.0,
                min_power: 150.0,
                a: 1000.0,
                b: 16.19,
                c: 0.00048,
                min_uptime: 8,
                min_downtime: 8,
                hot_start_cost: 4500.0,
                cold_start_cost: 9000.0,
                cold_start_hours: 5,
                initial_status: 8
            },
            Unit {
                // Unit 2
                max_power: 455.0,
                min_power: 150.0,
                a: 970.0,
                b: 17.26,
                c: 0.00031,
                min_uptime: 8,
                min_downtime: 8,
                hot_start_cost: 5000.0,
                cold_start_cost: 10000.0,
                cold_start_hours: 5,
                initial_status: 8
            },
            Unit {
                // Unit 3
                max_power: 130.0,
                min_power: 20.0,
                a: 700.0,
                b: 16.60,
                c: 0.002,
                min_uptime: 5,
                min_downtime: 5,
                hot_start_cost: 550.0,
                cold_start_cost: 1100.0,
                cold_start_hours: 4,
                initial_status: -5
            },
            Unit {
                // Unit 4
                max_power: 130.0,
                min_power: 20.0,
                a: 680.0,
                b: 16.50,
                c: 0.00211,
                min_uptime: 5,
                min_downtime: 5,
                hot_start_cost: 560.0,
                cold_start_cost: 1120.0,
                cold_start_hours: 4,
                initial_status: -5
            },
            Unit {
                // Unit 5
                max_power: 162.0,
                min_power: 25.0,
                a: 450.0,
                b: 19.70,
                c: 0.00398,
                min_uptime: 6,
                min_downtime: 6,
                hot_start_cost: 900.0,
                cold_start_cost: 1800.0,
                cold_start_hours: 4,
                initial_status: -6
            },
            Unit {
                // Unit 6
                max_power: 80.0,
                min_power: 20.0,
                a: 370.0,
                b: 22.26,
                c: 0.00712,
                min_uptime: 3,
                min_downtime: 3,
                hot_start_cost: 170.0,
                cold_start_cost: 340.0,
                cold_start_hours: 2,
                initial_status: -3
            },
            Unit {
                // Unit 7
                max_power: 85.0,
                min_power: 25.0,
                a: 480.0,
                b: 27.74,
                c: 0.00079,
                min_uptime: 3,
                min_downtime: 3,
                hot_start_cost: 260.0,
                cold_start_cost: 520.0,
                cold_start_hours: 2,
                initial_status: -3
            },
            Unit {
                // Unit 8
                max_power: 55.0,
                min_power: 10.0,
                a: 660.0,
                b: 25.92,
                c: 0.00413,
                min_uptime: 1,
                min_downtime: 1,
                hot_start_cost: 30.0,
                cold_start_cost: 60.0,
                cold_start_hours: 0,
                initial_status: -1
            },
            Unit {
                // Unit 9
                max_power: 55.0,
                min_power: 10.0,
                a: 665.0,
                b: 27.27,
                c: 0.00222,
                min_uptime: 1,
                min_downtime: 1,
                hot_start_cost: 30.0,
                cold_start_cost: 60.0,
                cold_start_hours: 0,
                initial_status: -1
            },
            Unit {
                // Unit 10
                max_power: 55.0,
                min_power: 10.0,
                a: 670.0,
                b: 27.79,
                c: 0.00173,
                min_uptime: 1,
                min_downtime: 1,
                hot_start_cost: 30.0,
                cold_start_cost: 60.0,
                cold_start_hours: 0,
                initial_status: -1
            },
        ],
        vec![
            // Demand for 24-hours
            700.0, 750.0, 850.0, 950.0, 1000.0, 1100.0, 1150.0, 1200.0,
            1300.0, 1400.0, 1450.0, 1500.0, 1400.0, 1300.0, 1200.0, 1050.0,
            1000.0, 1100.0, 1200.0, 1400.0, 1300.0, 1100.0, 900.0, 800.0
        ],
    )
}

pub fn get_ten_x_units(x: usize) -> (Vec<Unit>, Vec<f64>) {
    let (ten_units, mut demand) = get_ten_units();
    let mut ten_x_units: Vec<Unit> = Vec::with_capacity(ten_units.len() * x);

    for _ in 0..x {
        for j in 0..ten_units.len() {
            ten_x_units.push(ten_units[j].clone());
        }
    }

    for i in 0..demand.len() {
        demand[i] *= x as f64;
    }

    (ten_x_units, demand)
}