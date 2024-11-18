use rand::Rng;


#[derive(Clone)]
pub struct Solution{
    pub value: Vec<f64>,
    pub cost: f64
}

impl Solution {
    pub fn empty(dim: usize, is_cost_infinity: bool) -> Solution {
        Solution {
            value: vec![0.0; dim],
            cost: if is_cost_infinity {f64::INFINITY} else {0.0}
        }
    }
}

// pub fn transfer(
//     x: f64,
//     rng: &mut rand::rngs::ThreadRng
// ) -> bool {

//     let r: f64 = rng.gen::<f64>();
//     let y: f64 = 1.0 / (1.0 + f64::exp(-10.0 * (x - 0.5)));
//     if y > r {true} else {false}
// }

pub fn transfer(
    x: f64,
    alpha: f64,
    iter: usize,
    max_iter: usize,
    rng: &mut rand::rngs::ThreadRng
) -> bool {

    let r1: f64 = rng.gen::<f64>();


    // let r2: f64 = rng.gen::<f64>();
    // let r3: f64 = rng.gen::<f64>();
    // let a1: f64 = 2.0 - iter as f64 * (2.0 / max_iter as f64);
    // let a2: f64 = -1.0 + iter as f64 * (-1.0 / max_iter as f64);
    // let a3: f64 = 2.0 * a1 * r2 - a1;

    // let c2: f64 = 2.0 * r3;

    // let y: f64 = 1.0 / (1.0 + f64::exp(-10.0 * (a3 * (c2 * alpha - x).abs() - 0.5)));

    // let z_max: f64 = 100.0;
    // let z_min: f64 = 5.0;
    // let z: f64 = (z_max - z_min) * (1.0 - iter as f64 / max_iter as f64) + z_min;


    // let y: f64 = 1.0 / (1.0 + f64::exp(-z * (x - 0.5)));


    /* Double U-shape inverted */
    // let y: f64 = f64::exp(-4.0 * f64::exp(-f64::powi(10.0 * f64::powi(a3 * c2 * (alpha - x) - 0.5, 2), -2)));
    
    // // let y: f64 = f64::exp(-1.0 / (100.0 * f64::powi(x - 0.5, 2)));
    // let y: f64 = 1.0 - f64::exp(-1.0 / (z * f64::powi(x - 0.5, 2)));

    // let y: f64 = f64::exp(-10.0 / (1.0 * f64::powi(2.0 * x - 1.0, -4)));
    // let y: f64 = f64::exp(-10.0 / (1.0 * f64::powi(c2 * (a3 * alpha - x).abs() - 1.0, -4)));


    // let y: f64 = 0.5 * (f64::tanh(z * x - z / 2.0) + 1.0);

    // let y: f64 = f64::ln(10.0 * x) * 0.3;
    // let y: f64 = -f64::exp(-15.0 * x) + 1.0;
    // let y: f64 = 0.5 * (f64::tanh(10.0 * x - 4.0) + 1.0);
    // let y: f64 = -4.0 * ((1.0 - max_iter as f64 / iter as f64) * x - (1.0 - max_iter as f64 / iter as f64) / 2.0) * ((1.0 - max_iter as f64 / iter as f64) * x - (1.0 - max_iter as f64 / iter as f64) / 2.0) + 1.0;
    
    // let y: f64 = f64::abs(f64::tanh(20.0 * f64::powi(x - 0.5, 3))); /* U shape */
    // let y: f64 = f64::abs(f64::tanh(5.0 * (x - 0.5))); /* V-Shape */


    // let y: f64 = 1.0 / (1.0 + f64::exp(-10.0 * (x - 0.5))); /* S-shape, alpha = 10 */
    let y: f64 = 1.0 / (1.0 + f64::exp(-1000.0 * (x - 0.5))); /* S-shape, alpha = 1000 */
    // let y: f64 = 1.0 - f64::abs(f64::tanh(20.0 * f64::powi(x - 0.5, 3))); /* Inverse U shape */
    // let y: f64 = 1.0 - f64::abs(f64::tanh(5.0 * (x - 0.5))); /* Inverse V-Shape */



    // let m: f64 = (MUTATION_RATE_MAX - MUTATION_RATE_MIN) * (iter as f64 / max_iter as f64) + MUTATION_RATE_MIN;
    // if y > r1 {
    //     if r2 < m {false} else {true}
    // } else {
    //     if r2 < m {true} else {false}
    // }

    if y > r1 {true} else {false}
}
