pub mod severi;
pub mod inada;
pub mod courtemanche;
pub mod tentusscher;
pub mod purkinje;
pub mod fibroblast;

// Aqui ficará o gerenciador de estado global que orquestra as 4 células
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Pharmaco {
    pub ko: f64,
    pub cao: f64,
    pub nao: f64,
    pub block_na: f64,
    pub block_k: f64,
    pub block_ca: f64,
    pub block_nak: f64,
    pub symp: f64,
    pub parasymp: f64,
    pub isch: f64,
    pub fibrosis: f64,
}

impl Pharmaco {
    pub fn effective_ko(&self) -> f64 {
        if self.isch > 0.0 { self.ko + (self.isch * 6.0) } else { self.ko }
    }
    
    pub fn isch_block(&self) -> f64 {
        1.0 - (self.isch * 0.5)
    }

    pub fn ans_ca_modifier(&self) -> f64 {
        1.0 + (self.symp * 0.1) - (self.parasymp * 0.1)
    }
}

impl Default for Pharmaco {
    fn default() -> Self {
        Self {
            ko: 5.4,
            cao: 2.0,
            nao: 140.0,
            block_na: 1.0,
            block_k: 1.0,
            block_ca: 1.0,
            block_nak: 1.0,
            symp: 0.0,
            parasymp: 0.0,
            isch: 0.0,
            fibrosis: 0.0,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct HudMetrics {
    pub bpm: f64,
    pub pr: f64,
    pub qrs: f64,
    pub qt: f64,
    pub v_rest: f64,
    pub pr_rr: f64,
}

#[wasm_bindgen]
pub struct HeartSystem {
    sa_node: severi::SeveriCell,
    av_node: inada::InadaCell,
    atrium: courtemanche::AtriumCell,
    purkinje: purkinje::PurkinjeCell,
    vent_endo: tentusscher::VentricleCell,
    vent_m: tentusscher::VentricleCell,
    vent_epi: tentusscher::VentricleCell,
    fibroblast: fibroblast::FibroblastCell,
    time: f64,
    
    // Condução (Pingers & Estímulos Transitórios)
    timer_atrium: f64,
    timer_av: f64,
    timer_purk: f64,
    timer_endo: f64,
    timer_m: f64,
    timer_epi: f64,
    stim_until_atrium: f64,
    stim_until_purk: f64,
    stim_until_endo: f64,
    stim_until_m: f64,
    stim_until_epi: f64,
    sa_fired: bool,
    atrium_fired: bool,
    av_fired: bool,
    purk_fired: bool,
    endo_fired: bool,

    // Farmacologia
    pharm: Pharmaco,

    // HUD tracking states
    t_last_atr_beat: f64,
    t_last_vent_beat: f64,
    t_prev_vent_beat: f64,
    in_ap: bool,
    t_ap_start: f64,
    ap_v_peak: f64,
    v_min_cycle: f64,

    // Current smoothed metrics
    pub bpm: f64,
    pub pr: f64,
    pub qrs: f64,
    pub qt: f64,
    pub v_rest: f64,
    pub pr_rr: f64,
}

#[wasm_bindgen]
impl HeartSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sa_node: severi::SeveriCell::new(),
            av_node: inada::InadaCell::default(),
            atrium: courtemanche::AtriumCell::default(),
            purkinje: purkinje::PurkinjeCell::default(),
            vent_endo: tentusscher::VentricleCell::new_with_type(tentusscher::VentricleCellType::Endocardial),
            vent_m: tentusscher::VentricleCell::new_with_type(tentusscher::VentricleCellType::Midmyocardial),
            vent_epi: tentusscher::VentricleCell::new_with_type(tentusscher::VentricleCellType::Epicardial),
            fibroblast: fibroblast::FibroblastCell::default(),
            time: 0.0,
            timer_atrium: -1.0,
            timer_av: -1.0,
            timer_purk: -1.0,
            timer_endo: -1.0,
            timer_m: -1.0,
            timer_epi: -1.0,
            stim_until_atrium: -1.0,
            stim_until_purk: -1.0,
            stim_until_endo: -1.0,
            stim_until_m: -1.0,
            stim_until_epi: -1.0,
            sa_fired: false,
            atrium_fired: false,
            av_fired: false,
            purk_fired: false,
            endo_fired: false,
            pharm: Pharmaco::default(),
            t_last_atr_beat: -1.0,
            t_last_vent_beat: -1.0,
            t_prev_vent_beat: -1.0,
            in_ap: false,
            t_ap_start: 0.0,
            ap_v_peak: 0.0,
            v_min_cycle: -85.0,
            bpm: 75.0,
            pr: 160.0,
            qrs: 90.0,
            qt: 400.0,
            v_rest: -85.0,
            pr_rr: 0.20,
        }
    }

    pub fn update_params(&mut self, 
        ko: f64, cao: f64, nao: f64, 
        block_na: f64, block_k: f64, block_ca: f64, block_nak: f64,
        symp: f64, parasymp: f64, isch: f64, fibrosis: f64
    ) {
        self.pharm.ko = ko;
        self.pharm.cao = cao;
        self.pharm.nao = nao;
        self.pharm.block_na = block_na;
        self.pharm.block_k = block_k;
        self.pharm.block_ca = block_ca;
        self.pharm.block_nak = block_nak;
        self.pharm.symp = symp;
        self.pharm.parasymp = parasymp;
        self.pharm.isch = isch;
        self.pharm.fibrosis = fibrosis;
    }

    pub fn step(&mut self, dt: f64) {
        let prev_v_vent = self.vent_endo.v;
        let prev_v_atr = self.atrium.v;

        // Acoplamento eletrotônico miócito-fibroblasto (MacCannell et al., 2007)
        // Condutância juncional proporcional à fibrose (0.0 a 4.0 nS)
        let g_gap = self.pharm.fibrosis * 4.0;
        let i_gap_pa = self.fibroblast.step(dt, self.vent_epi.v, g_gap, &self.pharm);
        let i_gap_norm = i_gap_pa / 185.0; // Capacitância do miócito CM = 185 pF -> pA/pF
        self.vent_epi.i_gap = i_gap_norm;
        self.vent_m.i_gap = i_gap_norm * 0.6;
        self.vent_endo.i_gap = i_gap_norm * 0.2;

        // Gerenciamento dos pulsos transitórios de estímulo elétrico
        if self.stim_until_atrium > 0.0 {
            if self.time < self.stim_until_atrium {
                self.atrium.i_st = -2000.0;
            } else {
                self.atrium.i_st = 0.0;
                self.stim_until_atrium = -1.0;
            }
        } else {
            self.atrium.i_st = 0.0;
        }

        if self.stim_until_purk > 0.0 {
            if self.time < self.stim_until_purk {
                self.purkinje.i_stim = -40.0;
            } else {
                self.purkinje.i_stim = 0.0;
                self.stim_until_purk = -1.0;
            }
        } else {
            self.purkinje.i_stim = 0.0;
        }

        if self.stim_until_endo > 0.0 {
            if self.time < self.stim_until_endo {
                self.vent_endo.i_stim = -52.0;
            } else {
                self.vent_endo.i_stim = 0.0;
                self.stim_until_endo = -1.0;
            }
        } else {
            self.vent_endo.i_stim = 0.0;
        }

        if self.stim_until_m > 0.0 {
            if self.time < self.stim_until_m {
                self.vent_m.i_stim = -52.0;
            } else {
                self.vent_m.i_stim = 0.0;
                self.stim_until_m = -1.0;
            }
        } else {
            self.vent_m.i_stim = 0.0;
        }

        if self.stim_until_epi > 0.0 {
            if self.time < self.stim_until_epi {
                self.vent_epi.i_stim = -52.0;
            } else {
                self.vent_epi.i_stim = 0.0;
                self.stim_until_epi = -1.0;
            }
        } else {
            self.vent_epi.i_stim = 0.0;
        }

        // Passo de Integração (Forward Euler)
        // O nó SA (Severi) e Nó AV (Inada) foram modelados no artigo original em SEGUNDOS
        let dt_sec = dt / 1000.0;
        self.sa_node.step(dt_sec, &self.pharm);
        self.av_node.step(dt_sec, &self.pharm);
        
        // O Átrio (Courtemanche), Purkinje (Stewart) e Ventrículo Transmural (Ten Tusscher) em MILISSEGUNDOS
        self.atrium.step(dt, &self.pharm);
        self.purkinje.step(dt, &self.pharm);
        self.vent_endo.step(dt, &self.pharm);
        self.vent_m.step(dt, &self.pharm);
        self.vent_epi.step(dt, &self.pharm);
        
        self.time += dt;
        
        // --- SISTEMA DE CONDUÇÃO (Acoplamento Fisiológico com Dromotropismo Dinâmico) ---

        // Pinger 1: SA -> Átrio (Latência ~15ms modulada por canais de Sódio)
        if self.sa_node.v >= -20.0 && !self.sa_fired {
            let delay_sa_atr = 15.0 / self.pharm.block_na.max(0.2);
            self.timer_atrium = self.time + delay_sa_atr;
            self.sa_fired = true;
        } else if self.sa_node.v < -40.0 {
            self.sa_fired = false;
        }

        if self.timer_atrium > 0.0 && self.time >= self.timer_atrium {
            if self.atrium.v < -60.0 {
                self.stim_until_atrium = self.time + 2.0; // Injeção de corrente despolarizante (2ms)
                self.atrium.i_st = -2000.0;
            }
            self.timer_atrium = -1.0;
        }

        // Pinger 2: Átrio -> AV (Latência atrial de ~50ms até o Nó AV)
        if self.atrium.v >= -20.0 && !self.atrium_fired {
            let delay_atr_av = 50.0 * (1.0 + self.pharm.isch * 0.3) / self.pharm.block_na.max(0.2);
            self.timer_av = self.time + delay_atr_av;
            self.atrium_fired = true;
        } else if self.atrium.v < -60.0 {
            self.atrium_fired = false;
        }

        if self.timer_av > 0.0 && self.time >= self.timer_av {
            if self.av_node.v < -35.0 {
                self.av_node.v = -25.0; // Despolarização limiar do Nó AV (ativação de I_CaL)
            }
            self.timer_av = -1.0;
        }

        // Pinger 3: AV -> Purkinje / His
        // Dromotropismo fisiológico dinâmico:
        // - Condução decremental dependente de frequência (taquicardia aumenta o atraso)
        // - Tônus simpático encurta (dromotropismo +) e parassimpático alarga (dromotropismo -)
        // - Bloqueadores de cálcio (Verapamil) e isquemia aumentam o atraso ou bloqueiam (BAVT)
        if self.av_node.v >= -15.0 && !self.av_fired {
            let rate_factor = if self.bpm > 60.0 {
                1.0 + ((self.bpm - 60.0) / 120.0) * 0.30
            } else {
                1.0 - ((60.0 - self.bpm).min(30.0) / 60.0) * 0.10
            };
            let ans_factor = (1.0 - (self.pharm.symp * 0.25) + (self.pharm.parasymp * 0.40) + (self.pharm.isch * 0.50))
                / self.pharm.block_ca.max(0.15);
            let dynamic_his_delay = (25.0 * rate_factor * ans_factor).clamp(15.0, 150.0);

            // Bloqueio AV total se bloqueio de Ca2+ for crítico (< 0.35) ou isquemia severa (> 0.85)
            if self.pharm.block_ca >= 0.35 && self.pharm.isch <= 0.85 {
                self.timer_purk = self.time + dynamic_his_delay;
            }
            self.av_fired = true;
        } else if self.av_node.v < -45.0 {
            self.av_fired = false;
        }

        if self.timer_purk > 0.0 && self.time >= self.timer_purk {
            if self.purkinje.v < -60.0 {
                self.stim_until_purk = self.time + 1.5; // Injeção de corrente despolarizante (1.5ms)
                self.purkinje.i_stim = -40.0;
            }
            self.timer_purk = -1.0;
        }

        // Pinger 4: Purkinje -> Endocárdio (Condução rápida via rede subendocárdica ~12ms)
        if self.purkinje.v >= -15.0 && !self.purk_fired {
            let latency_purk_endo = 12.0 / self.pharm.block_na.max(0.2);
            self.timer_endo = self.time + latency_purk_endo;
            self.purk_fired = true;
        } else if self.purkinje.v < -60.0 {
            self.purk_fired = false;
        }

        if self.timer_endo > 0.0 && self.time >= self.timer_endo {
            if self.vent_endo.v < -60.0 {
                self.stim_until_endo = self.time + 1.5;
                self.vent_endo.i_stim = -52.0;
            }
            self.timer_endo = -1.0;
        }

        // Pinger 5: Condução Transmural Endocárdio -> Célula M (~6ms) -> Epicárdio (~12ms)
        if self.vent_endo.v >= -15.0 && !self.endo_fired {
            let delay_m = 6.0 / self.pharm.block_na.max(0.2);
            let delay_epi = 12.0 / self.pharm.block_na.max(0.2);
            self.timer_m = self.time + delay_m;
            self.timer_epi = self.time + delay_epi;
            self.endo_fired = true;
        } else if self.vent_endo.v < -60.0 {
            self.endo_fired = false;
        }

        if self.timer_m > 0.0 && self.time >= self.timer_m {
            if self.vent_m.v < -60.0 {
                self.stim_until_m = self.time + 1.5;
                self.vent_m.i_stim = -52.0;
            }
            self.timer_m = -1.0;
        }

        if self.timer_epi > 0.0 && self.time >= self.timer_epi {
            if self.vent_epi.v < -60.0 {
                self.stim_until_epi = self.time + 1.5;
                self.vent_epi.i_stim = -52.0;
            }
            self.timer_epi = -1.0;
        }

        // --- RASTREADOR FISIOLÓGICO DE MÉTRICAS (HUD) ---
        let v_vent = self.vent_endo.v;
        let v_atr = self.atrium.v;

        // Rastrear potencial mínimo diastólico
        if v_vent < self.v_min_cycle {
            self.v_min_cycle = v_vent;
        }

        // 1. Detecção de ativação atrial (Onda P)
        if prev_v_atr <= -20.0 && v_atr > -20.0 {
            self.t_last_atr_beat = self.time;
        }

        // 2. Detecção de despolarização ventricular (Complexo QRS)
        if prev_v_vent <= -20.0 && v_vent > -20.0 {
            self.t_prev_vent_beat = self.t_last_vent_beat;
            self.t_last_vent_beat = self.time;

            // Frequência Cardíaca (BPM) pelo intervalo RR
            if self.t_prev_vent_beat > 0.0 {
                let rr = self.t_last_vent_beat - self.t_prev_vent_beat;
                if rr > 200.0 && rr < 4000.0 {
                    let inst_bpm = 60000.0 / rr;
                    self.bpm = (self.bpm * 0.7) + (inst_bpm * 0.3);
                }
            }

            // Intervalo PR (Tempo do disparo atrial até o ventricular)
            if self.t_last_atr_beat > 0.0 {
                let pr_cand = self.t_last_vent_beat - self.t_last_atr_beat;
                if pr_cand > 40.0 && pr_cand < 500.0 {
                    self.pr = (self.pr * 0.7) + (pr_cand * 0.3);
                }
            }

            // Largura do QRS (base ~90ms, alargando com bloqueio de Sódio)
            let b_na = self.pharm.block_na.max(0.1);
            self.qrs = 90.0 / b_na;

            // Início do Potencial de Ação Ventricular
            self.in_ap = true;
            self.t_ap_start = self.time;
            self.ap_v_peak = v_vent;
            self.v_rest = self.v_min_cycle;
            self.v_min_cycle = 0.0;
        }

        // 3. Rastrear repolarização ventricular e intervalo QT pela Célula M (platô mais longo)
        if self.in_ap {
            if self.vent_m.v > self.ap_v_peak {
                self.ap_v_peak = self.vent_m.v;
            }
            let time_in_ap = self.time - self.t_ap_start;
            // Critério APD90: retorno a 90% do repouso em relação ao pico
            let repol_threshold = self.v_rest + 0.10 * (self.ap_v_peak - self.v_rest);
            let target_threshold = repol_threshold.max(-60.0);

            if time_in_ap > 100.0 && self.vent_m.v <= target_threshold {
                self.in_ap = false;
                self.qt = (self.qt * 0.7) + (time_in_ap * 0.3);
            } else if time_in_ap > 1200.0 {
                self.in_ap = false;
                self.qt = time_in_ap;
            }
        }

        // 4. Watchdog de Assistolia: se ficar sem bater por > 2.5 segundos, decai BPM
        if self.t_last_vent_beat > 0.0 && (self.time - self.t_last_vent_beat) > 2500.0 {
            self.bpm = (self.bpm * 0.98).max(0.0);
            if self.bpm < 1.0 {
                self.bpm = 0.0;
                self.pr = 0.0;
            }
        }

        // 5. Razão PR / RR
        if self.bpm > 0.0 {
            let est_rr = 60000.0 / self.bpm;
            self.pr_rr = self.pr / est_rr;
        } else {
            self.pr_rr = 0.0;
        }
    }

    // Roda um lote completo de cálculos no lado do Rust e retorna um array f64 achatado!
    pub fn run_batch(&mut self, dt: f64, steps: usize, downsample: usize) -> Vec<f64> {
        let mut batch = Vec::with_capacity((steps / downsample) * 10);
        for i in 0..steps {
            self.step(dt);
            if i % downsample == 0 {
                batch.push(self.sa_node.v);        // 0: SA
                batch.push(self.av_node.v);        // 1: AV
                batch.push(self.atrium.v);         // 2: Atrium
                batch.push(self.purkinje.v);       // 3: Purkinje
                batch.push(self.vent_endo.v);      // 4: Endocárdio
                batch.push(self.vent_epi.v);       // 5: Epicárdio
                batch.push(self.fibroblast.v);     // 6: Fibroblasto (MacCannell 2007)
                batch.push(self.vent_endo.ca_i);   // 7: Transiente de Cálcio
                batch.push(self.vent_endo.force);  // 8: Força de contração
                batch.push(self.compute_ecg());    // 9: ECG Dipolar Transmural
            }
        }
        batch
    }

    // Métodos de leitura individuais
    pub fn get_sa_v(&self) -> f64 { self.sa_node.v }
    pub fn get_av_v(&self) -> f64 { self.av_node.v }
    pub fn get_atrium_v(&self) -> f64 { self.atrium.v }
    pub fn get_purkinje_v(&self) -> f64 { self.purkinje.v }
    pub fn get_ventricle_v(&self) -> f64 { self.vent_endo.v }
    pub fn get_vent_endo_v(&self) -> f64 { self.vent_endo.v }
    pub fn get_vent_m_v(&self) -> f64 { self.vent_m.v }
    pub fn get_vent_epi_v(&self) -> f64 { self.vent_epi.v }
    pub fn get_fibroblast_v(&self) -> f64 { self.fibroblast.v }
    pub fn get_time(&self) -> f64 { self.time }

    pub fn compute_ecg(&self) -> f64 {
        let p_wave = (self.atrium.v + 80.0) * 0.15;
        let transmural = (self.vent_endo.v - self.vent_epi.v) * 0.55 + (self.vent_m.v - self.vent_epi.v) * 0.25;
        p_wave + transmural
    }

    pub fn get_hud_metrics(&self) -> HudMetrics {
        HudMetrics {
            bpm: self.bpm,
            pr: self.pr,
            qrs: self.qrs,
            qt: self.qt,
            v_rest: self.v_rest,
            pr_rr: self.pr_rr,
        }
    }
}
