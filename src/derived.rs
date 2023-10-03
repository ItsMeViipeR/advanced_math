pub fn derived(f: fn(f64) -> f64, x: f64, h: f64) -> String {
    let df = (f(x + h) - f(x)) / h;
    return format!("{:.0}x", df);
}