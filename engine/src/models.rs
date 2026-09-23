pub mod severi;
pub mod inada;
pub mod courtemanche;
pub mod tentusscher;
pub mod purkinje;
pub mod fibroblast;
pub mod hemodynamics;
pub mod respiratory;
pub mod baroreflex;
pub mod microvascular;

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
    pub edv: f64,
    pub esv: f64,
    pub sv: f64,
    pub ef: f64,
    pub co: f64,
    pub map: f64,
    pub cvp: f64,
    pub pmes: f64,
    pub pcp: f64,
    pub pi_c: f64,
    pub edema_pulm: f64,
    pub edema_godet: f64,
    pub spo2: f64,
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
    hemo: hemodynamics::HemodynamicsModel,
    resp: respiratory::RespiratorySystem,
    baro: baroreflex::BaroreflexModel,
    micro: microvascular::MicrovascularModel,
    acc_r_peak: bool,
    acc_b1: bool,
    acc_b2: bool,
    acc_sa_fire: bool,
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
    t_t_wave: f64,
    ecg_lead: usize,
    prev_v_m: f64,
    pub ion_cell: usize,
    pub ion_var: usize,
    pub rsa_enabled: bool,

    // Current smoothed metrics
    pub bpm: f64,
    pub pr: f64,
    pub qrs: f64,
    pub qt: f64,
    pub v_rest: f64,
    pub pr_rr: f64,
}

pub const BATCH_CHUNK_SIZE: usize = 17;

#[wasm_bindgen]
impl HeartSystem {
    pub fn get_chunk_size() -> usize {
        BATCH_CHUNK_SIZE
    }

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
            hemo: hemodynamics::HemodynamicsModel::new(),
            resp: respiratory::RespiratorySystem::new(),
            baro: baroreflex::BaroreflexModel::new(),
            micro: microvascular::MicrovascularModel::new(),
            acc_r_peak: false,
            acc_b1: false,
            acc_b2: false,
            acc_sa_fire: false,
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
            t_t_wave: -1.0,
            ecg_lead: 1, // DII como derivação padrão
            prev_v_m: -85.0,
            ion_cell: 4, // Endocárdio como padrão
            ion_var: 0,  // [Ca2+]_i como padrão
            rsa_enabled: false,
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
        let prev_v_m = self.prev_v_m;

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
        // 1. Barorreflexo Arterial em Malha Fechada (após estabilização dos batimentos iniciais)
        if self.time > 1500.0 {
            // Na posição ortostática (em pé), a coluna hidrostática gravitacional entre a raiz aórtica
            // e a bifurcação do seio carotídeo (~35 cm) impõe um gradiente hidrostático de ~18 mmHg (ΔP = ρ·g·h).
            // Isso descarrega os barorreceptores carotídeos e deflagra a compensação autonômica postural imediata.
            let hydrostatic_carotid_offset = if self.hemo.orthostasis { 18.0 } else { 0.0 };
            let p_baro_sensed = (self.hemo.p_ao - hydrostatic_carotid_offset).max(20.0);
            self.baro.step(dt, p_baro_sensed);
        }

        // 2. Dinâmica Respiratória & Acoplamento Cardiorrespiratório (RSA)
        self.resp.step(dt);

        // Modulação autonômica integrada (Farmacologia basal/manual + compensação barorreflexa)
        let mut eff_pharm = self.pharm;     // Para células ventriculares e nó AV
        let mut eff_pharm_hemo = self.pharm; // Para hemodinâmica (vasoconstrição R_tpr completa)

        if self.baro.enabled && self.time > 2000.0 {
            // Barorreflexo — separação de efeitos fisiológicos por via eferente:
            //
            // 1. Vagal (delta_parasymp): age no nó SA e no AV → bradicardia reflexa sob hipertensão
            //    ach = parasymp * 1e-3 no Severi → delta_parasymp=0.015 → ach=1.5e-5 µM (adequado)
            eff_pharm.parasymp = (eff_pharm.parasymp + self.baro.delta_parasymp).clamp(0.0, 1.0);

            // 2. Simpático cronótropo (SA node): ESCALA REDUZIDA de 0.02x.
            //    No Severi, iso = p.symp em µM. O range fisiológico é 0.001–0.010 µM.
            //    delta_symp=0.25 × 0.02 = 0.005 µM ISO → +4-6 BPM (taquicardia leve reflexa)
            //    Sem escala (iso=0.10 µM), os gates de ICaL/If saturam paradoxalmente.
            let baro_chron = self.baro.delta_symp * 0.02;
            eff_pharm.symp = (eff_pharm.symp + baro_chron).clamp(0.0, 1.0);

            // 3. Simpático vasomotor (hemodinâmica): delta_symp completo → R_tpr ↑ (vasoconstrição)
            //    Este é o mecanismo primário de restauração de PAM pelo barorreflexo.
            eff_pharm_hemo.symp = (eff_pharm_hemo.symp + self.baro.delta_symp).clamp(0.0, 1.0);
            eff_pharm_hemo.parasymp = (eff_pharm_hemo.parasymp + self.baro.delta_parasymp).clamp(0.0, 1.0);
        }

        // O nó SA recebe modulação autonômica do barorreflexo e da RSA
        let mut eff_pharm_sa = self.pharm;
        if self.baro.enabled && self.time > 2000.0 {
            // Reflexo taquicardizante simpático no nó SA sob hipotensão (desinibição adrenérgica)
            let sa_symp = self.baro.delta_symp * 0.35;
            eff_pharm_sa.symp = (eff_pharm_sa.symp + sa_symp).clamp(0.0, 1.0);
        }

        if self.rsa_enabled && self.time > 1000.0 {
            if self.resp.f_rsa > 0.0 {
                // Inspiração: inibição vagal fisiológica com aceleração sinusal transitória (~ +3 a +4 BPM)
                eff_pharm_sa.symp = (eff_pharm_sa.symp + self.resp.f_rsa * 0.25).clamp(0.0, 100.0);
                eff_pharm_sa.parasymp = (eff_pharm_sa.parasymp - self.resp.f_rsa * 0.5).max(0.0);
            }
        }

        // O nó SA (Severi) e Nó AV (Inada) foram modelados no artigo original em SEGUNDOS
        let dt_sec = dt / 1000.0;
        self.sa_node.step(dt_sec, &eff_pharm_sa);
        self.av_node.step(dt_sec, &eff_pharm);
        
        // O Átrio (Courtemanche), Purkinje (Stewart) e Ventrículo Transmural (Ten Tusscher) em MILISSEGUNDOS
        self.atrium.step(dt, &eff_pharm);
        self.purkinje.step(dt, &eff_pharm);
        self.vent_endo.step(dt, &eff_pharm);
        self.vent_m.step(dt, &eff_pharm);
        self.vent_epi.step(dt, &eff_pharm);
        
        self.time += dt;
        
        // --- SISTEMA DE CONDUÇÃO (Acoplamento Fisiológico com Dromotropismo Dinâmico) ---

        // Pinger 1: SA -> Átrio (Latência ~15ms modulada por canais de Sódio)
        if self.sa_node.v >= -20.0 && !self.sa_fired {
            let delay_sa_atr = 15.0 / self.pharm.block_na.max(0.2);
            self.timer_atrium = self.time + delay_sa_atr;
            self.sa_fired = true;
            self.acc_sa_fire = true;
        } else if self.sa_node.v < -40.0 {
            self.sa_fired = false;
        }

        if self.timer_atrium > 0.0 && self.time >= self.timer_atrium {
            if self.atrium.v < -45.0 {
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
        } else if self.atrium.v < -45.0 {
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
        // Bloqueadores de cálcio (Verapamil) e isquemia aumentam o atraso ou bloqueiam (BAVT)
        if self.av_node.v >= -20.0 && !self.av_fired {
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
        } else if self.av_node.v < -40.0 {
            self.av_fired = false;
        }

        if self.timer_purk > 0.0 && self.time >= self.timer_purk {
            if self.purkinje.v < -45.0 {
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
        } else if self.purkinje.v < -45.0 {
            self.purk_fired = false;
        }

        if self.timer_endo > 0.0 && self.time >= self.timer_endo {
            if self.vent_endo.v < -45.0 {
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
        } else if self.vent_endo.v < -45.0 {
            self.endo_fired = false;
        }

        if self.timer_m > 0.0 && self.time >= self.timer_m {
            if self.vent_m.v < -45.0 {
                self.stim_until_m = self.time + 1.5;
                self.vent_m.i_stim = -52.0;
            }
            self.timer_m = -1.0;
        }

        if self.timer_epi > 0.0 && self.time >= self.timer_epi {
            if self.vent_epi.v < -45.0 {
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
            self.t_t_wave = -1.0;
            self.ap_v_peak = v_vent;
            self.v_rest = self.v_min_cycle;
            self.v_min_cycle = 0.0;
        }

        // 3. Rastrear repolarização ventricular e intervalo QT pela Célula M (platô mais longo)
        if self.in_ap {
            if self.vent_m.v > self.ap_v_peak {
                self.ap_v_peak = self.vent_m.v;
            }

            // Gatilho biofísico da onda T: Fase 3 de repolarização ao cruzar -10mV descendente
            if prev_v_m >= -10.0 && self.vent_m.v < -10.0 {
                self.t_t_wave = self.time;
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

        // --- HEMODINÂMICA E ACOPLAMENTO ELETROMECÂNICO ---
        self.hemo.step(
            dt,
            self.vent_endo.ca_i,
            self.atrium.v,
            self.vent_endo.v,
            eff_pharm_hemo.symp,
            eff_pharm_hemo.parasymp,
        );

        if self.hemo.event_r_peak { self.acc_r_peak = true; }
        if self.hemo.event_b1 { self.acc_b1 = true; }
        if self.hemo.event_b2 { self.acc_b2 = true; }

        // --- MICROCIRCULAÇÃO, EQUILÍBRIO DE STARLING E DINÂMICA DE EDEMA ---
        self.micro.step(dt, self.hemo.p_la, self.hemo.cvp);
        self.resp.set_edema_stiffness(self.micro.get_crs_stiffness_factor());

        // 5. Razão PR / RR
        if self.bpm > 0.0 {
            let est_rr = 60000.0 / self.bpm;
            self.pr_rr = self.pr / est_rr;
        } else {
            self.pr_rr = 0.0;
        }

        self.prev_v_m = self.vent_m.v;
    }

    // Roda um lote completo de cálculos no lado do Rust e retorna um array f64 achatado!
    pub fn run_batch(&mut self, dt: f64, steps: usize, downsample: usize) -> Vec<f64> {
        let chunk_size = BATCH_CHUNK_SIZE;
        let mut batch = Vec::with_capacity((steps / downsample) * chunk_size);
        for i in 0..steps {
            self.step(dt);
            if i % downsample == 0 {
                let sound_code = (if self.acc_r_peak { 1.0 } else { 0.0 })
                    + (if self.acc_b1 { 2.0 } else { 0.0 })
                    + (if self.acc_b2 { 4.0 } else { 0.0 })
                    + (if self.acc_sa_fire { 8.0 } else { 0.0 });
                self.acc_r_peak = false;
                self.acc_b1 = false;
                self.acc_b2 = false;
                self.acc_sa_fire = false;

                batch.push(self.sa_node.v);        // 0: SA
                batch.push(self.av_node.v);        // 1: AV
                batch.push(self.atrium.v);         // 2: Atrium
                batch.push(self.purkinje.v);       // 3: Purkinje
                batch.push(self.vent_endo.v);      // 4: Endocárdio
                batch.push(self.vent_epi.v);       // 5: Epicárdio
                batch.push(self.fibroblast.v);     // 6: Fibroblasto (MacCannell 2007)
                batch.push(self.compute_selected_ion()); // 7: Variável Iônica / Corrente Celular Selecionada
                batch.push(self.hemo.p_lv);        // 8: Pressão Ventricular Esquerda (LVP, mmHg)
                batch.push(self.hemo.p_ao);        // 9: Pressão Aórtica (AoP, mmHg)
                batch.push(self.compute_ecg());    // 10: ECG Dipolar Transmural
                batch.push(sound_code);            // 11: Eventos Acústicos (Bitmask)
                batch.push(self.resp.vol);         // 12: Volume Pulmonar (L)
                batch.push(self.resp.flow);        // 13: Fluxo Aéreo Instantâneo (L/s)
                batch.push(self.resp.p_pl);        // 14: Pressão Intrapleural (cmH2O)
                batch.push(self.hemo.v_lv);        // 15: Volume Ventricular Esquerdo (mL)
                batch.push(self.hemo.p_la);        // 16: Pressão Atrial Esquerda (LAP, mmHg)
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
    pub fn get_lvp(&self) -> f64 { self.hemo.p_lv }
    pub fn get_aop(&self) -> f64 { self.hemo.p_ao }
    pub fn get_lvv(&self) -> f64 { self.hemo.v_lv }
    pub fn get_hemo_edv(&self) -> f64 { self.hemo.v_edv }
    pub fn get_hemo_esv(&self) -> f64 { self.hemo.v_esv }
    pub fn get_hemo_sv(&self) -> f64 { self.hemo.stroke_volume }
    pub fn get_hemo_ef(&self) -> f64 { self.hemo.ejection_fraction }
    pub fn get_hemo_co(&self) -> f64 { (self.bpm * self.hemo.stroke_volume) / 1000.0 }
    pub fn get_hemo_lap(&self) -> f64 { self.hemo.p_la }
    pub fn get_time(&self) -> f64 { self.time }

    pub fn trigger_spirometry(&mut self) {
        self.resp.trigger_spirometry();
    }

    pub fn set_respiratory_params(&mut self, rate: f64, raw: f64, c_rs: f64) {
        self.resp.resp_rate = rate;
        self.resp.raw = raw;
        self.resp.c_rs = c_rs;
    }

    pub fn get_resp_vol(&self) -> f64 { self.resp.vol }
    pub fn get_resp_flow(&self) -> f64 { self.resp.flow }
    pub fn get_resp_ppl(&self) -> f64 { self.resp.p_pl }
    pub fn get_vef1(&self) -> f64 { self.resp.vef1 }
    pub fn get_cvf(&self) -> f64 { self.resp.cvf }
    pub fn get_tiffeneau(&self) -> f64 { self.resp.tiffeneau }
    pub fn get_pef(&self) -> f64 { self.resp.pef }
    pub fn is_in_spirometry(&self) -> bool { self.resp.maneuver_state != respiratory::ManeuverState::Idle }

    pub fn set_rsa_enabled(&mut self, enabled: bool) {
        self.rsa_enabled = enabled;
    }

    pub fn get_rsa_enabled(&self) -> bool {
        self.rsa_enabled
    }

    pub fn set_ion_cell(&mut self, cell: usize) {
        self.ion_cell = cell % 8;
    }

    pub fn get_ion_cell(&self) -> usize {
        self.ion_cell
    }

    pub fn set_ion_var(&mut self, var: usize) {
        self.ion_var = var % 8;
    }

    pub fn get_ion_var(&self) -> usize {
        self.ion_var
    }

    pub fn compute_selected_ion(&self) -> f64 {
        match self.ion_var {
            0 => {
                // [Ca2+]_i em micromolar (µM)
                match self.ion_cell {
                    0 => self.sa_node.s[1] * 1000.0,
                    1 => self.atrium.ca_i * 1000.0,
                    2 => self.av_node.cai * 1000.0,
                    3 => self.purkinje.ca_i * 1000.0,
                    4 => self.vent_endo.ca_i * 1000.0,
                    5 => self.vent_m.ca_i * 1000.0,
                    6 => self.vent_epi.ca_i * 1000.0,
                    _ => 0.1, // Fibroblasto basal
                }
            }
            1 => {
                // [Na+]_i em millimolar (mM)
                match self.ion_cell {
                    0 => self.sa_node.s[2],
                    1 => self.atrium.na_i,
                    2 => self.av_node.nai,
                    3 => self.purkinje.na_i,
                    4 => self.vent_endo.na_i,
                    5 => self.vent_m.na_i,
                    6 => self.vent_epi.na_i,
                    _ => 8.55,
                }
            }
            2 => {
                // [K+]_i em millimolar (mM)
                match self.ion_cell {
                    0 => self.sa_node.s[3],
                    1 => self.atrium.k_i,
                    2 => self.av_node.ki,
                    3 => self.purkinje.k_i,
                    4 => self.vent_endo.k_i,
                    5 => self.vent_m.k_i,
                    6 => self.vent_epi.k_i,
                    _ => 140.0,
                }
            }
            3 => {
                // [Ca2+]_SR em millimolar (mM)
                match self.ion_cell {
                    0 => self.sa_node.s[23],
                    1 => self.atrium.ca_rel,
                    2 => self.av_node.ca_rel,
                    3 => self.purkinje.ca_sr,
                    4 => self.vent_endo.ca_sr,
                    5 => self.vent_m.ca_sr,
                    6 => self.vent_epi.ca_sr,
                    _ => 0.0,
                }
            }
            4 => {
                // Corrente I_CaL estimada (pA/pF)
                match self.ion_cell {
                    0 => -self.sa_node.a[72].abs() * 0.1,
                    1 => -12.0 * self.atrium.d * self.atrium.f * ((self.atrium.v - 65.0) / 60.0).clamp(-1.0, 1.0),
                    2 => -8.0 * self.av_node.d_gate * self.av_node.f_gate * ((self.av_node.v - 60.0) / 60.0).clamp(-1.0, 1.0),
                    3 => -14.0 * self.purkinje.d * self.purkinje.f * ((self.purkinje.v - 65.0) / 60.0).clamp(-1.0, 1.0),
                    4 => -15.0 * self.vent_endo.d * self.vent_endo.f * ((self.vent_endo.v - 65.0) / 60.0).clamp(-1.0, 1.0),
                    5 => -15.0 * self.vent_m.d * self.vent_m.f * ((self.vent_m.v - 65.0) / 60.0).clamp(-1.0, 1.0),
                    6 => -15.0 * self.vent_epi.d * self.vent_epi.f * ((self.vent_epi.v - 65.0) / 60.0).clamp(-1.0, 1.0),
                    _ => 0.0,
                }
            }
            5 => {
                // Corrente I_Na estimada (pA/pF)
                match self.ion_cell {
                    0 => -self.sa_node.a[68].abs() * 0.05,
                    1 => -60.0 * self.atrium.m.powi(3) * self.atrium.h * self.atrium.j * ((self.atrium.v - 70.0) / 70.0).clamp(-1.0, 1.0),
                    2 => -20.0 * self.av_node.m_gate.powi(3) * self.av_node.h1_gate * ((self.av_node.v - 65.0) / 65.0).clamp(-1.0, 1.0),
                    3 => -75.0 * self.purkinje.m.powi(3) * self.purkinje.h * self.purkinje.j * ((self.purkinje.v - 70.0) / 70.0).clamp(-1.0, 1.0),
                    4 => -70.0 * self.vent_endo.m.powi(3) * self.vent_endo.h * self.vent_endo.j * ((self.vent_endo.v - 70.0) / 70.0).clamp(-1.0, 1.0),
                    5 => -70.0 * self.vent_m.m.powi(3) * self.vent_m.h * self.vent_m.j * ((self.vent_m.v - 70.0) / 70.0).clamp(-1.0, 1.0),
                    6 => -70.0 * self.vent_epi.m.powi(3) * self.vent_epi.h * self.vent_epi.j * ((self.vent_epi.v - 70.0) / 70.0).clamp(-1.0, 1.0),
                    _ => 0.0,
                }
            }
            6 => {
                // Corrente I_K repolarizante estimada (pA/pF)
                match self.ion_cell {
                    0 => self.sa_node.a[76] * 0.2,
                    1 => 2.5 * self.atrium.xr * ((self.atrium.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    2 => 2.0 * self.av_node.paf_gate * ((self.av_node.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    3 => 3.0 * self.purkinje.xr1 * self.purkinje.xr2 * ((self.purkinje.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    4 => 3.2 * self.vent_endo.xr1 * self.vent_endo.xr2 * ((self.vent_endo.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    5 => 3.2 * self.vent_m.xr1 * self.vent_m.xr2 * ((self.vent_m.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    6 => 3.2 * self.vent_epi.xr1 * self.vent_epi.xr2 * ((self.vent_epi.v + 85.0) / 85.0).clamp(0.0, 2.0),
                    _ => 0.2,
                }
            }
            7 => {
                // Corrente Funny I_f (pA/pF)
                match self.ion_cell {
                    0 => -self.sa_node.a[65].abs() * 0.15,
                    2 => -3.5 * self.av_node.y_gate * ((self.av_node.v + 20.0) / 70.0).clamp(0.0, 1.5),
                    3 => -4.0 * self.purkinje.y * ((self.purkinje.v + 20.0) / 70.0).clamp(0.0, 1.5),
                    _ => 0.0, // Células ventriculares e atriais não possuem I_f significativo
                }
            }
            _ => 0.0,
        }
    }

    pub fn set_ecg_lead(&mut self, lead: usize) {
        self.ecg_lead = lead % 12;
    }

    pub fn get_ecg_lead(&self) -> usize {
        self.ecg_lead
    }

    pub fn compute_ecg(&self) -> f64 {
        self.compute_ecg_lead(self.ecg_lead)
    }

    pub fn compute_ecg_lead(&self, lead_idx: usize) -> f64 {
        let t_ms = self.time;
        let b_na = self.pharm.block_na.max(0.15);
        let b_k = self.pharm.block_k.max(0.2);
        let k_ratio = (self.pharm.ko / 5.4).clamp(0.4, 2.5);
        let isch = self.pharm.isch.clamp(0.0, 1.0);

        // 1. Onda P (Ativação Atrial, envelope suave senoidal de ~80ms)
        let mut p_wave = 0.0;
        if self.t_last_atr_beat > 0.0 {
            let dt_atr = t_ms - self.t_last_atr_beat;
            let p_width = 80.0;
            if dt_atr >= 0.0 && dt_atr < p_width {
                let phase = dt_atr / p_width;
                // Na hipercalemia grave, a onda P se achata e desaparece
                let p_amp = 0.12 * (2.2 - k_ratio).clamp(0.0, 1.0);
                p_wave = (phase * std::f64::consts::PI).sin() * p_amp;
            }
        }

        // 2. Complexo QRS (Ativação Ventricular)
        let mut q_wave = 0.0;
        let mut r_wave = 0.0;
        let mut s_wave = 0.0;
        let mut in_st = false;
        // Bloqueadores de sódio alargam o complexo QRS
        let qrs_width = 85.0 / b_na;

        if self.t_last_vent_beat > 0.0 {
            let dt_vent = t_ms - self.t_last_vent_beat;
            if dt_vent >= 0.0 && dt_vent < qrs_width {
                let phase = dt_vent / qrs_width;
                if phase < 0.15 {
                    // Onda Q (deflexão septal negativa)
                    let p_q = phase / 0.15;
                    q_wave = -(p_q * std::f64::consts::PI).sin() * 0.07;
                } else if phase < 0.60 {
                    // Onda R (ativação transmural endocárdio -> epicárdio)
                    let p_r = (phase - 0.15) / 0.45;
                    r_wave = (p_r * std::f64::consts::PI).sin() * 0.75;
                } else {
                    // Onda S (despolarização basal tardia)
                    let p_s = (phase - 0.60) / 0.40;
                    s_wave = -(p_s * std::f64::consts::PI).sin() * 0.17;
                }
            } else if dt_vent >= qrs_width && self.in_ap {
                in_st = true;
            }
        }

        // Segmento ST: Linha de base isoelétrica com elevação sob isquemia miocárdica (STEMI)
        let st_shift = if in_st {
            isch * 0.18
        } else {
            0.0
        };

        // 3. Onda T (Repolarização Ventricular Assimétrica)
        let mut t_wave = 0.0;
        if self.t_t_wave > 0.0 {
            let dt_t = t_ms - self.t_t_wave;
            // Amiodarona (block_k baixo) alarga a onda T; hipercalemia estreita a onda T
            let t_width = 160.0 / (b_k * k_ratio.sqrt());
            // Hipercalemia torna a onda T apiculada e alta; isquemia inverte a onda T
            let t_amp = 0.22 * (k_ratio.powi(2) / b_k.sqrt()) * (1.0 - 1.8 * isch);
            if dt_t >= 0.0 && dt_t < t_width {
                let phase = dt_t / t_width;
                let shape = (phase.powf(0.85) * std::f64::consts::PI).sin();
                t_wave = shape * t_amp;
            }
        }

        // Projeção nas 12 Derivações Clínicas
        let lead = lead_idx % 12;
        let (kp, kq, kr, ks, kst, kt) = match lead {
            0 => (0.7, 0.8, 0.8, 0.6, 0.7, 0.8),         // DI
            1 => (1.0, 1.0, 1.0, 1.0, 1.0, 1.0),         // DII (Padrão de Monitor)
            2 => (0.4, 0.5, 0.6, 0.8, 0.5, 0.4),         // DIII
            3 => (-0.8, -0.6, -0.9, -0.2, -0.8, -0.9),  // aVR (Invertida)
            4 => (0.4, 0.6, 0.6, 0.4, 0.4, 0.5),         // aVL
            5 => (0.8, 0.8, 0.9, 0.8, 0.8, 0.7),         // aVF
            6 => (0.3, 0.0, 0.25, 1.3, 0.3, -0.3),       // V1 (rS profundo)
            7 => (0.5, 0.0, 0.55, 1.1, 0.5, 0.5),        // V2 (rS transicional)
            8 => (0.7, 0.3, 0.85, 0.7, 0.7, 0.8),        // V3 (isodifásico)
            9 => (0.8, 0.5, 1.30, 0.4, 0.9, 1.0),        // V4 (R alto)
            10 => (0.9, 0.6, 1.45, 0.25, 1.0, 1.0),      // V5 (R proeminente)
            11 => (0.8, 0.5, 1.20, 0.15, 0.8, 0.85),     // V6 (lateral VE)
            _ => (1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
        };

        (p_wave * kp) + (q_wave * kq) + (r_wave * kr) + (s_wave * ks) + (st_shift * kst) + (t_wave * kt)
    }

    pub fn get_hud_metrics(&self) -> HudMetrics {
        let co = (self.bpm * self.hemo.stroke_volume) / 1000.0;
        HudMetrics {
            bpm: self.bpm,
            pr: self.pr,
            qrs: self.qrs,
            qt: self.qt,
            v_rest: self.v_rest,
            pr_rr: self.pr_rr,
            edv: self.hemo.v_edv,
            esv: self.hemo.v_esv,
            sv: self.hemo.stroke_volume,
            ef: self.hemo.ejection_fraction,
            co,
            map: self.baro.pam,
            cvp: self.hemo.cvp,
            pmes: self.hemo.pmes_effective,
            pcp: self.micro.pcp,
            pi_c: self.micro.pi_c,
            edema_pulm: self.micro.v_edema_pulm,
            edema_godet: self.micro.godet_grade as f64,
            spo2: self.micro.spo2,
        }
    }

    pub fn set_baroreflex_enabled(&mut self, enabled: bool) {
        self.baro.enabled = enabled;
    }

    pub fn get_baroreflex_enabled(&self) -> bool {
        self.baro.enabled
    }

    pub fn get_hemo_map(&self) -> f64 {
        self.baro.pam
    }

    pub fn get_baro_s_baro(&self) -> f64 {
        self.baro.s_baro
    }

    pub fn get_baro_delta_symp(&self) -> f64 {
        self.baro.delta_symp
    }

    pub fn get_baro_delta_parasymp(&self) -> f64 {
        self.baro.delta_parasymp
    }

    pub fn set_peripheral_resistance_ratio(&mut self, ratio: f64) {
        self.hemo.set_peripheral_resistance_ratio(ratio);
    }

    pub fn set_valvopathy_params(
        &mut self,
        aortic_stenosis: f64,
        aortic_regurg: f64,
        mitral_stenosis: f64,
        mitral_regurg: f64,
    ) {
        self.hemo.set_valvopathies(
            aortic_stenosis,
            aortic_regurg,
            mitral_stenosis,
            mitral_regurg,
        );
    }

    pub fn get_aortic_stenosis(&self) -> f64 {
        self.hemo.aortic_stenosis
    }

    pub fn get_aortic_regurgitation(&self) -> f64 {
        self.hemo.aortic_regurgitation
    }

    pub fn get_mitral_stenosis(&self) -> f64 {
        self.hemo.mitral_stenosis
    }

    pub fn get_mitral_regurgitation(&self) -> f64 {
        self.hemo.mitral_regurgitation
    }

    pub fn set_orthostasis(&mut self, enabled: bool) {
        self.hemo.set_orthostasis(enabled);
    }

    pub fn get_orthostasis(&self) -> bool {
        self.hemo.get_orthostasis()
    }

    pub fn set_pmes(&mut self, pmes: f64) {
        self.hemo.set_pmes(pmes);
    }

    pub fn get_pmes(&self) -> f64 {
        self.hemo.get_pmes()
    }

    pub fn get_cvp(&self) -> f64 {
        self.hemo.get_cvp()
    }

    pub fn get_venous_return(&self) -> f64 {
        self.hemo.get_venous_return()
    }

    pub fn get_r_rv(&self) -> f64 {
        self.hemo.get_r_rv()
    }

    pub fn get_inotropy(&self) -> f64 {
        self.hemo.get_inotropy()
    }

    pub fn set_microvascular_params(&mut self, albumin: f64, permeability: f64) {
        self.micro.set_params(albumin, permeability);
    }

    pub fn get_albumin(&self) -> f64 {
        self.micro.albumin
    }

    pub fn get_capillary_permeability(&self) -> f64 {
        self.micro.capillary_permeability
    }

    pub fn get_pcp(&self) -> f64 {
        self.micro.pcp
    }

    pub fn get_pi_c(&self) -> f64 {
        self.micro.pi_c
    }

    pub fn get_pulmonary_edema(&self) -> f64 {
        self.micro.v_edema_pulm
    }

    pub fn get_systemic_edema_godet(&self) -> f64 {
        self.micro.godet_grade as f64
    }

    pub fn get_spo2(&self) -> f64 {
        self.micro.spo2
    }
}
