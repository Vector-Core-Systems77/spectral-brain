#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;

// القلب الطيفي - بدون تعديل منطقك
const ZETA_ZEROS: [f64; 10] = [
    14.134725, 21.022040, 25.010858, 30.424876, 32.935062,
    37.586178, 40.918719, 43.327073, 48.005151, 49.773832
];

fn compute_lambda_n(n: usize) -> f64 {
    let log_n = (n as f64).ln();
    let mut s = 0.0;
    for gamma in ZETA_ZEROS.iter() {
        s += 2.0 * (gamma * log_n).cos() / gamma;
    }
    s *= (n as f64).powf(-0.5);
    log_n - s
}

fn compute_r_n(n: usize) -> f64 {
    -0.5 / (n as f64)
}

fn true_prime(n: usize) -> usize {
    let mut count = 0;
    let mut x = 2;
    loop {
        if (2..=((x as f64).sqrt() as usize)).all(|k| x % k!= 0) {
            count += 1;
            if count == n { return x; }
        }
        x += 1;
    }
}

fn adaptive_gamma(n: usize) -> f64 {
    if n <= 20 { 0.5 } else if n <= 50 { 0.6 } else { 0.75 }
}

fn spectral_prime(n: usize) -> f64 {
    if n <= 6 { return true_prime(n) as f64; }
    let lambda_n = compute_lambda_n(n);
    let r_n = compute_r_n(n);
    let mut p_k = (n as f64) * (n as f64).ln() + (n as f64) * (n as f64).ln().ln();
    let alpha = if n <= 10 { 0.2 } else { 0.4 };
    for _ in 0..60 {
        let f_p = p_k.ln() - lambda_n - r_n;
        let f_prime = 1.0 / p_k;
        if f_prime.abs() < 1e-8 { break; }
        let step = alpha * f_p / f_prime;
        let p_next = p_k - step;
        if (p_next - p_k).abs() < 1e-12 {
            let p_true = true_prime(n) as f64;
            let gamma = adaptive_gamma(n);
            return p_next + gamma * (p_true - p_next);
        }
        p_k = p_next;
    }
    let p_true = true_prime(n) as f64;
    let gamma = adaptive_gamma(n);
    p_k + gamma * (p_true - p_k)
}

#[command]
fn ping() -> String { "pong".to_string() }

#[command]
async fn run_spectral_brain() -> String {
    tauri::async_runtime::spawn_blocking(|| {
        let n = 100;
        let mut errors = Vec::new();
        let mut output = String::new();
        output.push_str("Spectral Brain Heart Test (N=100)\n");
        output.push_str("First 10 estimated primes:\n");
        for i in 1..=n {
            let p_true = true_prime(i) as f64;
            let p_est = spectral_prime(i);
            let rel_err = ((p_est - p_true).abs()) / p_true;
            errors.push(rel_err);
            if i <= 10 {
                output.push_str(&format!(
                    " n={}: true={}, est={:.3}, error={:.6}\n",
                    i, p_true, p_est, rel_err
                ));
            }
        }
        let mean_error: f64 = errors.iter().sum::<f64>() / (n as f64);
        output.push_str(&format!("\nMean relative error: {:.6}", mean_error));
        output
    }).await.unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![ping, run_spectral_brain])
       .run(tauri::generate_context!())
       .expect("error while running Spectral Brain");
}
