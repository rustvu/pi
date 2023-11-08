use rand::random;

fn functional_pi(n: usize) -> f64 {
    let k = (0..n)
        .filter(|_| (random::<f64>().powi(2) + random::<f64>().powi(2)) <= 1.0)
        .count();

    4.0 * k as f64 / n as f64
}

fn imperative_pi(n: usize) -> f64 {
    let mut k = 0;
    for _ in 0..n {
        let x = random::<f64>();
        let y = random::<f64>();
        if x * x + y * y <= 1.0 {
            k += 1;
        }
    }

    4.0 * k as f64 / n as f64
}

fn main() {
    println!("(imperative) π = {}", imperative_pi(1_000_000));
    println!("(functional) π = {}", functional_pi(1_000_000));
}
