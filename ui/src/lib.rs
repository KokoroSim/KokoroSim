#![allow(non_snake_case)]

use dioxus::prelude::*;
use engine::models::HeartSystem;
use wasm_bindgen::prelude::*;

mod plot;
use plot::Plotter;

mod audio;

const VERSION: &str = env!("KOKOROSIM_VERSION");
const BUILD_TIME: &str = env!("KOKOROSIM_BUILD_TIME");

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
    let mut fibrosis = use_signal(|| 0.0);
    
    let show_sa = use_signal(|| true);
    let show_av = use_signal(|| false);
    let show_atrium = use_signal(|| true);
    let show_purkinje = use_signal(|| false);
    let show_endo = use_signal(|| false);
    let show_epi = use_signal(|| true);
    let show_fibroblast = use_signal(|| false);

    // Audio & Monitoring state (desativados por padrão)
    let mut sound_uti = use_signal(|| false);
    let mut sound_bulhas = use_signal(|| false);

    // Modos de exibição do osciloscópio e Onda Fantasma
    let mut view_mode = use_signal(|| "rolling".to_string());
    let mut show_ghost = use_signal(|| false);
    let mut trigger_capture_ghost = use_signal(|| false);
    let mut trigger_clear_ghost = use_signal(|| false);
    let mut trigger_arm_single = use_signal(|| false);
    let mut is_paused = use_signal(|| false);
    let mut ghost_status_str = use_signal(|| "📸 Capturar".to_string());

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

        // Create plotters com capacidade expandida (500 pontos) para acomodar ~2.5 segundos de histórico
        let buffer_capacity = 500;
        let mut pa_plotter_sa = Plotter::new("canvas-pa", buffer_capacity); // Vermelho para SA
        let mut pa_plotter_av = Plotter::new("canvas-pa", buffer_capacity); // Amarelo para AV
        let mut pa_plotter_atrium = Plotter::new("canvas-pa", buffer_capacity); // Azul para Átrio
        let mut pa_plotter_purk = Plotter::new("canvas-pa", buffer_capacity); // Laranja para Purkinje
        let mut pa_plotter_endo = Plotter::new("canvas-pa", buffer_capacity); // Verde Clínico para Endocárdio
        let mut pa_plotter_epi = Plotter::new("canvas-pa", buffer_capacity); // Verde Menta para Epicárdio
        let mut pa_plotter_fib = Plotter::new("canvas-pa", buffer_capacity); // Lilás para Fibroblasto
        let mut ecg_plotter = Plotter::new("canvas-ecg", buffer_capacity); // Neon Cyan para ECG Dipolar
        let mut ch3_plotter = Plotter::new("canvas-ch3", buffer_capacity); // Cálcio
        let mut hemo_plotter_lvp = Plotter::new("canvas-ch4", buffer_capacity); // LVP Ventricular (Ciano)
        let mut hemo_plotter_aop = Plotter::new("canvas-ch4", buffer_capacity); // AoP Aórtica (Coral)

        let mut audio = audio::AudioManager::new();

        loop {
            gloo_timers::future::TimeoutFuture::new(16).await;

            // 0. Sincronizar Modo de Visualização do Osciloscópio e Comandos
            let current_mode = plot::ViewMode::from_str(&view_mode());
            pa_plotter_sa.set_mode(current_mode);
            pa_plotter_av.set_mode(current_mode);
            pa_plotter_atrium.set_mode(current_mode);
            pa_plotter_purk.set_mode(current_mode);
            pa_plotter_endo.set_mode(current_mode);
            pa_plotter_epi.set_mode(current_mode);
            pa_plotter_fib.set_mode(current_mode);
            ecg_plotter.set_mode(current_mode);
            ch3_plotter.set_mode(current_mode);
            hemo_plotter_lvp.set_mode(current_mode);
            hemo_plotter_aop.set_mode(current_mode);

            if trigger_clear_ghost() {
                pa_plotter_sa.clear_ghost();
                pa_plotter_av.clear_ghost();
                pa_plotter_atrium.clear_ghost();
                pa_plotter_purk.clear_ghost();
                pa_plotter_endo.clear_ghost();
                pa_plotter_epi.clear_ghost();
                pa_plotter_fib.clear_ghost();
                ecg_plotter.clear_ghost();
                ch3_plotter.clear_ghost();
                hemo_plotter_lvp.clear_ghost();
                hemo_plotter_aop.clear_ghost();
                trigger_clear_ghost.set(false);
                ghost_status_str.set("📸 Capturar".to_string());
            }

            if trigger_capture_ghost() {
                pa_plotter_sa.capture_ghost();
                pa_plotter_av.capture_ghost();
                pa_plotter_atrium.capture_ghost();
                pa_plotter_purk.capture_ghost();
                pa_plotter_endo.capture_ghost();
                pa_plotter_epi.capture_ghost();
                pa_plotter_fib.capture_ghost();
                ecg_plotter.capture_ghost();
                ch3_plotter.capture_ghost();
                hemo_plotter_lvp.capture_ghost();
                hemo_plotter_aop.capture_ghost();
                trigger_capture_ghost.set(false);
                ghost_status_str.set("⏳ Aguardando Nó SA...".to_string());
            }

            let g_state = ecg_plotter.ghost_state();
            match g_state {
                plot::GhostCaptureState::Armed => {
                    if ghost_status_str() != "⏳ Aguardando Nó SA..." {
                        ghost_status_str.set("⏳ Aguardando Nó SA...".to_string());
                    }
                }
                plot::GhostCaptureState::Recording => {
                    if ghost_status_str() != "🔴 Gravando Ciclo..." {
                        ghost_status_str.set("🔴 Gravando Ciclo...".to_string());
                    }
                }
                _ => {
                    if ghost_status_str() != "📸 Capturar" {
                        ghost_status_str.set("📸 Capturar".to_string());
                    }
                }
            }

            if trigger_arm_single() {
                pa_plotter_sa.arm_single();
                pa_plotter_av.arm_single();
                pa_plotter_atrium.arm_single();
                pa_plotter_purk.arm_single();
                pa_plotter_endo.arm_single();
                pa_plotter_epi.arm_single();
                pa_plotter_fib.arm_single();
                ecg_plotter.arm_single();
                ch3_plotter.arm_single();
                hemo_plotter_lvp.arm_single();
                hemo_plotter_aop.arm_single();
                trigger_arm_single.set(false);
            }
            
            if !is_paused() {
                // 1. Update Parameters from UI
                let b_na = 1.0 - (block_na() / 100.0);
                let b_k = 1.0 - (block_k() / 100.0);
                let b_ca = 1.0 - (block_ca() / 100.0);
                let b_nak = 1.0 - (block_nak() / 100.0);
                let s_symp = symp() / 100.0;
                let s_parasymp = parasymp() / 100.0;
                let s_isch = isch() / 100.0;
                let s_fibrosis = fibrosis() / 100.0;

                system.write().update_params(
                    ko(), cao(), nao(), 
                    b_na, b_k, b_ca, b_nak, 
                    s_symp, s_parasymp, s_isch, s_fibrosis
                );
                
                // 2. Step Engine
                // Amostragem compacta: 1 ponto a cada 5.0ms (downsample=500 com dt=0.01ms)
                // Em 500 amostras temos 2500ms (2.5s) de traçado visível, comportando > 3 ciclos completos
                // Com Rush-Larsen dt=0.01ms, 1600 passos simulam exatamente 16ms em 1x tempo real
                // com consumo de CPU mínimo (<5-10%), viabilizando execução em celulares e computadores modestos.
                let dt = 0.01; // dt de integração numérica (estável via Rush-Larsen)
                let steps = 1600; // 16ms de simulação biológica por frame a 60 FPS (1x tempo real)
                let downsample = 500; // 5ms por ponto amostrado (500 * 0.01ms = 5.0ms)
                let batch = system.write().run_batch(dt, steps, downsample);
                
                // Batch achatado de 12 canais: [sa, av, atr, purk, endo, epi, fib, cai, lvp, aop, ecg, sound_events]
                let chunk_size = 12;
                for chunk in batch.chunks(chunk_size) {
                    if chunk.len() == 12 {
                        let sa = chunk[0];
                        let av = chunk[1];
                        let atrium = chunk[2];
                        let purk = chunk[3];
                        let endo = chunk[4];
                        let epi = chunk[5];
                        let fib = chunk[6];
                        let cai = chunk[7];
                        let lvp = chunk[8];
                        let aop = chunk[9];
                        let ecg = chunk[10];
                        let sound_code = chunk[11] as u32;
                        let is_sa_fire = (sound_code & 8) != 0;
                        
                        pa_plotter_sa.push(sa, sound_code, is_sa_fire);
                        pa_plotter_av.push(av, sound_code, is_sa_fire);
                        pa_plotter_atrium.push(atrium, sound_code, is_sa_fire);
                        pa_plotter_purk.push(purk, sound_code, is_sa_fire);
                        pa_plotter_endo.push(endo, sound_code, is_sa_fire);
                        pa_plotter_epi.push(epi, sound_code, is_sa_fire);
                        pa_plotter_fib.push(fib, sound_code, is_sa_fire);

                        ecg_plotter.push(ecg, sound_code, is_sa_fire);
                        ch3_plotter.push(cai, sound_code, is_sa_fire);
                        hemo_plotter_lvp.push(lvp, sound_code, is_sa_fire);
                        hemo_plotter_aop.push(aop, sound_code, is_sa_fire);

                        // Disparo dos eventos acústicos de acordo com as checkboxes ativas
                        if (sound_code & 1) != 0 && sound_uti() {
                            audio.play_uti_beep();
                        }
                        if (sound_code & 2) != 0 && sound_bulhas() {
                            audio.play_b1();
                        }
                        if (sound_code & 4) != 0 && sound_bulhas() {
                            audio.play_b2();
                        }
                    }
                }
            }

            // 3. Draw to Canvas
            // Flags de visibilidade das camadas celulares
            let any_epi = show_epi();
            let any_endo = show_endo();
            let any_purk = show_purkinje();
            let any_atrium = show_atrium();
            let any_av = show_av();
            let any_sa = show_sa();
            let any_fib = show_fibroblast();

            let ghost = show_ghost();
            let uti_m = sound_uti();
            let bulhas_m = sound_bulhas();

            // O primeiro traçado a ser desenhado limpa o canvas e plota as linhas verticais de áudio
            let mut cleared = false;

            if any_sa {
                pa_plotter_sa.draw(-90.0, 50.0, "#e74c3c", !cleared, ghost, uti_m, bulhas_m); // Nó SA: Vermelho (#e74c3c)
                cleared = true;
            }
            if any_atrium {
                pa_plotter_atrium.draw(-90.0, 50.0, "#3498db", !cleared, ghost, uti_m, bulhas_m); // Átrio: Azul (#3498db)
                cleared = true;
            }
            if any_av {
                pa_plotter_av.draw(-90.0, 50.0, "#f1c40f", !cleared, ghost, uti_m, bulhas_m); // Nó AV: Amarelo (#f1c40f)
                cleared = true;
            }
            if any_purk {
                pa_plotter_purk.draw(-90.0, 50.0, "#e67e22", !cleared, ghost, uti_m, bulhas_m); // Purkinje: Laranja (#e67e22)
                cleared = true;
            }
            if any_endo {
                pa_plotter_endo.draw(-90.0, 50.0, "#2ecc71", !cleared, ghost, uti_m, bulhas_m); // Endocárdio: Verde Clínico (#2ecc71)
                cleared = true;
            }
            if any_epi {
                pa_plotter_epi.draw(-90.0, 50.0, "#1abc9c", !cleared, ghost, uti_m, bulhas_m); // Epicárdio: Verde Menta (#1abc9c)
                cleared = true;
            }
            if any_fib {
                pa_plotter_fib.draw(-90.0, 50.0, "#a29bfe", !cleared, ghost, uti_m, bulhas_m); // Fibroblasto: Lilás (#a29bfe)
                cleared = true;
            }

            // Se nenhuma camada estiver ativa, limpa a tela e plota os marcadores
            if !cleared {
                pa_plotter_endo.draw(-90.0, 50.0, "#000000", true, false, uti_m, bulhas_m);
            }
            
            ecg_plotter.draw(-35.0, 120.0, "#00f2fe", true, ghost, uti_m, bulhas_m); // Neon Cyan (#00f2fe) para DII (ECG Transmural)
            ch3_plotter.draw(0.0, 0.002, "#a29bfe", true, ghost, uti_m, bulhas_m); // Roxo-Lilás (#a29bfe) para Cálcio
            
            // Hemodinâmica: LVP e AoP sobrepostas (0 a 140 mmHg) com marcadores verticais
            hemo_plotter_lvp.draw(0.0, 140.0, "#00f2fe", true, ghost, uti_m, bulhas_m); // Ciano (#00f2fe) para LVP Ventricular
            hemo_plotter_aop.draw(0.0, 140.0, "#ff1754", false, ghost, uti_m, bulhas_m); // Carmesim (#ff1754) para Pressão Aórtica (AoP)

            // 4. Update HUD metrics at ~10 Hz (every 6 frames)
            if !is_paused() {
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
        }
    });

    let bpm_str = if bpm() > 0.0 { format!("{:.0}", bpm()) } else { "0".to_string() };
    let pr_str = if pr() > 0.0 { format!("{:.0}ms", pr()) } else { "---".to_string() };
    let qrs_str = format!("{:.0}ms", qrs());
    let qt_str = if qt() > 0.0 { format!("{:.0}ms", qt()) } else { "---".to_string() };
    let vrest_str = format!("{:.0}mV", v_rest());
    let pr_rr_str = if pr_rr() > 0.0 { format!("{:.2}", pr_rr()) } else { "---".to_string() };

    let mut active_accordion = use_signal(|| None::<usize>);

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
        fibrosis.set(0.0);
        sound_uti.set(false);
        sound_bulhas.set(false);
        view_mode.set("rolling".to_string());
        show_ghost.set(false);
        trigger_capture_ghost.set(false);
        trigger_clear_ghost.set(true);
        trigger_arm_single.set(false);
        is_paused.set(false);
        ghost_status_str.set("📸 Capturar".to_string());
        system.set(HeartSystem::new());
        active_accordion.set(None);
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
                            h2 {
                                "Sobre o kokor",
                                span { class: "hud-kanji", "心" },
                                span { class: "hud-sim", "sim" }
                            }
                            button {
                                class: "btn-close-modal",
                                onclick: move |_| show_about.set(false),
                                "✕"
                            }
                        }
                        div { class: "modal-body",
                            p { "O KokoroSim é um simulador eletrofisiológico cardíaco celular em tempo real executado nativamente via WebAssembly (WASM) com Rust e Dioxus." }
                            br {}
                            h4 { style: "color: var(--neon-yellow); margin-bottom: 6px;", "Modelos Biofísicos Integrados:" }
                            ul { style: "margin-left: 20px; margin-bottom: 12px;",
                                li { b { "Nó Sinoatrial (SA): " }, "Severi et al. (2012) — Automatismo biológico e modulação autonômica cronotrópica." }
                                li { b { "Músculo Atrial: " }, "Courtemanche et al. (1998) — Células atriais humanas de resposta rápida." }
                                li { b { "Nó Atrioventricular (AV): " }, "Inada et al. (2009) — Dromotropismo dinâmico dependente de taxa, tônus autonômico e canais de cálcio." }
                                li { b { "Fibras de Purkinje (His): " }, "Stewart et al. (2009) — Rede de condução rápida e marcapasso terciário de escape." }
                                li { b { "Heterogeneidade Transmural: " }, "ten Tusscher & Panfilov (2006) — Subtipos Endocárdio, Célula M e Epicárdio gerando ECG dipolar P-QRS-T real." }
                                li { b { "Fibroblastos Cardíacos: " }, "MacCannell et al. (2007) — Acoplamento eletrotônico via gap junctions, dreno capacitivo e fibrose miocárdica." }
                                li { b { "Hemodinâmica e Mecânica: " }, "Elastância variável no tempo (Suga-Sagawa) e modelo arterial Windkessel acoplados ao cálcio intracelular, com bulhas (B1/B2) e monitor UTI." }
                            }
                            h4 { style: "color: var(--neon-cyan); margin-bottom: 6px;", "Modulação Farmacológica e Autonômica:" }
                            p { "Permite intervenção direta nos eletrólitos extracelulares (K+, Ca2+, Na+), tônus autonômico (simpático e parassimpático), condições isquêmicas, fibrose miocárdica e quatro classes de fármacos antiarrítmicos (Lidocaína, Amiodarona, Verapamil e Digoxina)." }
                        }
                    }
                }
            }

            header {
                div { class: "hud-brand",
                    img { src: "assets/icon.png", class: "hud-logo", alt: "KokoroSim" }
                    span { class: "hud-brand-text",
                        "kokor",
                        span { class: "hud-kanji", "心" },
                        span { class: "hud-sim", "sim" }
                    }
                }
                div { class: "hud-metrics",
                    div { class: "hud-item", "心拍数 // FC: ", span { id: "hud-bpm", "{bpm_str} BPM" } }
                    div { class: "hud-item", "PR間隔 // PR: ", span { id: "hud-pr", "{pr_str}" } }
                    div { class: "hud-item", "QRS幅 // QRS: ", span { id: "hud-qrs", "{qrs_str}" } }
                    div { class: "hud-item", "QT間隔 // QT: ", span { id: "hud-qt", "{qt_str}" } }
                    div { class: "hud-item", "静止電位 // V.Rep: ", span { id: "hud-vrest", "{vrest_str}" } }
                    div { class: "hud-item", "PR/RR: ", span { id: "hud-pr-rr", "{pr_rr_str}" } }
                }
            }

            aside { id: "controls-sidebar",
                div { class: "top-buttons",
                    button {
                        class: "btn-reset-all",
                        onclick: move |_| reset_all(),
                        "☢ RESETAR"
                    }
                    button {
                        class: if is_paused() { "btn-resume" } else { "btn-pause" },
                        onclick: move |_| is_paused.set(!is_paused()),
                        if is_paused() { "▶ CONTINUAR" } else { "⏸ CONGELAR" }
                    }
                    button {
                        class: "btn-about",
                        title: "Abrir documentação científica e manual completo em nova aba",
                        onclick: move |_| {
                            if let Some(w) = web_sys::window() {
                                let _ = w.open_with_url_and_target("./index.html", "_blank");
                            }
                        },
                        "ℹ SOBRE"
                    }
                }

                Accordion { index: 0, label: "0. 画面表示 // Visualização".to_string(), active_accordion,
                    div { style: "margin-bottom: 6px;",
                        div { class: "slider-header", style: "margin-bottom: 3px;",
                            span { style: "color: var(--neon-cyan); font-weight: bold; font-size: 1.4vh;", "Modo do Osciloscópio:" }
                        }
                        select {
                            style: "width: 100%; padding: 4px 6px; border-radius: 4px; background: #161616; color: var(--text-main); border: 1px solid var(--neon-cyan); font-size: 1.35vh; cursor: pointer;",
                            value: "{view_mode}",
                            onchange: move |e| view_mode.set(e.value()),
                            option { value: "rolling", "Fita Deslizante (Fluxo Contínuo) [Padrão]" }
                            option { value: "sweep", "Varredura Contínua (Monitor UTI)" }
                            option { value: "paged", "Paginação Sincronizada (Página por Ciclo)" }
                            option { value: "triggered_auto", "Gatilho Automático (Auto a cada Batimento)" }
                            option { value: "triggered_single", "Gatilho Único (Single-Shot / Congelado)" }
                        }
                    }

                    if view_mode() == "triggered_single" {
                        button {
                            class: "icon-btn",
                            style: "width: 100%; padding: 6px; margin-bottom: 6px; background: var(--neon-yellow); color: #000; font-weight: bold; border: none; border-radius: 4px; cursor: pointer; font-size: 1.35vh; box-shadow: 0 0 8px rgba(241, 196, 15, 0.4);",
                            onclick: move |_| trigger_arm_single.set(true),
                            "⚡ DISPARAR / ARMAR PRÓXIMO CICLO"
                        }
                    }

                    div { style: "display: flex; gap: 6px; align-items: center; margin-bottom: 6px; background: #111; padding: 4px 6px; border-radius: 4px; border: 1px solid #333;",
                        div { style: "flex: 1;",
                            Checkbox { label: "Onda Fantasma (Histórico)".to_string(), color: "#ffffff".to_string(), checked: show_ghost, compact: true }
                        }
                        button {
                            class: "icon-btn",
                            style: "padding: 4px 8px; background: #222; color: var(--neon-cyan); border: 1px solid var(--neon-cyan); border-radius: 4px; cursor: pointer; font-size: 1.25vh; white-space: nowrap;",
                            title: "Captura snapshot de 1 ciclo cardíaco completo ancorado na Fase 0 do Nó SA",
                            onclick: move |_| {
                                show_ghost.set(true);
                                trigger_capture_ghost.set(true);
                            },
                            "{ghost_status_str}"
                        }
                    }

                    div { class: "slider-header", style: "margin-bottom: 4px; margin-top: 2px;",
                        span { style: "color: var(--neon-yellow); font-size: 1.4vh;", "Camadas / Células Ativas:" }
                    }

                    Checkbox { label: "Nó SA (Gatilho)".to_string(), color: "#e74c3c".to_string(), checked: show_sa, compact: true }
                    Checkbox { label: "Átrio (Contração)".to_string(), color: "#3498db".to_string(), checked: show_atrium, compact: true }
                    Checkbox { label: "Nó AV (Condução)".to_string(), color: "#f1c40f".to_string(), checked: show_av, compact: true }
                    Checkbox { label: "Purkinje (Feixe de His)".to_string(), color: "#e67e22".to_string(), checked: show_purkinje, compact: true }
                    Checkbox { label: "Endocárdio (Subendocárdico)".to_string(), color: "#2ecc71".to_string(), checked: show_endo, compact: true }
                    Checkbox { label: "Epicárdio (Subepicárdico)".to_string(), color: "#1abc9c".to_string(), checked: show_epi, compact: true }
                    Checkbox { label: "Fibroblasto (Eletrotônico)".to_string(), color: "#a29bfe".to_string(), checked: show_fibroblast, compact: true }
                }

                Accordion { index: 1, label: "1. 電解質 // Íons e Eletrólitos".to_string(), active_accordion,
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

                Accordion { index: 2, label: "2. 自律神経 // Sistema Nervoso Autônomo".to_string(), active_accordion,
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

                Accordion { index: 3, label: "3. 抗不整脈薬 // Fármacos Antiarrítmicos".to_string(), active_accordion,
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

                Accordion { index: 4, label: "4. 病態生理 // Condições Patológicas".to_string(), active_accordion,
                    Slider {
                        label: "Nível de Isquemia".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Falta de ATP induz a abertura dos canais I_K,ATP. Aborta o platô precocemente e causa Supra de ST.".to_string()),
                        val: isch
                    }
                    Slider {
                        label: "Fibrose Miocárdica".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Acoplamento a fibroblastos não-excitáveis (MacCannell 2007). Drena corrente da fase 0, despolariza repouso e causa bloqueios intramiocárdicos.".to_string()),
                        val: fibrosis
                    }
                }

                Accordion { index: 5, label: "5. 生体音響 // Monitorização & Áudio".to_string(), active_accordion,
                    Checkbox { label: "🔊 Bip de Monitor (UTI - Onda R)".to_string(), color: "#2ecc71".to_string(), checked: sound_uti }
                    Checkbox { label: "🩺 Bulhas Cardíacas (B1 / B2)".to_string(), color: "#e74c3c".to_string(), checked: sound_bulhas }
                }
            }

            main {
                div { class: "canvas-wrapper",
                    div { class: "canvas-label", "CH-01 [ 活動電位 // POTENCIAIS DE AÇÃO CELULAR ]" }
                    canvas { id: "canvas-pa" }
                }
                div { class: "canvas-wrapper",
                    div { class: "canvas-label", "CH-02 [ 心電図 // ECG DERIVAÇÃO II ]" }
                    canvas { id: "canvas-ecg" }
                }
                div { class: "canvas-wrapper",
                    div { class: "canvas-label", "CH-03 [ カルシウム動態 // TRANSIENTE DE CÁLCIO (Ca²⁺) ]" }
                    canvas { id: "canvas-ch3" }
                }
                div { class: "canvas-wrapper",
                    div { class: "canvas-label",
                        span { "CH-04 [ 左室圧迫曲線 // HEMODINÂMICA: " }
                        span { 
                            style: "display: inline-flex; align-items: center; gap: 4px;",
                            span { style: "display: inline-block; width: 8px; height: 8px; border-radius: 50%; background-color: #00f2fe; box-shadow: 0 0 6px #00f2fe;" }
                            span { style: "color: #00f2fe;", "LVP" }
                        }
                        span { "&" }
                        span { 
                            style: "display: inline-flex; align-items: center; gap: 4px;",
                            span { style: "display: inline-block; width: 8px; height: 8px; border-radius: 50%; background-color: #ff1754; box-shadow: 0 0 6px #ff1754;" }
                            span { style: "color: #ff1754;", "AoP" }
                        }
                        span { "]" }
                    }
                    canvas { id: "canvas-ch4" }
                }
            }

            footer {
                div { class: "footer-item",
                    "kokor",
                    span { class: "hud-kanji", "心" },
                    span { class: "hud-sim", "sim" },
                    " [REALTIME WASM RUSH-LARSEN]"
                }
                div { class: "footer-item",
                    "SYS-VER: ", span { "{VERSION}" }, " // BUILD: ", span { "{BUILD_TIME}" }
                }
            }
        }
    }
}

#[component]
fn Accordion(index: usize, label: String, mut active_accordion: Signal<Option<usize>>, children: Element) -> Element {
    let is_open = active_accordion() == Some(index);
    let summary_color = if is_open { "var(--neon-cyan)" } else { "var(--neon-yellow)" };
    let summary_style = format!("cursor: pointer; user-select: none; font-weight: bold; color: {}; font-size: 1.65vh; padding: 0.8vh 0.8vw;", summary_color);
    rsx! {
        details {
            class: "control-group",
            "name": "sidebar-accordions",
            open: is_open,
            summary {
                style: "{summary_style}",
                prevent_default: "onclick",
                onclick: move |_| {
                    if active_accordion() == Some(index) {
                        active_accordion.set(None);
                    } else {
                        active_accordion.set(Some(index));
                    }
                },
                "{label}"
            }
            div {
                class: "accordion-content",
                style: "display: flex; flex-direction: column; margin-top: 6px;",
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
fn Checkbox(
    label: String,
    color: String,
    checked: Signal<bool>,
    #[props(default = false)] compact: bool,
) -> Element {
    let container_style = if compact {
        "display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 0.35vh 0.4vw; margin-bottom: 0.3vh; border-bottom: none;"
    } else {
        "display: flex; align-items: center; gap: 10px; cursor: pointer; margin-bottom: 1.2vh;"
    };
    let font_size = if compact { "1.35vh" } else { "1.8vh" };
    let input_size = if compact { "1.4vh" } else { "1.8vh" };
    let dot_size = if compact { "8px" } else { "12px" };

    rsx! {
        div {
            class: if compact { "slider-container compact-checkbox" } else { "slider-container" },
            style: "{container_style}",
            onclick: move |_| checked.set(!checked()),
            input {
                r#type: "checkbox",
                checked: "{checked}",
                style: "pointer-events: none; width: {input_size}; height: {input_size}; cursor: pointer;"
            }
            div {
                style: "width: {dot_size}; height: {dot_size}; border-radius: 50%; background-color: {color}; box-shadow: 0 0 5px {color}; flex-shrink: 0;"
            }
            span {
                style: "font-size: {font_size}; color: var(--text-main);",
                "{label}"
            }
        }
    }
}
