pub fn normalize(a: f64, b: f64) -> (f64, f64) {
    let total = a + b;
    (a / total, b / total)
}
