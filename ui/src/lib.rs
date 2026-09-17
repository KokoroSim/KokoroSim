#![allow(non_snake_case)]

use dioxus::prelude::*;
use engine::models::HeartSystem;
use wasm_bindgen::prelude::*;

mod plot;
use plot::Plotter;

const VERSION: &str = env!("SIMCARDIO_VERSION");
const BUILD_TIME: &str = env!("SIMCARDIO_BUILD_TIME");

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
    
    let show_sa = use_signal(|| true);
    let show_av = use_signal(|| true);
    let show_atrium = use_signal(|| true);
    let show_purkinje = use_signal(|| true);
    let show_vent = use_signal(|| true);

    // Dynamic HUD metrics state
    let mut bpm = use_signal(|| 75.0);
    let mut pr = use_signal(|| 160.0);
    let mut qrs = use_signal(|| 90.0);
    let mut qt = use_signal(|| 400.0);
    let mut v_rest = use_signal(|| -85.0);
    let mut pr_rr = use_signal(|| 0.20);
    let mut show_about = use_signal(|| false);
    
    use_future(move || async move {
        let mut frame_count: u32 = 0;

        // Create plotters (Canvas IDs match the HTML)
        let mut pa_plotter_sa = Plotter::new("canvas-pa", 300); // Vermelho para SA
        let mut pa_plotter_av = Plotter::new("canvas-pa", 300); // Amarelo para AV
        let mut pa_plotter_atrium = Plotter::new("canvas-pa", 300); // Azul para Átrio
        let mut pa_plotter_purk = Plotter::new("canvas-pa", 300); // Laranja para Purkinje
        let mut pa_plotter_vent = Plotter::new("canvas-pa", 300); // Verde para Ventrículo
        let mut ecg_plotter = Plotter::new("canvas-ecg", 300); // Neon Green/Cyan para ECG
        let mut ch3_plotter = Plotter::new("canvas-ch3", 300); // Cálcio
        let mut ch4_plotter = Plotter::new("canvas-ch4", 300); // Força

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
            
            // Batch is flattened: [sa_v, av_v, atrium_v, purk_v, vent_v, cai, force]
            let chunk_size = 7;
            for chunk in batch.chunks(chunk_size) {
                if chunk.len() == 7 {
                    let sa = chunk[0];
                    let av = chunk[1];
                    let atrium = chunk[2];
                    let purk = chunk[3];
                    let vent = chunk[4];
                    let cai = chunk[5];
                    let force = chunk[6];
                    
                    pa_plotter_sa.push(sa);
                    pa_plotter_av.push(av);
                    pa_plotter_atrium.push(atrium);
                    pa_plotter_purk.push(purk);
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
            let any_purk = show_purkinje();
            let any_atrium = show_atrium();
            let any_av = show_av();
            let any_sa = show_sa();

            // First drawn line clears the canvas, subsequent ones draw on top
            let mut cleared = false;

            if any_sa {
                pa_plotter_sa.draw(-90.0, 50.0, "#e74c3c", !cleared); // Nó SA: Vermelho (#e74c3c)
                cleared = true;
            }
            if any_atrium {
                pa_plotter_atrium.draw(-90.0, 50.0, "#3498db", !cleared); // Átrio: Azul (#3498db)
                cleared = true;
            }
            if any_av {
                pa_plotter_av.draw(-90.0, 50.0, "#f1c40f", !cleared); // Nó AV: Amarelo (#f1c40f)
                cleared = true;
            }
            if any_purk {
                pa_plotter_purk.draw(-90.0, 50.0, "#e67e22", !cleared); // Purkinje / His: Laranja (#e67e22)
                cleared = true;
            }
            if any_vent {
                pa_plotter_vent.draw(-90.0, 50.0, "#2ecc71", !cleared); // Ventrículo: Verde Clínico (#2ecc71)
                cleared = true;
            }

            // If nothing was drawn, clear it manually
            if !cleared {
                pa_plotter_vent.draw(-90.0, 50.0, "#000000", true); // Dummy draw to clear
            }
            
            ecg_plotter.draw(-90.0, 50.0, "#00ffff", true); // Ciano (#00ffff) para DII (ECG)
            ch3_plotter.draw(0.0, 0.002, "#9b59b6", true); // Roxo (#9b59b6) para Cálcio
            ch4_plotter.draw(0.0, 1.2, "#e67e22", true); // Laranja (#e67e22) para Força

            // 4. Update HUD metrics at ~10 Hz (every 6 frames)
            frame_count = frame_count.wrapping_add(1);
            if frame_count % 6 == 0 {
                let metrics = system.read().get_hud_metrics();
                bpm.set(metrics.bpm);
                pr.set(metrics.pr);
                qrs.set(metrics.qrs);
                qt.set(metrics.qt);
                v_rest.set(metrics.v_rest);
                pr_rr.set(metrics.pr_rr);
            }
        }
    });

    let bpm_str = if bpm() > 0.0 { format!("{:.0}", bpm()) } else { "0".to_string() };
    let pr_str = if pr() > 0.0 { format!("{:.0}ms", pr()) } else { "---".to_string() };
    let qrs_str = format!("{:.0}ms", qrs());
    let qt_str = if qt() > 0.0 { format!("{:.0}ms", qt()) } else { "---".to_string() };
    let vrest_str = format!("{:.0}mV", v_rest());
    let pr_rr_str = if pr_rr() > 0.0 { format!("{:.2}", pr_rr()) } else { "---".to_string() };

    let mut reset_all = move || {
        ko.set(5.4);
        cao.set(2.0);
        nao.set(140.0);
        block_na.set(0.0);
        block_k.set(0.0);
        block_ca.set(0.0);
        block_nak.set(0.0);
        symp.set(0.0);
        parasymp.set(0.0);
        isch.set(0.0);
        system.set(HeartSystem::new());
    };

    rsx! {
        div { id: "app-grid",
            if show_about() {
                div {
                    class: "modal-backdrop",
                    onclick: move |_| show_about.set(false),
                    div {
                        class: "modal-content",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "modal-header",
                            h2 { "Sobre o SimCardio" }
                            button {
                                class: "btn-close-modal",
                                onclick: move |_| show_about.set(false),
                                "✕"
                            }
                        }
                        div { class: "modal-body",
                            p { "O SimCardio é um simulador eletrofisiológico cardíaco celular em tempo real executado nativamente via WebAssembly (WASM) com Rust e Dioxus." }
                            br {}
                            h4 { style: "color: var(--neon-yellow); margin-bottom: 6px;", "Modelos Biofísicos Integrados:" }
                            ul { style: "margin-left: 20px; margin-bottom: 12px;",
                                li { b { "Nó Sinoatrial (SA): " }, "Severi et al. (2012) — Automatismo biológico e modulação autonômica cronotrópica." }
                                li { b { "Músculo Atrial: " }, "Courtemanche et al. (1998) — Células atriais humanas de resposta rápida." }
                                li { b { "Nó Atrioventricular (AV): " }, "Inada et al. (2009) — Retardo fisiológico PR e via de condução juncional." }
                                li { b { "Fibras de Purkinje (His): " }, "Stewart et al. (2009) — Rede de condução rápida e marcapasso terciário de escape." }
                                li { b { "Músculo Ventricular: " }, "ten Tusscher & Panfilov (2006) — Platô ventricular humano e transiente de cálcio." }
                            }
                            h4 { style: "color: var(--neon-cyan); margin-bottom: 6px;", "Modulação Farmacológica e Autonômica:" }
                            p { "Permite intervenção direta nos eletrólitos extracelulares (K+, Ca2+, Na+), tônus autonômico (simpático e parassimpático), condições isquêmicas e quatro classes de fármacos antiarrítmicos (Lidocaína, Amiodarona, Verapamil e Digoxina)." }
                        }
                    }
                }
            }

            header {
                div { class: "hud-item", "BPM: ", span { id: "hud-bpm", "{bpm_str}" } }
                div { class: "hud-item", "PR: ", span { id: "hud-pr", "{pr_str}" } }
                div { class: "hud-item", "QRS: ", span { id: "hud-qrs", "{qrs_str}" } }
                div { class: "hud-item", "QT: ", span { id: "hud-qt", "{qt_str}" } }
                div { class: "hud-item", "V_rest: ", span { id: "hud-vrest", "{vrest_str}" } }
                div { class: "hud-item", "PR/RR: ", span { id: "hud-pr-rr", "{pr_rr_str}" } }
            }

            aside { id: "controls-sidebar",
                div { class: "top-buttons",
                    button {
                        class: "btn-reset-all",
                        onclick: move |_| reset_all(),
                        "☢ RESETAR SIMULAÇÃO ☢"
                    }
                    button {
                        class: "btn-about",
                        onclick: move |_| show_about.set(true),
                        "ℹ SOBRE"
                    }
                }

                Accordion { label: "0. Visualização das Células".to_string(), open: true,
                    Checkbox { label: "Nó SA (Gatilho)".to_string(), color: "#e74c3c".to_string(), checked: show_sa }
                    Checkbox { label: "Átrio (Contração)".to_string(), color: "#3498db".to_string(), checked: show_atrium }
                    Checkbox { label: "Nó AV (Condução)".to_string(), color: "#f1c40f".to_string(), checked: show_av }
                    Checkbox { label: "Purkinje (Feixe de His)".to_string(), color: "#e67e22".to_string(), checked: show_purkinje }
                    Checkbox { label: "Ventrículo (Motor)".to_string(), color: "#2ecc71".to_string(), checked: show_vent }
                }

                Accordion { label: "1. Íons e Eletrólitos".to_string(), open: false,
                    Slider {
                        label: "Potássio [K+]_o".to_string(),
                        min: 2.0, max: 8.5, step: 0.1, default_val: 5.4, unit: " mEq/L".to_string(),
                        help: Some("Mínimo (2.0) causa hiperexcitabilidade, máximo (8.5) causa parada em diástole (todas as células param).".to_string()),
                        val: ko
                    }
                    Slider {
                        label: "Cálcio [Ca2+]_o".to_string(),
                        min: 1.0, max: 3.5, step: 0.1, default_val: 2.0, unit: " mmol/L".to_string(),
                        help: Some("Hipocalcemia (1.0) prolonga QT, hipercalcemia (3.5) encurta QT.".to_string()),
                        val: cao
                    }
                    Slider {
                        label: "Sódio [Na+]_o".to_string(),
                        min: 125.0, max: 155.0, step: 1.0, default_val: 140.0, unit: " mEq/L".to_string(),
                        help: Some("Reduzir o Sódio diminui a amplitude do Potencial de Ação. Mínimo ajustado (125) para evitar Bloqueio Sinoatrial isolado.".to_string()),
                        val: nao
                    }
                }

                Accordion { label: "2. Sistema Nervoso Autônomo".to_string(), open: false,
                    Slider {
                        label: "Tônus Simpático".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Aumenta a permeabilidade dos canais I_f (funny), I_CaL (cálcio lento) e a velocidade da bomba SERCA.".to_string()),
                        val: symp
                    }
                    Slider {
                        label: "Tônus Parassimpático".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Ativa imediatamente a corrente I_K,ACh (hiperpolarizando as células nodais) e reduz o I_CaL.".to_string()),
                        val: parasymp
                    }
                }

                Accordion { label: "3. Fármacos Antiarrítmicos".to_string(), open: false,
                    Slider {
                        label: "Bloq. Na+ (Lidocaína)".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Classe I: Alargamento progressivo do complexo QRS; redução violenta do dV/dt_max da Fase 0.".to_string()),
                        val: block_na
                    }
                    Slider {
                        label: "Bloq. K+ (Amiodarona)".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Classe III: Atraso na repolarização (Fase 3); prolongamento agudo do intervalo QT.".to_string()),
                        val: block_k
                    }
                    Slider {
                        label: "Bloq. Ca2+ (Verapamil)".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Classe IV: Achatamento do platô ventricular e Bloqueio Atrioventricular.".to_string()),
                        val: block_ca
                    }
                    Slider {
                        label: "Inibidor Na+/K+ (Digoxina)".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Reduz a força da corrente I_NaK. Causa acúmulo intracelular de sódio e cálcio.".to_string()),
                        val: block_nak
                    }
                }

                Accordion { label: "4. Condições Patológicas".to_string(), open: false,
                    Slider {
                        label: "Nível de Isquemia".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Falta de ATP induz a abertura dos canais I_K,ATP. Aborta o platô precocemente e causa Supra de ST.".to_string()),
                        val: isch
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
                    div { class: "canvas-label", "Cálcio Intracelular (Ca²⁺)" }
                    canvas { id: "canvas-ch3" }
                }
                div { class: "canvas-wrapper",
                    div { class: "canvas-label", "Força Ativa (Contração)" }
                    canvas { id: "canvas-ch4" }
                }
            }

            footer {
                div { class: "footer-item",
                    "Versão: ",
                    span { "{VERSION}" }
                }
                div { class: "footer-item",
                    "Build: ",
                    span { "{BUILD_TIME}" }
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
                style: "cursor: pointer; user-select: none; font-weight: bold; color: var(--neon-yellow); font-size: 2vh; padding: 0.8vh 0;",
                "{label}"
            }
            div {
                class: "accordion-content",
                style: "display: flex; flex-direction: column; margin-top: 10px;",
                {children}
            }
        }
    }
}

#[component]
fn Slider(
    label: String,
    min: f64,
    max: f64,
    step: f64,
    default_val: f64,
    unit: String,
    help: Option<String>,
    mut val: Signal<f64>,
) -> Element {
    let mut show_help = use_signal(|| false);

    let val_num = val();
    let percentage = if max > min {
        ((val_num - min) / (max - min)).clamp(0.0, 1.0) * 100.0
    } else {
        0.0
    };

    let max_diff = (max - default_val).abs().max((min - default_val).abs());
    let diff = (val_num - default_val).abs();
    let severity = if max_diff > 0.0 { (diff / max_diff).clamp(0.0, 1.0) } else { 0.0 };
    let hue = 120.0 - (severity * 120.0); // 120 (Green) to 0 (Red)
    let style_str = format!("--val: {:.1}%; --track-color: hsl({:.0}, 100%, 50%);", percentage, hue);

    let formatted_val = if step < 1.0 {
        format!("{:.1}{}", val_num, unit)
    } else {
        format!("{:.0}{}", val_num, unit)
    };

    rsx! {
        div { class: "slider-container",
            div { class: "slider-header",
                span { "{label}" }
                span { style: "color: hsl({hue:.0}, 100%, 60%); font-weight: bold;", "{formatted_val}" }
            }
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{val_num}",
                style: "{style_str}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        val.set(v);
                    }
                }
            }
            div { class: "slider-actions",
                button {
                    class: "icon-btn btn-step",
                    title: "Diminuir",
                    onclick: move |_| {
                        let mult = if step < 1.0 { 10.0 } else { 1.0 };
                        let next_v = ((val() - step) * mult).round() / mult;
                        val.set(next_v.max(min));
                    },
                    "-"
                }
                button {
                    class: "icon-btn btn-step",
                    title: "Aumentar",
                    onclick: move |_| {
                        let mult = if step < 1.0 { 10.0 } else { 1.0 };
                        let next_v = ((val() + step) * mult).round() / mult;
                        val.set(next_v.min(max));
                    },
                    "+"
                }
                button {
                    class: "icon-btn btn-reset",
                    title: "Restaurar padrão",
                    onclick: move |_| val.set(default_val),
                    "↺ Reset"
                }
                if let Some(ref _help_text) = help {
                    button {
                        class: "icon-btn btn-help",
                        title: "Ajuda",
                        onclick: move |_| show_help.set(!show_help()),
                        "? Ajuda"
                    }
                }
            }
            if show_help() {
                if let Some(ref help_text) = help {
                    div { class: "help-text show", "{help_text}" }
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
            style: "display: flex; align-items: center; gap: 10px; cursor: pointer; margin-bottom: 1.2vh;",
            onclick: move |_| checked.set(!checked()),
            input {
                r#type: "checkbox",
                checked: "{checked}",
                style: "pointer-events: none; width: 1.8vh; height: 1.8vh;"
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
