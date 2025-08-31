/// Integrates the given function `f_int` using the Midpoint Rule.
///
/// Args:
///     f_int: Routine integrating $ f(x) $.
///     a: Left endpoint of the integration interval.
///     b: Right endpoint of the integration interval.
///     n: Number of partition intervals.
///
/// Returns:
///     Midpoint Rule approximation of $ \int_a^b f(x) $.
fn midpoint_rule<F>(f_int: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / (n as f64);
    h * (1..=n)
        .map(|i| i as f64)
        .fold(0.0, |acc, i| acc + f_int(a + h * (i - 0.5)))
}

/// Integrates the given function `f_int` using the Trapezoidal Rule.
///
/// Args:
///     f_int: Routine integrating $ f(x) $.
///     a: Left endpoint of the integration interval.
///     b: Right endpoint of the integration interval.
///     n: Number of partition intervals.
///
/// Returns:
///     Trapezoidal Rule approximation of $ \int_a^b f(x) $.
fn trapezoidal_rule<F>(f_int: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / (n as f64);
    let mut result = 0.5 * (f_int(a) + f_int(b));
    for i in (1..n).map(|i| i as f64) {
        result += f_int(a + h * i);
    }
    h * result
}

/// Integrates the given function `f_int` using the Simpson's Rule.
///
/// Args:
///     f_int: Routine integrating $ f(x) $.
///     a: Left endpoint of the integration interval.
///     b: Right endpoint of the integration interval.
///     n: Number of partition intervals.
///
/// Returns:
///     Simpson's Rule approximation of $ \int_a^b f(x) $.
fn simpsons_rule<F>(f_int: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / (n as f64);
    let mut result = (f_int(a) + f_int(b)) / 6.0;
    for i in (1..n).map(|i| i as f64) {
        result += f_int(a + h * i) / 3.0;
    }
    for i in (1..=n).map(|i| i as f64) {
        result += 2.0 * f_int(a + h * (i - 0.5)) / 3.0;
    }
    h * result
}

/// Computes an approximate value of an integral within the given tolerance
/// `tol`.
///
/// Args:
///     tol: Tolerance.
///     i_numerical: Returns the result of the numerical integration rule with
///         n intervals.
///
/// Returns:
///     Approximation of \int_a^b f(x) with tolerance of `tol`.
fn stopping_criterion<N>(tol: f64, i_numerical: N) -> f64
where
    N: Fn(usize) -> f64,
{
    let mut n = 4;
    let mut result = i_numerical(n);

    while let new_result = i_numerical(2 * n)
        && (new_result - result).abs() > tol
    {
        println!("{n}\t{result}");
        n *= 2;
        result = new_result;
    }

    println!("{n}\t{result}");
    result
}

/// Evaluates $ e^{-x^2} $.
fn gaussian(x: f64) -> f64 {
    (-x * x).exp()
}

fn partial<R>(rule: R) -> impl Fn(usize) -> f64
where
    R: Fn(fn(f64) -> f64, f64, f64, usize) -> f64 + Copy,
{
    move |n| rule(gaussian, 0.0, 2.0, n)
}

fn main() {
    println!("{}", midpoint_rule(gaussian, 0.0, 2.0, 4));
    println!("{}", trapezoidal_rule(gaussian, 0.0, 2.0, 4));
    println!("{}", simpsons_rule(gaussian, 0.0, 2.0, 4));
    stopping_criterion(0.5e-7, partial(midpoint_rule));
    stopping_criterion(0.5e-7, partial(trapezoidal_rule));
    stopping_criterion(0.5e-7, partial(simpsons_rule));
}
