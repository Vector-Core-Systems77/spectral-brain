#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

// ============================================
// 1. ترميز النص ↔ الأعداد الأولية
// ============================================
fn char_primes(c: char) -> Option<u64> {
    match c {
        'h' => Some(19),
        'e' => Some(11),
        'l' => Some(37),
        'o' => Some(47),
        _ => None,
    }
}

fn prime_chars(p: u64) -> Option<char> {
    match p {
        19 => Some('h'),
        11 => Some('e'),
        37 => Some('l'),
        47 => Some('o'),
        _ => None,
    }
}

fn text_to_prime_vector(text: &str) -> Vec<u64> {
    text.chars().filter_map(char_primes).collect()
}

fn prime_vector_to_text(primes: &[u64]) -> String {
    primes.iter().filter_map(|p| prime_chars(*p)).collect()
}

// ============================================
// 2. Spectral Kernel (نسخة مبسطة)
// ============================================
fn prime_index_approx(p: f64) -> i64 {
    if p <= 2.0 { return 1; }
    (p / p.ln()) as i64
}

fn spectral_prime_estimate(n: i64) -> f64 {
    if n <= 1 { return 2.0; }
    let nf = n as f64;
    nf * nf.ln()
}

fn spectral_kernel(p_i: u64, p_j: u64) -> f64 {
    let ni = prime_index_approx(p_i as f64);
    let nj = prime_index_approx(p_j as f64);
    if ni <= 0 || nj <= 0 { return 0.0; }
    let pi_spec = spectral_prime_estimate(ni);
    let pj_spec = spectral_prime_estimate(nj);
    let dist = (pi_spec - pj_spec).abs();
    1.0 / (1.0 + dist)
}

// ============================================
// 3. Spectral Neuron + Spectral Layer
// ============================================
struct SpectralNeuron {
    p_id: u64,
}

impl SpectralNeuron {
    fn new(p_id: u64) -> Self {
        Self { p_id }
    }

    fn activate(&self, prime_inputs: &[u64]) -> (Option<f64>, f64) {
        let mut total_resonance = 0.0;
        for p_in in prime_inputs {
            total_resonance += spectral_kernel(*p_in, self.p_id);
        }
        let activity = total_resonance;
        if activity <= 0.0 {
            return (None, activity);
        }
        let n_new = if activity > 1.0 { activity as i64 } else { 2 };
        let p_new = spectral_prime_estimate(n_new);
        (Some(p_new), activity)
    }
}

struct SpectralLayer {
    neurons: Vec<SpectralNeuron>,
}

impl SpectralLayer {
    fn new(prime_ids: &[u64]) -> Self {
        let neurons = prime_ids.iter().map(|p| SpectralNeuron::new(*p)).collect();
        Self { neurons }
    }

    fn forward(&self, prime_inputs: &[u64]) -> Vec<f64> {
        let mut outputs = Vec::new();
        for neuron in &self.neurons {
            if let (Some(p_out), _) = neuron.activate(prime_inputs) {
                outputs.push(p_out);
            }
        }
        outputs
    }
}

// ============================================
// 4. المسار الكامل
// ============================================
fn spectral_reasoning_pipeline(input_text: &str) -> String {
    let prime_in = text_to_prime_vector(input_text);
    let layer_prime_ids = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
    let layer = SpectralLayer::new(&layer_prime_ids);
    let prime_out_f = layer.forward(&prime_in);
    let prime_out_u: Vec<u64> = prime_out_f.iter().map(|v| v.round() as u64).collect();
    let output_text = prime_vector_to_text(&prime_out_u);

    format!(
        "input_text: {}\nprime_in: {:?}\nprime_out_raw: {:?}\nprime_out_rounded: {:?}\noutput_text: {}",
        input_text, prime_in, prime_out_f, prime_out_u, output_text
    )
}

// ============================================
// 5. Tauri command
// ============================================
#[command]
fn run_spectral_brain() -> String {
    spectral_reasoning_pipeline("hello")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_spectral_brain])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
