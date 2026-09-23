#![allow(non_snake_case)]

use dioxus::prelude::*;
use engine::models::HeartSystem;
use wasm_bindgen::prelude::*;

mod plot;
use plot::{Plotter, PvLoopPlotter, GuytonPlotter};

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
    
    // Valvopatias (0% a 100%)
    let mut aortic_stenosis = use_signal(|| 0.0);
    let mut aortic_regurg = use_signal(|| 0.0);
    let mut mitral_stenosis = use_signal(|| 0.0);
    let mut mitral_regurg = use_signal(|| 0.0);

    // Dinâmica Microvascular & Starling (0% a 100%, 1.0 a 6.0 g/dL)
    let mut albumin_slider = use_signal(|| 4.0); // g/dL
    let mut permeability_slider = use_signal(|| 0.0); // 0% a 100%
    
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
    let mut ecg_lead_idx = use_signal(|| 1usize);
    let mut ion_cell_idx = use_signal(|| 4usize); // Endocárdio por padrão
    let mut ion_var_idx = use_signal(|| 0usize);  // [Ca2+]_i por padrão
    let mut two_d_mode = use_signal(|| "pv".to_string()); // "pv" ou "guyton"

    // Seletor de Especialidade / Abas de Laboratório
    let mut active_tab = use_signal(|| "cardio".to_string()); // "cardio" ou "pulmo"

    // Parâmetros e Métricas Respiratórias
    let mut resp_rate = use_signal(|| 15.0); // irpm
    let mut resp_raw = use_signal(|| 1.5);   // cmH2O/(L/s)
    let mut resp_crs = use_signal(|| 0.10);  // L/cmH2O
    let mut rsa_toggle = use_signal(|| true); // Arritmia Sinusal Respiratória
    let mut baro_toggle = use_signal(|| true); // Barorreflexo em Malha Fechada
    let mut orthostasis_toggle = use_signal(|| false); // Desafio Postural de Ortostase (Guyton)
    let mut pmes_slider = use_signal(|| 7.5); // Pressão Média de Enchimento Sistêmico basal (mmHg)
    let mut trigger_spiro = use_signal(|| false);
    let mut spiro_vef1 = use_signal(|| 3.80);
    let mut spiro_cvf = use_signal(|| 4.60);
    let mut spiro_tiff = use_signal(|| 82.6);
    let mut spiro_pef = use_signal(|| 8.50);
    let mut is_in_spiro = use_signal(|| false);

    // Parâmetros e Métricas Nefrológicas (Nefro Lab)
    let mut renal_stenosis = use_signal(|| 0.0);       // % estenose de artéria renal (0 a 100%)
    let mut renal_afferent_tone = use_signal(|| 1.0);  // tônus/resistência aferente (0.5 a 3.0x)
    let mut renal_raas_block = use_signal(|| false);   // bloqueio SRAA (IECA / BRA)
    let mut renal_loop_diuretic = use_signal(|| 0.0);  // diurético de alça / furosemida (0 a 100%)
    let mut renal_thiazide = use_signal(|| 0.0);      // tiazídico (0 a 100%)
    let mut renal_water_intake = use_signal(|| 2000.0);// mL/dia (500 a 4000)
    let mut trigger_bolus = use_signal(|| false);      // infusão rápida de bolus 500 mL
    let mut nefro_tfg = use_signal(|| 125.0);
    let mut nefro_du = use_signal(|| 60.0);
    let mut nefro_rpf = use_signal(|| 660.0);
    let mut nefro_ff = use_signal(|| 18.9);
    let mut nefro_vlec = use_signal(|| 15.0);
    let mut nefro_renal_map = use_signal(|| 90.0);
    let mut nefro_sodium = use_signal(|| 150.0);

    // Dynamic HUD metrics state
    let mut bpm = use_signal(|| 75.0);
    let mut pr = use_signal(|| 160.0);
    let mut qrs = use_signal(|| 90.0);
    let mut qt = use_signal(|| 400.0);
    let mut v_rest = use_signal(|| -85.0);
    let mut pr_rr = use_signal(|| 0.20);
    let mut edv = use_signal(|| 120.0);
    let mut esv = use_signal(|| 50.0);
    let mut sv = use_signal(|| 70.0);
    let mut ef = use_signal(|| 58.3);
    let mut co = use_signal(|| 4.8);
    let mut map = use_signal(|| 90.0);
    let mut cvp = use_signal(|| 3.2);
    let mut pmes = use_signal(|| 7.5);
    let mut pcp = use_signal(|| 8.1);
    let mut pi_c = use_signal(|| 27.5);
    let mut edema_pulm = use_signal(|| 0.0);
    let mut edema_godet = use_signal(|| 0.0);
    let mut spo2 = use_signal(|| 99.0);
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
        let mut ch3_plotter = Plotter::new("canvas-ch3", buffer_capacity); // Cálcio / Íons
        let mut hemo_plotter_lvp = Plotter::new("canvas-ch4", buffer_capacity); // LVP Ventricular (Ciano)
        let mut hemo_plotter_aop = Plotter::new("canvas-ch4", buffer_capacity); // AoP Aórtica (Coral)
        let mut hemo_plotter_lap = Plotter::new("canvas-ch4", buffer_capacity); // LAP Atrial (Amarelo Âmbar)
        let mut pv_plotter = PvLoopPlotter::new("canvas-pv", 700);              // Alça P x V 2D em plano de fase
        let guyton_plotter = GuytonPlotter::new("canvas-guyton");              // Diagrama de Guyton 2D (DC x RV)

        // Plotters Respiratórios Dedicados (Pulmo Lab)
        let mut resp_plotter_vol = Plotter::new("canvas-resp-vol", buffer_capacity);   // Espirograma V x t (Turquesa)
        let mut resp_plotter_flow = Plotter::new("canvas-resp-flow", buffer_capacity); // Fluxo V̇ x t (Amarelo)
        let mut resp_plotter_ppl = Plotter::new("canvas-resp-ppl", buffer_capacity);   // Pressão Intrapleural (Coral)
        let mut resp_plotter_rsa = Plotter::new("canvas-resp-rsa", buffer_capacity);   // Acoplamento Cardiorrespiratório RSA (Carmine)

        // Plotters Nefrológicos Dedicados (Nefro Lab)
        let mut nefro_plotter_gfr = Plotter::new("canvas-nefro-gfr", buffer_capacity);   // TFG (Ouro/Âmbar #f39c12)
        let mut nefro_plotter_uout = Plotter::new("canvas-nefro-uout", buffer_capacity); // Débito Urinário (Turquesa #00cec9)
        let mut nefro_plotter_rbf = Plotter::new("canvas-nefro-rbf", buffer_capacity);   // FPR (Coral #e17055)
        let mut nefro_plotter_vlec = Plotter::new("canvas-nefro-vlec", buffer_capacity); // VLEC (Lilás #a29bfe)

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
            hemo_plotter_lap.set_mode(current_mode);
            resp_plotter_vol.set_mode(current_mode);
            resp_plotter_flow.set_mode(current_mode);
            resp_plotter_ppl.set_mode(current_mode);
            resp_plotter_rsa.set_mode(current_mode);
            nefro_plotter_gfr.set_mode(current_mode);
            nefro_plotter_uout.set_mode(current_mode);
            nefro_plotter_rbf.set_mode(current_mode);
            nefro_plotter_vlec.set_mode(current_mode);

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
                hemo_plotter_lap.clear_ghost();
                resp_plotter_vol.clear_ghost();
                resp_plotter_flow.clear_ghost();
                resp_plotter_ppl.clear_ghost();
                resp_plotter_rsa.clear_ghost();
                nefro_plotter_gfr.clear_ghost();
                nefro_plotter_uout.clear_ghost();
                nefro_plotter_rbf.clear_ghost();
                nefro_plotter_vlec.clear_ghost();
                pv_plotter.reset();
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
                hemo_plotter_lap.capture_ghost();
                resp_plotter_vol.capture_ghost();
                resp_plotter_flow.capture_ghost();
                resp_plotter_ppl.capture_ghost();
                resp_plotter_rsa.capture_ghost();
                nefro_plotter_gfr.capture_ghost();
                nefro_plotter_uout.capture_ghost();
                nefro_plotter_rbf.capture_ghost();
                nefro_plotter_vlec.capture_ghost();
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
                hemo_plotter_lap.arm_single();
                resp_plotter_vol.arm_single();
                resp_plotter_flow.arm_single();
                resp_plotter_ppl.arm_single();
                resp_plotter_rsa.arm_single();
                nefro_plotter_gfr.arm_single();
                nefro_plotter_uout.arm_single();
                nefro_plotter_rbf.arm_single();
                nefro_plotter_vlec.arm_single();
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
                system.write().set_valvopathy_params(
                    aortic_stenosis() / 100.0,
                    aortic_regurg() / 100.0,
                    mitral_stenosis() / 100.0,
                    mitral_regurg() / 100.0,
                );
                system.write().set_microvascular_params(
                    albumin_slider(),
                    permeability_slider() / 100.0,
                );
                system.write().set_ecg_lead(ecg_lead_idx());
                system.write().set_ion_cell(ion_cell_idx());
                system.write().set_ion_var(ion_var_idx());

                // Parâmetros e Gatilhos Respiratórios
                if trigger_spiro() {
                    system.write().trigger_spirometry();
                    trigger_spiro.set(false);
                }
                system.write().set_respiratory_params(resp_rate(), resp_raw(), resp_crs());
                system.write().set_rsa_enabled(rsa_toggle());
                system.write().set_baroreflex_enabled(baro_toggle());
                system.write().set_orthostasis(orthostasis_toggle());
                system.write().set_pmes(pmes_slider());

                // Parâmetros e Gatilhos Nefrológicos (Nefro Lab)
                if trigger_bolus() {
                    system.write().infuse_fluid_bolus(500.0);
                    trigger_bolus.set(false);
                }
                system.write().set_renal_params(
                    renal_stenosis() / 100.0,
                    renal_afferent_tone(),
                    renal_raas_block(),
                    renal_loop_diuretic() / 100.0,
                    renal_thiazide() / 100.0,
                    renal_water_intake(),
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
                
                // Batch achatado de 21 canais: [sa, av, atr, purk, endo, epi, fib, cai, lvp, aop, ecg, sound_events, vol, flow, p_pl, v_lv, p_la, tfg, du, rpf, vlec]
                let chunk_size = HeartSystem::get_chunk_size();
                for chunk in batch.chunks(chunk_size) {
                    if chunk.len() == chunk_size {
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
                        let vol = chunk[12];
                        let flow = chunk[13];
                        let ppl = chunk[14];
                        let vol_lv = chunk[15];
                        let lap = chunk[16];
                        let tfg_val = chunk[17];
                        let du_val = chunk[18];
                        let rpf_val = chunk[19];
                        let vlec_val = chunk[20];
                        
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
                        hemo_plotter_lap.push(lap, sound_code, is_sa_fire);
                        pv_plotter.push(vol_lv, lvp);

                        // Traçados do Pulmo Lab
                        resp_plotter_vol.push(vol, sound_code, is_sa_fire);
                        resp_plotter_flow.push(flow, sound_code, is_sa_fire);
                        resp_plotter_ppl.push(ppl, sound_code, is_sa_fire);
                        resp_plotter_rsa.push(sa, sound_code, is_sa_fire);

                        // Traçados do Nefro Lab
                        nefro_plotter_gfr.push(tfg_val, sound_code, is_sa_fire);
                        nefro_plotter_uout.push(du_val, sound_code, is_sa_fire);
                        nefro_plotter_rbf.push(rpf_val, sound_code, is_sa_fire);
                        nefro_plotter_vlec.push(vlec_val, sound_code, is_sa_fire);

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
            let ghost = show_ghost();
            let uti_m = sound_uti();
            let bulhas_m = sound_bulhas();

            if active_tab() == "cardio" {
                // Flags de visibilidade das camadas celulares
                let any_epi = show_epi();
                let any_endo = show_endo();
                let any_purk = show_purkinje();
                let any_atrium = show_atrium();
                let any_av = show_av();
                let any_sa = show_sa();
                let any_fib = show_fibroblast();

                // O primeiro traçado a ser desenhado limpa o canvas e plota as linhas verticais de áudio
                let mut cleared = false;

                if any_sa {
                    pa_plotter_sa.draw(-90.0, 50.0, "#e74c3c", !cleared, ghost, uti_m, bulhas_m, false, None); // Nó SA: Vermelho (#e74c3c)
                    cleared = true;
                }
                if any_atrium {
                    pa_plotter_atrium.draw(-90.0, 50.0, "#3498db", !cleared, ghost, uti_m, bulhas_m, false, None); // Átrio: Azul (#3498db)
                    cleared = true;
                }
                if any_av {
                    pa_plotter_av.draw(-90.0, 50.0, "#f1c40f", !cleared, ghost, uti_m, bulhas_m, false, None); // Nó AV: Amarelo (#f1c40f)
                    cleared = true;
                }
                if any_purk {
                    pa_plotter_purk.draw(-90.0, 50.0, "#e67e22", !cleared, ghost, uti_m, bulhas_m, false, None); // Purkinje: Laranja (#e67e22)
                    cleared = true;
                }
                if any_endo {
                    pa_plotter_endo.draw(-90.0, 50.0, "#2ecc71", !cleared, ghost, uti_m, bulhas_m, false, None); // Endocárdio: Verde Clínico (#2ecc71)
                    cleared = true;
                }
                if any_epi {
                    pa_plotter_epi.draw(-90.0, 50.0, "#1abc9c", !cleared, ghost, uti_m, bulhas_m, false, None); // Epicárdio: Verde Menta (#1abc9c)
                    cleared = true;
                }
                if any_fib {
                    pa_plotter_fib.draw(-90.0, 50.0, "#a29bfe", !cleared, ghost, uti_m, bulhas_m, false, None); // Fibroblasto: Lilás (#a29bfe)
                    cleared = true;
                }

                // Se nenhuma camada estiver ativa, limpa a tela e plota os marcadores
                if !cleared {
                    pa_plotter_endo.draw(-90.0, 50.0, "#000000", true, false, uti_m, bulhas_m, false, None);
                }
                pa_plotter_endo.draw_scales(-90.0, 50.0, false, "+50 mV", "-20 mV", "-90 mV");
                
                // CH-02: ECG Calibrado em mV com Grade Isotrópica (40ms x 0.1mV)
                ecg_plotter.draw(-0.5, 1.5, "#00f2fe", true, ghost, uti_m, bulhas_m, true, None);
                ecg_plotter.draw_scales(-0.5, 1.5, true, "+1.5 mV", "0.0 mV", "-0.5 mV");

                // CH-03: Variável Iônica / Cinética Celular Dinâmica
                let (min_ch3, max_ch3, color_ch3, top_l, mid_l, bot_l) = match ion_var_idx() {
                    0 => (0.0, 2.0, "#a29bfe", "2.0 µM", "1.0 µM", "0.0 µM"),
                    1 => (0.0, 25.0, "#f1c40f", "25 mM", "12.5 mM", "0 mM"),
                    2 => (100.0, 160.0, "#e74c3c", "160 mM", "130 mM", "100 mM"),
                    3 => (0.0, 5.0, "#00cec9", "5.0 mM", "2.5 mM", "0.0 mM"),
                    4 => (-16.0, 2.0, "#fdcb6e", "+2 pA/pF", "-7 pA/pF", "-16 pA/pF"),
                    5 => (-80.0, 10.0, "#e17055", "+10 pA/pF", "-35 pA/pF", "-80 pA/pF"),
                    6 => (-1.0, 5.0, "#0984e3", "+5.0 pA/pF", "+2.0 pA/pF", "-1.0 pA/pF"),
                    7 => (-6.0, 1.0, "#00b894", "+1.0 pA/pF", "-2.5 pA/pF", "-6.0 pA/pF"),
                    _ => (0.0, 2.0, "#a29bfe", "2.0 µM", "1.0 µM", "0.0 µM"),
                };
                ch3_plotter.draw(min_ch3, max_ch3, color_ch3, true, ghost, uti_m, bulhas_m, false, None);
                ch3_plotter.draw_scales(min_ch3, max_ch3, false, top_l, mid_l, bot_l);
                
                // CH-04: Hemodinâmica de Wiggers: LVP (Ciano), AoP (Coral) e LAP (Amarelo Âmbar)
                hemo_plotter_lvp.draw(0.0, 180.0, "#00f2fe", true, ghost, uti_m, bulhas_m, false, None);
                hemo_plotter_aop.draw(0.0, 180.0, "#ff1754", false, ghost, uti_m, bulhas_m, false, None);
                hemo_plotter_lap.draw(0.0, 180.0, "#eab308", false, ghost, uti_m, bulhas_m, false, None);
                hemo_plotter_aop.draw_scales(0.0, 180.0, false, "180 mmHg", "90 mmHg", "0 mmHg");

                // CH-2D: Alça Pressão-Volume 2D (Plano de Fase) ou Diagrama de Guyton
                if two_d_mode() == "pv" {
                    pv_plotter.draw(edv(), esv(), ef(), sv());
                } else {
                    let sys_read = system.read();
                    guyton_plotter.draw(
                        sys_read.get_pmes(),
                        sys_read.get_r_rv(),
                        sys_read.get_inotropy(),
                        cvp(),
                        co(),
                        sys_read.get_venous_return(),
                    );
                }
            } else if active_tab() == "pulmo" {
                // PULMO LAB:
                // CH-01: Volume Pulmonar (V x t) de 0.0 a 7.0 L
                resp_plotter_vol.draw(0.0, 7.0, "#00f2fe", true, ghost, uti_m, bulhas_m, false, None);
                resp_plotter_vol.draw_scales(0.0, 7.0, false, "7.0 L (CPT)", "2.8 L (CRF)", "0.0 L");

                // CH-02: Fluxo Aéreo (V̇ x t) de -8.0 a 12.0 L/s
                resp_plotter_flow.draw(-8.0, 12.0, "#f1c40f", true, ghost, uti_m, bulhas_m, false, None);
                resp_plotter_flow.draw_scales(-8.0, 12.0, false, "+10 L/s (PEF)", "0.0 L/s", "-6.0 L/s");

                // CH-03: Pressão Intrapleural (P_pl x t) de -30.0 a 35.0 cmH2O
                resp_plotter_ppl.draw(-30.0, 35.0, "#ff7675", true, ghost, uti_m, bulhas_m, false, None);
                resp_plotter_ppl.draw_scales(-30.0, 35.0, false, "+30 cmH₂O", "0 cmH₂O", "-25 cmH₂O");

                // CH-04: Acoplamento Cardiorrespiratório RSA (Disparo Sinusal guiado pela Respiração)
                resp_plotter_rsa.draw(-90.0, 50.0, "#a29bfe", true, ghost, uti_m, bulhas_m, false, None);
                resp_plotter_rsa.draw_scales(-90.0, 50.0, false, "+50 mV (Nó SA)", "-20 mV", "-90 mV");
            } else {
                // NEFRO LAB:
                // CH-01: Taxa de Filtração Glomerular (TFG) de 0.0 a 160.0 mL/min
                nefro_plotter_gfr.draw(0.0, 160.0, "#f39c12", true, ghost, uti_m, bulhas_m, false, None);
                nefro_plotter_gfr.draw_scales(0.0, 160.0, false, "160 mL/min", "125 mL/min (TFG)", "0 mL/min");

                // CH-02: Débito Urinário Instantâneo (DU) de 0.0 a 400.0 mL/h
                nefro_plotter_uout.draw(0.0, 400.0, "#00cec9", true, ghost, uti_m, bulhas_m, false, None);
                nefro_plotter_uout.draw_scales(0.0, 400.0, false, "400 mL/h (Poliúria)", "60 mL/h (Basal)", "0 mL/h (Anúria)");

                // CH-03: Fluxo Plasmático Renal (FPR) de 0.0 a 900.0 mL/min
                nefro_plotter_rbf.draw(0.0, 900.0, "#e17055", true, ghost, uti_m, bulhas_m, false, None);
                nefro_plotter_rbf.draw_scales(0.0, 900.0, false, "900 mL/min", "660 mL/min (FPR)", "0 mL/min");

                // CH-04: Volume de Líquido Extracelular (VLEC) de 10.0 a 20.0 L
                nefro_plotter_vlec.draw(10.0, 20.0, "#a29bfe", true, ghost, uti_m, bulhas_m, false, None);
                nefro_plotter_vlec.draw_scales(10.0, 20.0, false, "20.0 L (Hipervolemia)", "15.0 L (Basal)", "10.0 L (Depleção)");
            }

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
                    edv.set(metrics.edv);
                    esv.set(metrics.esv);
                    sv.set(metrics.sv);
                    ef.set(metrics.ef);
                    co.set(metrics.co);
                    map.set(metrics.map);
                    cvp.set(metrics.cvp);
                    pmes.set(metrics.pmes);
                    pcp.set(metrics.pcp);
                    pi_c.set(metrics.pi_c);
                    edema_pulm.set(metrics.edema_pulm);
                    edema_godet.set(metrics.edema_godet);
                    spo2.set(metrics.spo2);
                    nefro_tfg.set(metrics.tfg);
                    nefro_du.set(metrics.diuresis);
                    nefro_rpf.set(metrics.rpf);
                    nefro_ff.set(metrics.ff * 100.0);
                    nefro_vlec.set(metrics.vlec);
                    nefro_renal_map.set(metrics.renal_map);
                    nefro_sodium.set(metrics.sodium_excretion);

                    spiro_vef1.set(system.read().get_vef1());
                    spiro_cvf.set(system.read().get_cvf());
                    spiro_tiff.set(system.read().get_tiffeneau());
                    spiro_pef.set(system.read().get_pef());
                    is_in_spiro.set(system.read().is_in_spirometry());
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
    let ef_str = format!("{:.1}", ef());
    let sv_str = format!("{:.0}", sv());
    let co_str = format!("{:.2}", co());
    let map_str = format!("{:.0}", map());
    let cvp_str = format!("{:.1}", cvp());
    let pmes_str = format!("{:.1}", pmes());
    let pcp_str = format!("{:.1}", pcp());
    let pic_str = format!("{:.1}", pi_c());
    let godet_str = if edema_godet() >= 0.5 {
        format!("{:.0}+", edema_godet())
    } else {
        "0".to_string()
    };
    let edema_pulm_str = if edema_pulm() > 0.01 {
        format!("{:.2} L", edema_pulm())
    } else {
        "0.0 L".to_string()
    };
    let spo2_str = format!("{:.0}", spo2());

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
        aortic_stenosis.set(0.0);
        aortic_regurg.set(0.0);
        mitral_stenosis.set(0.0);
        mitral_regurg.set(0.0);
        albumin_slider.set(4.0);
        permeability_slider.set(0.0);
        renal_stenosis.set(0.0);
        renal_afferent_tone.set(1.0);
        renal_raas_block.set(false);
        renal_loop_diuretic.set(0.0);
        renal_thiazide.set(0.0);
        renal_water_intake.set(2000.0);
        trigger_bolus.set(false);
        sound_uti.set(false);
        sound_bulhas.set(false);
        view_mode.set("rolling".to_string());
        show_ghost.set(false);
        trigger_capture_ghost.set(false);
        trigger_clear_ghost.set(true);
        trigger_arm_single.set(false);
        is_paused.set(false);
        ghost_status_str.set("📸 Capturar".to_string());
        resp_rate.set(15.0);
        resp_raw.set(1.5);
        resp_crs.set(0.10);
        rsa_toggle.set(true);
        baro_toggle.set(true);
        orthostasis_toggle.set(false);
        pmes_slider.set(7.5);
        two_d_mode.set("pv".to_string());
        trigger_spiro.set(false);
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
                div { class: "lab-tabs",
                    button {
                        class: if active_tab() == "cardio" { "lab-tab active" } else { "lab-tab" },
                        onclick: move |_| active_tab.set("cardio".to_string()),
                        "💓 CARDIO LAB"
                    }
                    button {
                        class: if active_tab() == "pulmo" { "lab-tab active pulmo" } else { "lab-tab" },
                        onclick: move |_| active_tab.set("pulmo".to_string()),
                        "🫁 PULMO LAB"
                    }
                    button {
                        class: if active_tab() == "nefro" { "lab-tab active nefro" } else { "lab-tab" },
                        onclick: move |_| active_tab.set("nefro".to_string()),
                        "🫘 NEFRO LAB"
                    }
                }
                div { class: "hud-metrics",
                    if active_tab() == "cardio" {
                        div { class: "hud-item", "心拍数 // FC: ", span { id: "hud-bpm", "{bpm_str} BPM" } }
                        div { class: "hud-item", "PR間隔 // PR: ", span { id: "hud-pr", "{pr_str}" } }
                        div { class: "hud-item", "QRS幅 // QRS: ", span { id: "hud-qrs", "{qrs_str}" } }
                        div { class: "hud-item", "QT間隔 // QT: ", span { id: "hud-qt", "{qt_str}" } }
                        div { class: "hud-item", "静止電位 // V.Rep: ", span { id: "hud-vrest", "{vrest_str}" } }
                        div { class: "hud-item", "PR/RR: ", span { id: "hud-pr-rr", "{pr_rr_str}" } }
                        div { class: "hud-item", "駆出率 // FE: ", span { id: "hud-ef", "{ef_str}%" } }
                        div { class: "hud-item", "一拍拍出量 // VS: ", span { id: "hud-sv", "{sv_str} mL" } }
                        div { class: "hud-item", "心拍出量 // DC: ", span { id: "hud-co", "{co_str} L/min" } }
                        div { class: "hud-item", "平均動脈圧 // PAM: ", span { id: "hud-map", "{map_str} mmHg" } }
                        div { class: "hud-item", "中心静脈圧 // PVC: ", span { id: "hud-cvp", "{cvp_str} mmHg" } }
                        div { class: "hud-item", "充満圧 // PMES: ", span { id: "hud-pmes", "{pmes_str} mmHg" } }
                        div { class: "hud-item", "毛細管圧 // Pcp: ", span { id: "hud-pcp", "{pcp_str} mmHg" } }
                        div { class: "hud-item", "膠質浸透圧 // πc: ", span { id: "hud-pic", "{pic_str} mmHg" } }
                        div { class: "hud-item", "浮腫 // Godet: ", span { id: "hud-godet", "{godet_str}" } }
                    } else if active_tab() == "pulmo" {
                        div { class: "hud-item", "呼吸数 // FR: ", span { id: "hud-rr", "{resp_rate():.0} irpm" } }
                        div { class: "hud-item", "一秒量 // VEF₁: ", span { id: "hud-vef1", "{spiro_vef1():.2} L" } }
                        div { class: "hud-item", "努力肺活量 // CVF: ", span { id: "hud-cvf", "{spiro_cvf():.2} L" } }
                        div { class: "hud-item", "ティフノー指数 // Tiffeneau: ", span { id: "hud-tiff", "{spiro_tiff():.1}%" } }
                        div { class: "hud-item", "最大呼気流速 // PEF: ", span { id: "hud-pef", "{spiro_pef():.2} L/s" } }
                        div { class: "hud-item", "心拍数 // FC: ", span { id: "hud-pulmo-bpm", "{bpm_str}" } }
                        div { class: "hud-item", "心拍出量 // DC: ", span { id: "hud-pulmo-co", "{co_str} L/min" } }
                        div { class: "hud-item", "気道抵抗 // Raw: ", span { id: "hud-raw", "{resp_raw():.1} cmH₂O" } }
                        div { class: "hud-item", "肺水腫 // Edema: ", span { id: "hud-edema-pulm", "{edema_pulm_str}" } }
                        div { class: "hud-item", "酸素飽和度 // SpO₂: ", span { id: "hud-spo2", "{spo2_str}%" } }
                    } else {
                        div { class: "hud-item", "濾過量 // TFG: ", span { id: "hud-tfg", "{nefro_tfg():.0} mL/min" } }
                        div { class: "hud-item", "時間尿量 // DU: ", span { id: "hud-du", "{nefro_du():.0} mL/h" } }
                        div { class: "hud-item", "血漿流量 // FPR: ", span { id: "hud-fpr", "{nefro_rpf():.0} mL/min" } }
                        div { class: "hud-item", "濾過率 // FF: ", span { id: "hud-ff", "{nefro_ff():.1}%" } }
                        div { class: "hud-item", "細胞外液 // VLEC: ", span { id: "hud-vlec", "{nefro_vlec():.2} L" } }
                        div { class: "hud-item", "腎灌流圧 // PAMr: ", span { id: "hud-pamr", "{nefro_renal_map():.0} mmHg" } }
                        div { class: "hud-item", "Na排泄 // Na⁺: ", span { id: "hud-na", "{nefro_sodium():.0} mEq/d" } }
                        div { class: "hud-item", "平均動脈圧 // PAM: ", span { id: "hud-nefro-pam", "{map_str} mmHg" } }
                    }
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

                if active_tab() == "cardio" {
                    Accordion { index: 0, label: "0. 画面表示 // Osciloscópio & Fantasma".to_string(), active_accordion,
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
                }

                Accordion { index: 1, label: "1. 細胞層 // Células & Camadas Ativas".to_string(), active_accordion,
                    div { class: "slider-header", style: "margin-bottom: 6px;",
                        span { style: "color: var(--neon-yellow); font-size: 1.35vh;", "Sobreposição de Potenciais no CH-01:" }
                    }
                    Checkbox { label: "Nó SA (Gatilho)".to_string(), color: "#e74c3c".to_string(), checked: show_sa, compact: true }
                    Checkbox { label: "Átrio (Contração)".to_string(), color: "#3498db".to_string(), checked: show_atrium, compact: true }
                    Checkbox { label: "Nó AV (Condução)".to_string(), color: "#f1c40f".to_string(), checked: show_av, compact: true }
                    Checkbox { label: "Purkinje (Feixe de His)".to_string(), color: "#e67e22".to_string(), checked: show_purkinje, compact: true }
                    Checkbox { label: "Endocárdio (Subendocárdico)".to_string(), color: "#2ecc71".to_string(), checked: show_endo, compact: true }
                    Checkbox { label: "Epicárdio (Subepicárdico)".to_string(), color: "#1abc9c".to_string(), checked: show_epi, compact: true }
                    Checkbox { label: "Fibroblasto (Eletrotônico)".to_string(), color: "#a29bfe".to_string(), checked: show_fibroblast, compact: true }
                }

                Accordion { index: 2, label: "2. イオン動態 // Cinética Iônica & Célula".to_string(), active_accordion,
                    div { style: "margin-bottom: 6px;",
                        div { class: "slider-header", style: "margin-bottom: 3px;",
                            span { style: "color: var(--neon-cyan); font-weight: bold; font-size: 1.35vh;", "Célula Inspecionada (CH-03):" }
                        }
                        select {
                            style: "width: 100%; padding: 4px 6px; border-radius: 4px; background: #161616; color: var(--text-main); border: 1px solid var(--neon-cyan); font-size: 1.3vh; cursor: pointer; margin-bottom: 6px;",
                            value: "{ion_cell_idx()}",
                            onchange: move |e| {
                                if let Ok(idx) = e.value().parse::<usize>() {
                                    ion_cell_idx.set(idx);
                                }
                            },
                            option { value: "4", selected: ion_cell_idx() == 4, "Endocárdio Ventricular (TP06)" }
                            option { value: "5", selected: ion_cell_idx() == 5, "Célula M Ventricular (TP06)" }
                            option { value: "6", selected: ion_cell_idx() == 6, "Epicárdio Ventricular (TP06)" }
                            option { value: "0", selected: ion_cell_idx() == 0, "Nó Sinoatrial (SA - Severi)" }
                            option { value: "1", selected: ion_cell_idx() == 1, "Músculo Atrial (Courtemanche)" }
                            option { value: "2", selected: ion_cell_idx() == 2, "Nó Atrioventricular (AV - Inada)" }
                            option { value: "3", selected: ion_cell_idx() == 3, "Fibras de Purkinje (Stewart)" }
                            option { value: "7", selected: ion_cell_idx() == 7, "Fibroblasto Cardíaco (MacCannell)" }
                        }

                        div { class: "slider-header", style: "margin-bottom: 3px;",
                            span { style: "color: var(--neon-yellow); font-weight: bold; font-size: 1.35vh;", "Fluxo / Concentração Iônica:" }
                        }
                        select {
                            style: "width: 100%; padding: 4px 6px; border-radius: 4px; background: #161616; color: var(--text-main); border: 1px solid var(--neon-yellow); font-size: 1.3vh; cursor: pointer;",
                            value: "{ion_var_idx()}",
                            onchange: move |e| {
                                if let Ok(idx) = e.value().parse::<usize>() {
                                    ion_var_idx.set(idx);
                                }
                            },
                            option { value: "0", selected: ion_var_idx() == 0, "[Ca²⁺]ᵢ Cálcio Citosólico (µM)" }
                            option { value: "1", selected: ion_var_idx() == 1, "[Na⁺]ᵢ Sódio Citosólico (mM)" }
                            option { value: "2", selected: ion_var_idx() == 2, "[K⁺]ᵢ Potássio Citosólico (mM)" }
                            option { value: "3", selected: ion_var_idx() == 3, "[Ca²⁺]ₛᵣ Cálcio no Retículo (mM)" }
                            option { value: "4", selected: ion_var_idx() == 4, "I_CaL Corrente de Cálcio L (pA/pF)" }
                            option { value: "5", selected: ion_var_idx() == 5, "I_Na Corrente Rápida de Sódio (pA/pF)" }
                            option { value: "6", selected: ion_var_idx() == 6, "I_K Corrente de Potássio (pA/pF)" }
                            option { value: "7", selected: ion_var_idx() == 7, "I_f Corrente Marcapasso Funny (pA/pF)" }
                        }
                    }
                }

                Accordion { index: 3, label: "3. 電解質 // Íons e Eletrólitos".to_string(), active_accordion,
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

                Accordion { index: 4, label: "4. 自律神経 // Sistema Nervoso Autônomo".to_string(), active_accordion,
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

                Accordion { index: 5, label: "5. 抗不整脈薬 // Fármacos Antiarrítmicos".to_string(), active_accordion,
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

                Accordion { index: 6, label: "6. 病態生理 // Condições Patológicas".to_string(), active_accordion,
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
                    Slider {
                        label: "Albumina Sérica (πc)".to_string(),
                        min: 1.0, max: 6.0, step: 0.1, default_val: 4.0, unit: " g/dL".to_string(),
                        help: Some("Determina a pressão coloidosmótica oncótica capilar (πc). Hipoalbuminemia severa (< 2.5 g/dL por desnutrição, cirrose ou síndrome nefrótica) colapsa a reabsorção capilar, culminando em anasarca e edema sistêmico.".to_string()),
                        val: albumin_slider
                    }
                    Slider {
                        label: "Permeabilidade Capilar (σ)".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Lesão endotelial por sepse, trauma ou choque inflamatório. Aumenta a condutância capilar Kf e reduz o coeficiente de reflexão de Staverman (σ), provocando edema não-cardiogênico / SARA / ARDS.".to_string()),
                        val: permeability_slider
                    }
                }

                Accordion { index: 7, label: "7. 生体音響 // Monitorização & Áudio".to_string(), active_accordion,
                    Checkbox { label: "🔊 Bip de Monitor (UTI - Onda R)".to_string(), color: "#2ecc71".to_string(), checked: sound_uti }
                    Checkbox { label: "🩺 Bulhas Cardíacas (B1 / B2)".to_string(), color: "#e74c3c".to_string(), checked: sound_bulhas }
                    Checkbox { label: "🫁 Arritmia Sinusal Respiratória (RSA)".to_string(), color: "#a29bfe".to_string(), checked: rsa_toggle }
                    Checkbox { label: "🎯 Barorreflexo em Malha Fechada (PAM)".to_string(), color: "#00cec9".to_string(), checked: baro_toggle }
                }

                Accordion { index: 8, label: "8. 弁膜症 // Valvopatias & Dinâmica Valvar".to_string(), active_accordion,
                    Slider {
                        label: "Estenose Aórtica".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Obstrução da via de saída do VE. Gera gradiente pressórico sistólico VE-Aorta e sobrecarga sistólica.".to_string()),
                        val: aortic_stenosis
                    }
                    Slider {
                        label: "Insuficiência Aórtica".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Refluxo diastólico da aorta para o VE. Queda acentuada da pressão diastólica aórtica e sobrecarga de volume.".to_string()),
                        val: aortic_regurg
                    }
                    Slider {
                        label: "Estenose Mitral".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Resistência ao enchimento ventricular diastólico. Causa congestão e hipertensão no átrio esquerdo.".to_string()),
                        val: mitral_stenosis
                    }
                    Slider {
                        label: "Insuficiência Mitral".to_string(),
                        min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                        help: Some("Regurgitação sistólica do VE para o átrio esquerdo gerando onda v patológica gigante.".to_string()),
                        val: mitral_regurg
                    }
                }

                Accordion { index: 9, label: "9. 循環平衡 // Retorno Venoso & Guyton".to_string(), active_accordion,
                    Checkbox {
                        label: "🚶 Ortostase (Em Pé / Tilt)".to_string(),
                        color: "#fdcb6e".to_string(),
                        checked: orthostasis_toggle
                    }
                    Slider {
                        label: "Volemia (PMES)".to_string(),
                        min: 1.0, max: 15.0, step: 0.5, default_val: 7.5, unit: " mmHg".to_string(),
                        help: Some("Pressão Média de Enchimento Sistêmico de Arthur Guyton. Determina a pré-carga e o retorno venoso às cavas. Reduzida na hemorragia e choque hipovolêmico (< 4 mmHg); aumentada na reposição volêmica (> 10 mmHg).".to_string()),
                        val: pmes_slider
                    }
                    button {
                        class: "btn-toggle-guyton-2d",
                        style: "width: 100%; margin-top: 6px; padding: 6px 8px; background: rgba(0, 242, 254, 0.12); border: 1px solid var(--neon-cyan); color: var(--neon-cyan); border-radius: 4px; font-weight: bold; cursor: pointer; font-size: 1.25vh; text-align: center; transition: all 0.2s;",
                        onclick: move |_| {
                            if two_d_mode() == "guyton" {
                                two_d_mode.set("pv".to_string());
                            } else {
                                two_d_mode.set("guyton".to_string());
                            }
                        },
                        if two_d_mode() == "guyton" { "🔄 Voltar para Alça P×V (VE)" } else { "📊 Exibir Diagrama de Guyton (CH-2D)" }
                    }
                }
                } else if active_tab() == "pulmo" {
                    Accordion { index: 10, label: "1. 換気力学 // Mecânica Ventilatória".to_string(), active_accordion,
                            Slider {
                                label: "Frequência Respiratória".to_string(),
                                min: 6.0, max: 40.0, step: 1.0, default_val: 15.0, unit: " irpm".to_string(),
                                help: Some("Frequência dos ciclos ventilatórios espontâneos (eupneia de repouso: 12-16 irpm).".to_string()),
                                val: resp_rate
                            }
                            Slider {
                                label: "Resistência Aérea (Raw)".to_string(),
                                min: 0.5, max: 12.0, step: 0.1, default_val: 1.5, unit: " cmH₂O/(L/s)".to_string(),
                                help: Some("Resistência friccional das vias aéreas traqueobrônquicas. Na asma e DPOC, Raw eleva-se (> 4-6 cmH₂O/(L/s)), retardando o esvaziamento alveolar e reduzindo VEF1 e Tiffeneau.".to_string()),
                                val: resp_raw
                            }
                            Slider {
                                label: "Complacência do Sistema (Crs)".to_string(),
                                min: 0.02, max: 0.25, step: 0.01, default_val: 0.10, unit: " L/cmH₂O".to_string(),
                                help: Some("Distensibilidade elástica pulmonar e da caixa torácica (Normal: ~0.10 L/cmH₂O). Reduzida na fibrose pulmonar e SDRA (< 0.05 L/cmH₂O); aumentada no enfisema (> 0.15 L/cmH₂O).".to_string()),
                                val: resp_crs
                            }
                            Checkbox {
                                label: "🫀 Modulação Autonômica RSA (Nó SA)".to_string(),
                                color: "#2ecc71".to_string(),
                                checked: rsa_toggle
                            }
                        }

                        Accordion { index: 11, label: "2. スパイロメトリー // Espirometria Forçada".to_string(), active_accordion,
                            button {
                                class: if is_in_spiro() { "btn-spiro-trigger active" } else { "btn-spiro-trigger" },
                                onclick: move |_| trigger_spiro.set(true),
                                if is_in_spiro() { "⏳ EXECUTANDO MANOBRA FORÇADA..." } else { "💨 INICIAR MANOBRA DE ESPIROMETRIA" }
                            }

                            div { class: "spiro-results-grid",
                                div { class: "spiro-card",
                                    span { class: "spiro-card-title", "VEF₁ (Volume 1º s)" }
                                    span { class: "spiro-card-val", "{spiro_vef1():.2} L" }
                                }
                                div { class: "spiro-card",
                                    span { class: "spiro-card-title", "CVF (Capacidade Vital)" }
                                    span { class: "spiro-card-val", "{spiro_cvf():.2} L" }
                                }
                                div { class: "spiro-card",
                                    span { class: "spiro-card-title", "VEF₁ / CVF (Tiffeneau)" }
                                    span {
                                        class: "spiro-card-val",
                                        style: if spiro_tiff() < 70.0 { "color: var(--neon-carmine);" } else { "color: #2ecc71;" },
                                        "{spiro_tiff():.1} %"
                                    }
                                }
                                div { class: "spiro-card",
                                    span { class: "spiro-card-title", "PEF (Pico Expiratório)" }
                                    span { class: "spiro-card-val", "{spiro_pef():.1} L/s" }
                                }
                            }

                            if spiro_tiff() < 70.0 {
                                div { class: "spiro-diag obstrutivo",
                                    "⚠️ DISTÚRBIO VENTILATÓRIO OBSTRUTIVO (Índice de Tiffeneau < 70%). Compatível com Asma Brônquica ou DPOC."
                                }
                            } else if edema_pulm() > 0.15 || spiro_cvf() < 3.5 {
                                div { class: "spiro-diag restritivo",
                                    if edema_pulm() > 0.15 {
                                        "⚠️ DISTÚRBIO RESTRITIVO POR EDEMA PULMONAR (Congestão Alveolar com redução de CVF e rigidez tecidual)."
                                    } else {
                                        "⚠️ SUGESTÃO DE PADRÃO RESTRITIVO (CVF reduzida com Tiffeneau preservado). Necessita CPT para confirmação (ex: Fibrose)."
                                    }
                                }
                            } else {
                                div { class: "spiro-diag normal",
                                    "✅ ESPIROMETRIA DENTRO DOS LIMITES DA NORMALIDADE (Relação VEF₁/CVF normal ≥ 70%)."
                                }
                            }
                        }

                        Accordion { index: 12, label: "3. 肺気量 // Volumes e Capacidades".to_string(), active_accordion,
                            table { class: "spiro-vol-table",
                                tbody {
                                    tr {
                                        td { "Volume Corrente (Vt):" }
                                        td { "0.50 L" }
                                    }
                                    tr {
                                        td { "Vol. Reserva Insp. (VRI):" }
                                        td { "3.00 L" }
                                    }
                                    tr {
                                        td { "Vol. Reserva Exp. (VRE):" }
                                        td { "1.10 L" }
                                    }
                                    tr {
                                        td { "Volume Residual (VR):" }
                                        td { "1.20 L" }
                                    }
                                    tr {
                                        td { "Cap. Residual Funcional (CRF):" }
                                        td { "2.30 L" }
                                    }
                                    tr {
                                        td { "Capacidade Vital (CV):" }
                                        td { "4.60 L" }
                                    }
                                    tr {
                                        td { "Capacidade Pulmonar Total (CPT):" }
                                        td { "5.80 L" }
                                    }
                                }
                            }
                        }
                } else {
                    Accordion { index: 20, label: "1. 糸球体動態 // Hemodinâmica & Autoregulação".to_string(), active_accordion,
                        Slider {
                            label: "Estenose de Artéria Renal".to_string(),
                            min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                            help: Some("Estenose aterosclerótica da artéria renal (modelo de Goldblatt). Reduz a pressão de perfusão nos glomérulos, retém sódio e desvia a curva de Guyton para a direita.".to_string()),
                            val: renal_stenosis
                        }
                        Slider {
                            label: "Tônus Aferente (Simpático)".to_string(),
                            min: 0.5, max: 3.0, step: 0.1, default_val: 1.0, unit: "x".to_string(),
                            help: Some("Resistência da arteríola aferente controlada pelo tônus simpático renal. Vasoconstrição aferente aguda reduz o FPR e a TFG protegendo contra hipertensão grave.".to_string()),
                            val: renal_afferent_tone
                        }
                        Checkbox {
                            label: "💊 Bloqueio SRAA (IECA / BRA)".to_string(),
                            color: "#fdcb6e".to_string(),
                            checked: renal_raas_block
                        }
                    }

                    Accordion { index: 21, label: "2. 尿細管薬理 // Farmacologia Tubular & Diuréticos".to_string(), active_accordion,
                        Slider {
                            label: "Diurético de Alça (Furosemida)".to_string(),
                            min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                            help: Some("Inibidor potente do cotransportador NKCC2 na alça de Henle. Aumenta a fração excretada de sódio e água em até 5x, aliviando congestão volêmica e edema de pulmão.".to_string()),
                            val: renal_loop_diuretic
                        }
                        Slider {
                            label: "Tiazídico (Hidroclorotiazida)".to_string(),
                            min: 0.0, max: 100.0, step: 1.0, default_val: 0.0, unit: "%".to_string(),
                            help: Some("Inibidor do cotransportador NCC no túbulo contorcido distal. Aumenta a natriurese moderada de longo prazo e reduz a resistência vascular periférica.".to_string()),
                            val: renal_thiazide
                        }
                    }

                    Accordion { index: 22, label: "3. 水塩出納 // Balanço Hidrossalino & Volemia".to_string(), active_accordion,
                        Slider {
                            label: "Ingesta Hídrica Diária".to_string(),
                            min: 500.0, max: 4000.0, step: 100.0, default_val: 2000.0, unit: " mL/dia".to_string(),
                            help: Some("Consumo diário de água e sais minerais. Em equilíbrio normotenso (2000 mL/dia), iguala exatamente o débito urinário somado às perdas insensíveis.".to_string()),
                            val: renal_water_intake
                        }
                        button {
                            class: "icon-btn",
                            style: "width: 100%; padding: 8px 10px; margin-top: 6px; background: rgba(243, 156, 18, 0.2); color: #f39c12; font-weight: bold; border: 1px solid #f39c12; border-radius: 4px; cursor: pointer; font-size: 1.35vh; text-align: center; transition: all 0.2s;",
                            onclick: move |_| trigger_bolus.set(true),
                            "💧 INFUNDIR BOLUS IV (500 mL CRISTALÓIDE)"
                        }
                    }
                }
            }

            main {
                if active_tab() == "cardio" {
                    div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-01 [ 活動電位 // POTENCIAIS DE AÇÃO CELULAR ]" }
                            canvas { id: "canvas-pa" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label",
                                span { "CH-02 [ 心電図 // ECG: " }
                                select {
                                    value: "{ecg_lead_idx()}",
                                    onchange: move |evt| {
                                        if let Ok(idx) = evt.value().parse::<usize>() {
                                            ecg_lead_idx.set(idx);
                                        }
                                    },
                                    option { value: "1", selected: ecg_lead_idx() == 1, "DII (Padrão de Monitor)" }
                                    option { value: "0", selected: ecg_lead_idx() == 0, "DI (Bipolar Frontal)" }
                                    option { value: "2", selected: ecg_lead_idx() == 2, "DIII (Bipolar Frontal)" }
                                    option { value: "3", selected: ecg_lead_idx() == 3, "aVR (Unipolar Aumentada)" }
                                    option { value: "4", selected: ecg_lead_idx() == 4, "aVL (Unipolar Aumentada)" }
                                    option { value: "5", selected: ecg_lead_idx() == 5, "aVF (Unipolar Aumentada)" }
                                    option { value: "6", selected: ecg_lead_idx() == 6, "V1 (Precordial Direita)" }
                                    option { value: "7", selected: ecg_lead_idx() == 7, "V2 (Precordial Anterosseptal)" }
                                    option { value: "8", selected: ecg_lead_idx() == 8, "V3 (Precordial Transicional)" }
                                    option { value: "9", selected: ecg_lead_idx() == 9, "V4 (Precordial Anterior)" }
                                    option { value: "10", selected: ecg_lead_idx() == 10, "V5 (Precordial Lateral Baixa)" }
                                    option { value: "11", selected: ecg_lead_idx() == 11, "V6 (Precordial Lateral Baixa)" }
                                }
                                span { " ]" }
                            }
                            canvas { id: "canvas-ecg" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label",
                                span { "CH-03 [ イオン動態 // " }
                                select {
                                    value: "{ion_cell_idx()}",
                                    onchange: move |evt| {
                                        if let Ok(idx) = evt.value().parse::<usize>() {
                                            ion_cell_idx.set(idx);
                                        }
                                    },
                                    option { value: "4", selected: ion_cell_idx() == 4, "Endocárdio (TP06)" }
                                    option { value: "5", selected: ion_cell_idx() == 5, "Célula M (TP06)" }
                                    option { value: "6", selected: ion_cell_idx() == 6, "Epicárdio (TP06)" }
                                    option { value: "0", selected: ion_cell_idx() == 0, "Nó SA (Severi)" }
                                    option { value: "1", selected: ion_cell_idx() == 1, "Átrio (Courtemanche)" }
                                    option { value: "2", selected: ion_cell_idx() == 2, "Nó AV (Inada)" }
                                    option { value: "3", selected: ion_cell_idx() == 3, "Purkinje (Stewart)" }
                                    option { value: "7", selected: ion_cell_idx() == 7, "Fibroblasto (MacCannell)" }
                                }
                                span { " ➔ " }
                                select {
                                    value: "{ion_var_idx()}",
                                    onchange: move |evt| {
                                        if let Ok(idx) = evt.value().parse::<usize>() {
                                            ion_var_idx.set(idx);
                                        }
                                    },
                                    option { value: "0", selected: ion_var_idx() == 0, "[Ca²⁺]ᵢ Cálcio (µM)" }
                                    option { value: "1", selected: ion_var_idx() == 1, "[Na⁺]ᵢ Sódio (mM)" }
                                    option { value: "2", selected: ion_var_idx() == 2, "[K⁺]ᵢ Potássio (mM)" }
                                    option { value: "3", selected: ion_var_idx() == 3, "[Ca²⁺]ₛᵣ Retículo (mM)" }
                                    option { value: "4", selected: ion_var_idx() == 4, "I_CaL Cálcio L (pA/pF)" }
                                    option { value: "5", selected: ion_var_idx() == 5, "I_Na Sódio (pA/pF)" }
                                    option { value: "6", selected: ion_var_idx() == 6, "I_K Potássio (pA/pF)" }
                                    option { value: "7", selected: ion_var_idx() == 7, "I_f Funny (pA/pF)" }
                                }
                                span { " ]" }
                            }
                            canvas { id: "canvas-ch3" }
                        }
                        div { class: "canvas-row-split",
                            div { class: "canvas-wrapper",
                                div { class: "canvas-label",
                                    span { "CH-04 [ 左室圧迫曲線 // WIGGERS: " }
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
                                    span { "&" }
                                    span { 
                                        style: "display: inline-flex; align-items: center; gap: 4px;",
                                        span { style: "display: inline-block; width: 8px; height: 8px; border-radius: 50%; background-color: #eab308; box-shadow: 0 0 6px #eab308;" }
                                        span { style: "color: #eab308;", "LAP" }
                                    }
                                    span { " ]" }
                                }
                                canvas { id: "canvas-ch4" }
                            }
                            div { class: "canvas-wrapper",
                                div { class: "canvas-label",
                                    span { "CH-2D [ " }
                                    select {
                                        id: "select-2d-mode",
                                        style: "background: #111; color: var(--neon-cyan); border: 1px solid var(--neon-cyan); border-radius: 3px; font-size: 1.2vh; padding: 1px 4px; cursor: pointer;",
                                        value: "{two_d_mode()}",
                                        onchange: move |evt| two_d_mode.set(evt.value()),
                                        option { value: "pv", selected: two_d_mode() == "pv", "圧力-容積 // Alça P×V (VE)" }
                                        option { value: "guyton", selected: two_d_mode() == "guyton", "循環平衡 // Guyton (DC × RV)" }
                                    }
                                    span { " ]" }
                                }
                                if two_d_mode() == "pv" {
                                    canvas { id: "canvas-pv" }
                                } else {
                                    canvas { id: "canvas-guyton" }
                                }
                            }
                        }
                } else if active_tab() == "pulmo" {
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-01 [ 呼吸気量 // ESPIROGRAMA CONTÍNUO (Volume x Tempo em Litros) ]" }
                            canvas { id: "canvas-resp-vol" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-02 [ 気流速度 // FLUXO AÉREO INSTANTÂNEO (V̇ x Tempo em L/s) ]" }
                            canvas { id: "canvas-resp-flow" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-03 [ 胸腔内圧 // PRESSÃO INTRAPLEURAL (P_pl x Tempo em cmH₂O) ]" }
                            canvas { id: "canvas-resp-ppl" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-04 [ 心肺カップリング // ACOPLAMENTO CARDIORRESPIRATÓRIO (RSA & Nó SA) ]" }
                            canvas { id: "canvas-resp-rsa" }
                        }
                } else {
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-01 [ 糸球体濾過量 // TAXA DE FILTRAÇÃO GLOMERULAR (TFG x Tempo em mL/min) ]" }
                            canvas { id: "canvas-nefro-gfr" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-02 [ 時間尿量 // DÉBITO URINÁRIO INSTANTÂNEO (DU x Tempo em mL/h) ]" }
                            canvas { id: "canvas-nefro-uout" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-03 [ 腎血流動態 // FLUXO PLASMÁTICO RENAL (FPR x Tempo em mL/min) ]" }
                            canvas { id: "canvas-nefro-rbf" }
                        }
                        div { class: "canvas-wrapper",
                            div { class: "canvas-label", "CH-04 [ 体液平衡 // VOLUME DE LÍQUIDO EXTRACELULAR (VLEC x Tempo em Litros) ]" }
                            canvas { id: "canvas-nefro-vlec" }
                        }
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
