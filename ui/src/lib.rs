#![allow(non_snake_case)]

use dioxus::prelude::*;
use engine::models::HeartSystem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Shared state
    let mut system = use_signal(|| HeartSystem::new());
    
    // Sliders state
    let mut ko = use_signal(|| 5.4);
    let mut cao = use_signal(|| 2.0);
    let mut nao = use_signal(|| 140.0);
    let mut block_na = use_signal(|| 0.0);
    let mut block_k = use_signal(|| 0.0);
    let mut block_ca = use_signal(|| 0.0);
    let mut block_nak = use_signal(|| 0.0);
    let mut symp = use_signal(|| 0.0);
    let mut parasymp = use_signal(|| 0.0);
    let mut isch = use_signal(|| 0.0);

    // Canvas references
    let canvas_pa = use_signal(|| None::<web_sys::HtmlCanvasElement>);
    
    use_future(move || async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(16).await;
            
            // 1. Update Parameters from UI
            let b_na = 1.0 - (block_na() / 100.0);
            let b_k = 1.0 - (block_k() / 100.0);
            let b_ca = 1.0 - (block_ca() / 100.0);
            let b_nak = 1.0 - (block_nak() / 100.0);
            let s_symp = symp() / 100.0;
            let s_parasymp = parasymp() / 100.0;
            let s_isch = isch() / 100.0;

            system.write().update_params(
                ko(), cao(), nao(), 
                b_na, b_k, b_ca, b_nak, 
                s_symp, s_parasymp, s_isch
            );
            
            // 2. Step Engine
            let dt = 0.01;
            let steps = 1600;
            let downsample = 10;
            let _batch = system.write().run_batch(dt, steps, downsample);
            
            // 3. Draw to Canvas (TODO)
            // For now, we just ensure the loop runs natively in Rust!
        }
    });

    rsx! {
        link { rel: "stylesheet", href: "assets/style.css" }
        
        header {
            div { class: "hud-item", "BPM: ", span { id: "hud-bpm", "75" } }
            div { class: "hud-item", "PR: ", span { id: "hud-pr", "160ms" } }
            div { class: "hud-item", "QRS: ", span { id: "hud-qrs", "90ms" } }
            div { class: "hud-item", "QT: ", span { id: "hud-qt", "400ms" } }
            div { class: "hud-item", "V_rest: ", span { id: "hud-vrest", "-85mV" } }
        }

        aside { id: "controls-sidebar",
            div { class: "control-group",
                h3 { "Íons Extracelulares" }
                div { class: "accordion-content active",
                    Slider { label: "K+ (Potássio)", min: 2.0, max: 10.0, step: 0.1, val: ko }
                    Slider { label: "Ca2+ (Cálcio)", min: 0.5, max: 5.0, step: 0.1, val: cao }
                    Slider { label: "Na+ (Sódio)", min: 100.0, max: 160.0, step: 1.0, val: nao }
                }
            }
            div { class: "control-group",
                h3 { "Fármacos" }
                div { class: "accordion-content active",
                    Slider { label: "Lidocaína (Na+)", min: 0.0, max: 100.0, step: 1.0, val: block_na }
                    Slider { label: "Amiodarona (K+)", min: 0.0, max: 100.0, step: 1.0, val: block_k }
                    Slider { label: "Verapamil (Ca2+)", min: 0.0, max: 100.0, step: 1.0, val: block_ca }
                }
            }
            div { class: "control-group",
                h3 { "Sistema Autônomo" }
                div { class: "accordion-content active",
                    Slider { label: "Tônus Simpático", min: 0.0, max: 100.0, step: 1.0, val: symp }
                    Slider { label: "Tônus Parassimpático", min: 0.0, max: 100.0, step: 1.0, val: parasymp }
                    Slider { label: "Isquemia Local", min: 0.0, max: 100.0, step: 1.0, val: isch }
                }
            }
        }

        main {
            div { class: "canvas-wrapper",
                div { class: "canvas-label", "Potencial de Ação Celular" }
                canvas { id: "canvas-pa" }
            }
            div { class: "canvas-wrapper",
                div { class: "canvas-label", "DII (ECG)" }
                canvas { id: "canvas-ecg" }
            }
            div { class: "canvas-wrapper",
                div { class: "canvas-label", "Correntes (Na, Ca, K)" }
                canvas { id: "canvas-ch3" }
            }
            div { class: "canvas-wrapper",
                div { class: "canvas-label", "Concentrações Intracelulares" }
                canvas { id: "canvas-ch4" }
            }
        }
    }
}

#[component]
fn Slider(label: String, min: f64, max: f64, step: f64, val: Signal<f64>) -> Element {
    rsx! {
        div { class: "slider-container",
            div { class: "slider-header",
                span { "{label}" }
                span { "{val():.1}" }
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{val()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        val.set(v);
                    }
                }
            }
        }
    }
}
