pub mod models;
use wasm_bindgen::prelude::*;

// Esta macro diz ao compilador para exportar essa função para o JavaScript
#[wasm_bindgen]
pub fn init_engine() -> String {
    "KokoroSim WASM Engine Inicializada com Sucesso!".to_string()
}

// Uma função de teste para garantir que a matemática roda e se comunica
#[wasm_bindgen]
pub fn test_math(a: f64, b: f64) -> f64 {
    a + b
}
