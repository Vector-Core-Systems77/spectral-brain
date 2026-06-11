import { invoke } from '@tauri-apps/api/core';

const output = document.getElementById("output");

document.getElementById("ping").addEventListener("click", async () => {
  output.textContent = "Testing connection...";
  try {
    const result = await invoke("ping");
    output.textContent = "Response: " + result;
  } catch (err) {
    output.textContent = "Error: " + err;
  }
});

document.getElementById("run").addEventListener("click", async () => {
  output.textContent = "Running Spectral Reasoning...";
  try {
    const result = await invoke("run_spectral_brain");
    output.textContent = result;
  } catch (err) {
    output.textContent = "Error: " + err;
  }
});
