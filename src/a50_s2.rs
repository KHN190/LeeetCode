// recursive log(n)
pub fn my_pow(x: f64, n: i32) -> f64 {
    use std::i32::MIN;
    match n {
        o if o == 0 => 1.0,
        o if o == MIN => my_pow(x * x, MIN / 2),
        o if o < 0 => 1.0 / my_pow(x, -n),
        _ => match n % 2 {
            0 => my_pow(x * x, n / 2),
            _ => x * my_pow(x * x, n / 2),
        },
    }
}

#[test]
fn run() {
    assert_eq!(my_pow(2.0, 10), 1024.0);
    assert_eq!(my_pow(2.0, -2), 0.25);
}
