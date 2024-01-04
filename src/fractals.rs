pub fn julia(x: f64, y: f64, max_iterations: usize) -> f64 {
    const REAL: f64 = -0.8;
    const IMAG: f64 = 0.156;

    let mut iterations = 0;
    let mut r = x;
    let mut i = y;
    let mut mod_ = r * r + i * i;
    while mod_ < 4.0 && iterations < max_iterations {
        let tmp = r;
        r = r * r - i * i + REAL;
        i = 2.0 * i * tmp + IMAG;
        mod_ = r * r + i * i;
        iterations += 1;
    }
    iterations as f64 - f64::log2(f64::max(1.0, f64::log2(mod_)))
}

pub fn mandelbrot(x: f64, y: f64, max_iterations: usize) -> f64 {
    let real = -2.0 + x * 4.0;
    let imag = -2.0 + y * 4.0;

    let mut iterations = 0;
    let mut  r = 0.0;
    let mut  i = 0.0;
    let mut mod_ = r * r + i * i;
    while iterations < max_iterations && mod_ < 4.0 {
        let temp = r * r - i * i + real;
        i = 2.0 * r * i + imag;
        r = temp;
        mod_ = r * r + i * i;
        iterations += 1;
    }

    iterations as f64 - f64::log2(f64::max(1.0, f64::log2(mod_)))
}

pub fn burning_ship(x: f64, y: f64, max_iterations: usize) -> f64 {
    let real = x;
    let imag = y;

    let mut iterations = 0;
    let mut r = 0.0;
    let mut i = 0.0;
    let mut mod_ = r * r + i * i;

    while iterations < max_iterations && mod_ < 4.0 {
        let temp = f64::abs(r * r - i * i) + real;
        i = 2.0 * f64::abs(r * i) + imag;
        r = temp;
        mod_ = r * r + i * i;
        iterations += 1;
    }

    iterations as f64 - f64::log2(f64::max(1.0, f64::log2(mod_)))
}

pub fn tricorn(x: f64, y: f64, max_iterations: usize) -> f64 {
    let real = x;
    let imag = y;

    let mut iterations = 0;
    let mut r = 0.0;
    let mut i = 0.0;
    let mut mod_ = r * r + i * i;

    while iterations < max_iterations && mod_ < 4.0 {
        let temp = r * r - i * i + real;
        i = -2.0 * r * i + imag;
        r = temp;
        mod_ = r * r + i * i;
        iterations += 1;
    }

    iterations as f64 - f64::log2(f64::max(1.0, f64::log2(mod_)))
}