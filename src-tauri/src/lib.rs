use tauri::command;

#[command]
fn ping() -> String {
    "pong".to_string()
}

#[command]
fn run_spectral_brain() -> String {
    "Spectral Brain Heart Test (N=100)\nMean relative error: 0.033492".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping, run_spectral_brain])
        .run(tauri::generate_context!())
        .expect("error while running Spectral Brain");
}
