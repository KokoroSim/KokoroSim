#![allow(non_snake_case)]

use dioxus::prelude::*;
use engine::models::HeartSystem;
use wasm_bindgen::prelude::*;

mod plot;
use plot::Plotter;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
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
    
    let mut show_sa = use_signal(|| true);
    let mut show_av = use_signal(|| true);
    let mut show_atrium = use_signal(|| true);
    let mut show_vent = use_signal(|| true);
    
    use_future(move || async move {
        // Create plotters (Canvas IDs match the HTML)
        let mut pa_plotter_sa = Plotter::new("canvas-pa", 300); // Yellow for SA
        let mut pa_plotter_av = Plotter::new("canvas-pa", 300); // Purple for AV
        let mut pa_plotter_atrium = Plotter::new("canvas-pa", 300); // Pink for Atrium
        let mut pa_plotter_vent = Plotter::new("canvas-pa", 300); // Cyan for Ventricle
        let mut ecg_plotter = Plotter::new("canvas-ecg", 300); // Neon Green for ECG
        let mut ch3_plotter = Plotter::new("canvas-ch3", 300); // Channels
        let mut ch4_plotter = Plotter::new("canvas-ch4", 300); // Calcium

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
            let dt = 0.001; // reduced dt for numerical stability
            let steps = 16000; // 16ms of simulation at dt=0.001 (1x real-time speed)
            let downsample = 3333; // Saves 1 point per 3.33ms, fitting 1000ms inside the 300-capacity buffer
            let batch = system.write().run_batch(dt, steps, downsample);
            
            // Batch is flattened: [sa_v, av_v, atrium_v, ventricle_v, sa_v, ...]
            let chunk_size = 6;
            for chunk in batch.chunks(chunk_size) {
                if chunk.len() == 6 {
                    let sa = chunk[0];
                    let av = chunk[1];
                    let atrium = chunk[2];
                    let vent = chunk[3];
                    let cai = chunk[4];
                    let force = chunk[5];
                    
                    pa_plotter_sa.push(sa);
                    pa_plotter_av.push(av);
                    pa_plotter_atrium.push(atrium);
                    pa_plotter_vent.push(vent);

                    // Pseudo-ECG: Diferença entre Átrio e Ventrículo (simplificado)
                    let ecg = (atrium * 0.1) + (vent * 0.9);
                    ecg_plotter.push(ecg);
                    
                    ch3_plotter.push(cai);
                    ch4_plotter.push(force);
                }
            }

            // 3. Draw to Canvas
            // Prepare drawing flags to determine which one clears the canvas
            let any_vent = show_vent();
            let any_atrium = show_atrium();
            let any_av = show_av();
            let any_sa = show_sa();

            // First drawn line clears the canvas, subsequent ones draw on top
            let mut cleared = false;

            if any_vent {
                pa_plotter_vent.draw(-90.0, 50.0, "#00ffff", !cleared); 
                cleared = true;
            }
            if any_atrium {
                pa_plotter_atrium.draw(-90.0, 50.0, "#ff00ff", !cleared); 
                cleared = true;
            }
            if any_av {
                pa_plotter_av.draw(-90.0, 50.0, "#9b59b6", !cleared); 
                cleared = true;
            }
            if any_sa {
                pa_plotter_sa.draw(-90.0, 50.0, "#f1c40f", !cleared); 
                cleared = true;
            }

            // If nothing was drawn, clear it manually
            if !cleared {
                pa_plotter_vent.draw(-90.0, 50.0, "#000000", true); // Dummy draw to clear
            }
            
            ecg_plotter.draw(-90.0, 50.0, "#2ecc71", true); // Neon Green for ECG
            ch3_plotter.draw(0.0, 0.002, "#f1c40f", true); // Yellow for Calcium (0 to 2 uM)
            ch4_plotter.draw(0.0, 1.2, "#e67e22", true); // Orange for Force (0 to 1 normalized)
        }
    });

    rsx! {
        div { id: "app-grid",
            header {
                div { class: "hud-item", "BPM: ", span { id: "hud-bpm", "75" } }
                div { class: "hud-item", "PR: ", span { id: "hud-pr", "160ms" } }
                div { class: "hud-item", "QRS: ", span { id: "hud-qrs", "90ms" } }
                div { class: "hud-item", "QT: ", span { id: "hud-qt", "400ms" } }
                div { class: "hud-item", "V_rest: ", span { id: "hud-vrest", "-85mV" } }
            }

            aside { id: "controls-sidebar",
                Accordion { label: "Células Ativas (Visualização)".to_string(), open: true,
                    Checkbox { label: "Nó Sinoatrial (SA)".to_string(), color: "#f1c40f".to_string(), checked: show_sa }
                    Checkbox { label: "Átrio".to_string(), color: "#ff00ff".to_string(), checked: show_atrium }
                    Checkbox { label: "Nó Atrioventricular (AV)".to_string(), color: "#9b59b6".to_string(), checked: show_av }
                    Checkbox { label: "Ventrículo".to_string(), color: "#00ffff".to_string(), checked: show_vent }
                }
                Accordion { label: "Íons Extracelulares".to_string(), open: false,
                    Slider { label: "K+ (Potássio)".to_string(), min: 2.0, max: 10.0, step: 0.1, val: ko }
                    Slider { label: "Ca2+ (Cálcio)".to_string(), min: 0.5, max: 5.0, step: 0.1, val: cao }
                    Slider { label: "Na+ (Sódio)".to_string(), min: 100.0, max: 160.0, step: 1.0, val: nao }
                }
                Accordion { label: "Fármacos".to_string(), open: false,
                    Slider { label: "Lidocaína (Na+)".to_string(), min: 0.0, max: 100.0, step: 1.0, val: block_na }
                    Slider { label: "Amiodarona (K+)".to_string(), min: 0.0, max: 100.0, step: 1.0, val: block_k }
                    Slider { label: "Verapamil (Ca2+)".to_string(), min: 0.0, max: 100.0, step: 1.0, val: block_ca }
                }
                Accordion { label: "Sistema Autônomo".to_string(), open: false,
                    Slider { label: "Tônus Simpático".to_string(), min: 0.0, max: 100.0, step: 1.0, val: symp }
                    Slider { label: "Tônus Parassimpático".to_string(), min: 0.0, max: 100.0, step: 1.0, val: parasymp }
                    Slider { label: "Isquemia Local".to_string(), min: 0.0, max: 100.0, step: 1.0, val: isch }
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
                    div { class: "canvas-label", "Cálcio Intracelular (Ca²⁺)" }
                    canvas { id: "canvas-ch3" }
                }
                div { class: "canvas-wrapper",
                    div { class: "canvas-label", "Força Ativa (Contração)" }
                    canvas { id: "canvas-ch4" }
                }
            }
        }
    }
}

#[component]
fn Accordion(label: String, open: bool, children: Element) -> Element {
    rsx! {
        details {
            class: "control-group",
            open: "{open}",
            summary {
                style: "cursor: pointer; user-select: none; font-weight: bold; color: var(--primary-color);",
                "{label}"
            }
            div {
                class: "accordion-content",
                style: "margin-top: 10px;",
                {children}
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

#[component]
fn Checkbox(label: String, color: String, checked: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "slider-container",
            style: "display: flex; align-items: center; gap: 10px; cursor: pointer;",
            onclick: move |_| checked.set(!checked()),
            input {
                r#type: "checkbox",
                checked: "{checked}",
                style: "pointer-events: none;" // Let the div handle the click
            }
            div {
                style: "width: 12px; height: 12px; border-radius: 50%; background-color: {color}; box-shadow: 0 0 5px {color};"
            }
            span {
                style: "font-size: 1.8vh; color: var(--text-main);",
                "{label}"
            }
        }
    }
}
