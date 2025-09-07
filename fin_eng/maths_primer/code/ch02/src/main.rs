use ch02::integrate::{midpoint_rule, simpsons_rule, stopping_criterion, trapezoidal_rule};

fn q3() {
    /// Evaluates $ \sqrt{x} e^{-x} $.
    fn integrand(x: f64) -> f64 {
        x.sqrt() * (-x).exp()
    }

    fn partial<R>(rule: R) -> impl Fn(usize) -> f64
    where
        R: Fn(fn(f64) -> f64, f64, f64, usize) -> f64 + Copy,
    {
        move |n| rule(integrand, 1.0, 3.0, n)
    }

    println!("Midpoint");
    stopping_criterion(1e-6, partial(midpoint_rule));
    println!("Trapezoidal");
    stopping_criterion(1e-6, partial(trapezoidal_rule));
    println!("Simpson's");
    stopping_criterion(1e-6, partial(simpsons_rule));
}

fn q4() {
    /// Evaluates $ \frac{x^{5/2}}{1 + x^2} $.
    fn integrand(x: f64) -> f64 {
        x.powf(2.5) / (1.0 + x * x)
    }

    fn partial<R>(rule: R) -> impl Fn(usize) -> f64
    where
        R: Fn(fn(f64) -> f64, f64, f64, usize) -> f64 + Copy,
    {
        move |n| rule(integrand, 0.0, 1.0, n)
    }

    println!("Midpoint");
    stopping_criterion(1e-6, partial(midpoint_rule));
    println!("Simpson's");
    stopping_criterion(1e-7, partial(simpsons_rule));
}

fn main() {
    // q3();
    q4();
}
