extern crate num_complex;
extern crate num_traits;
extern crate town_map_lib;

use num_complex::Complex;
use num_traits::Float;
use std::f64::consts;
use town_map_lib::Polar;

fn fun(z: Complex<f64>) -> Complex<f64> {
    let on = Complex::from(1.0);
    let tw = Complex::from(2.0);
    let mut w = on;
    for _i in 0..10 {
        let wexp = w.exp();
        let fact = w * wexp - z;
        w = w - fact / (wexp * (w + on) - (w + tw) / (tw * w + tw) * fact)
    }

    w
}

fn pol(p: f64, t: u8) -> Polar<f64> {
    Polar {
        r: ((p + t as f64 * 2.0 * consts::PI) * 3.0).cos() * 2.0,
        theta: (p + t as f64 * 2.0 * consts::PI),
    }
}

fn main() {
    use town_map_lib::{Circle, Plot};
    let _circle = Circle { r: 40 };
    Plot::plot(&fun).unwrap();
}
