//! Módulo de Fisiologia Respiratória, Espirometria e Acoplamento Cardiorrespiratório
//! Modela os volumes e capacidades pulmonares estáticos e dinâmicos, a dinâmica da
//! pressão intrapleural, a alça fluxo-volume e a arritmia sinusal respiratória (RSA).

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ManeuverState {
    Idle,
    InspiringMax,
    ExpiringForced,
    Completed,
}

#[derive(Clone, Debug)]
pub struct RespiratoryMetrics {
    pub vol: f64,          // Volume atual (L)
    pub flow: f64,         // Fluxo atual (L/s, positivo = expiratório na convenção espirométrica)
    pub p_pl: f64,         // Pressão intrapleural (cmH2O)
    pub vef1: f64,         // VEF1 medido (L)
    pub cvf: f64,          // CVF medido (L)
    pub tiffeneau: f64,    // Índice de Tiffeneau (VEF1/CVF %)
    pub pef: f64,          // Pico de Fluxo Expiratório (L/s)
    pub in_maneuver: bool, // Se manobra de CVF está em curso
}

#[derive(Clone, Debug)]
pub struct RespiratorySystem {
    pub time: f64,          // Tempo acumulado (ms)
    pub vol: f64,           // Volume pulmonar absoluto atual (L)
    pub flow: f64,          // Fluxo aéreo instantâneo (L/s, positivo na expiração)
    pub p_pl: f64,          // Pressão intrapleural (cmH2O)
    pub f_rsa: f64,         // Fator de Arritmia Sinusal Respiratória (-0.10 a +0.15)
    
    // Parâmetros Fisiológicos Estáticos e Mecânicos
    pub resp_rate: f64,     // Frequência respiratória (irpm, padrão 15)
    pub raw: f64,           // Resistência de vias aéreas (cmH2O / (L/s), normal ~1.5)
    pub c_rs: f64,          // Complacência toracopulmonar (L/cmH2O, normal ~0.10)
    pub vr: f64,            // Volume Residual - o ar que fica (L, ~1.2 L)
    pub vre: f64,           // Volume de Reserva Expiratório (L, ~1.1 L)
    pub vt: f64,            // Volume Corrente em repouso (L, ~0.5 L)
    pub vri: f64,           // Volume de Reserva Inspiratório (L, ~3.0 L)
    pub edema_stiffness: f64, // Fator multiplicador de rigidez por edema intersticial/alveolar (1.0 = seco)

    // Estados da Manobra de Espirometria Forçada
    pub maneuver_state: ManeuverState,
    pub t_maneuver_start: f64,
    pub t_exp_start: f64,
    pub vol_at_exp_start: f64,
    pub vef1: f64,
    pub cvf: f64,
    pub pef: f64,
    pub tiffeneau: f64,
}

impl Default for RespiratorySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RespiratorySystem {
    pub fn new() -> Self {
        let vr = 1.20;
        let vre = 1.10;
        let vt = 0.50;
        let vri = 3.00;
        let crf = vr + vre; // 2.30 L (ponto de repouso elástico)
        Self {
            time: 0.0,
            vol: crf,
            flow: 0.0,
            p_pl: -5.0,
            f_rsa: 0.0,
            resp_rate: 15.0,
            raw: 1.5,
            c_rs: 0.10,
            vr,
            vre,
            vt,
            vri,
            edema_stiffness: 1.0,
            maneuver_state: ManeuverState::Idle,
            t_maneuver_start: -1.0,
            t_exp_start: -1.0,
            vol_at_exp_start: 0.0,
            vef1: 3.80,
            cvf: 4.60,
            pef: 8.50,
            tiffeneau: 82.6,
        }
    }

    pub fn set_edema_stiffness(&mut self, factor: f64) {
        self.edema_stiffness = factor.clamp(1.0, 5.0);
    }

    pub fn get_effective_crs(&self) -> f64 {
        self.c_rs / self.edema_stiffness
    }

    /// Capacidade Residual Funcional (CRF = VR + VRE)
    pub fn crf(&self) -> f64 {
        let eff_vre = self.vre / (1.0 + (self.edema_stiffness - 1.0) * 0.5);
        self.vr + eff_vre
    }

    /// Capacidade Pulmonar Total (CPT = CRF + VT + VRI)
    pub fn cpt(&self) -> f64 {
        let eff_vt = self.vt / (1.0 + (self.edema_stiffness - 1.0) * 0.4);
        let eff_vri = self.vri / self.edema_stiffness;
        self.crf() + eff_vt + eff_vri
    }

    /// Capacidade Vital Forçada Fisiológica Teórica (CVF = VRI + VT + VRE)
    pub fn cvf_expected(&self) -> f64 {
        self.vri + self.vt + self.vre
    }

    /// Dispara manobra de espirometria forçada (Inspiração máxima até CPT seguida de expiração forçada até VR)
    pub fn trigger_spirometry(&mut self) {
        self.maneuver_state = ManeuverState::InspiringMax;
        self.t_maneuver_start = self.time;
        self.pef = 0.0;
        self.vef1 = 0.0;
        self.cvf = 0.0;
        self.tiffeneau = 0.0;
    }

    /// Avança a dinâmica respiratória em dt milissegundos
    pub fn step(&mut self, dt_ms: f64) {
        let dt_sec = dt_ms / 1000.0;
        self.time += dt_ms;

        let crf = self.crf();
        let cpt = self.cpt();

        match self.maneuver_state {
            ManeuverState::Idle => {
                // Respiração Eupneica Basal em Repouso
                let period_sec = 60.0 / self.resp_rate.max(5.0);
                let t_sec = self.time / 1000.0;
                let phase = (t_sec % period_sec) / period_sec; // 0.0 a 1.0
                let eff_vt = self.vt / (1.0 + (self.edema_stiffness - 1.0) * 0.4);

                // Proporção fisiológica I:E de 1:2 (inspiração dura 1/3 do ciclo, expiração dura 2/3)
                let insp_fraction = 0.35;
                if phase < insp_fraction {
                    // Fase Inspiratória Ativa
                    let p_i = phase / insp_fraction;
                    let target_vol = crf + eff_vt * (p_i * std::f64::consts::PI / 2.0).sin();
                    let d_vol = target_vol - self.vol;
                    self.flow = -d_vol / dt_sec.max(1e-4); // Fluxo negativo na inspiração
                    self.vol = target_vol;

                    // Pressão intrapleural mais negativa na inspiração: -5.0 -> -8.0 cmH2O
                    self.p_pl = -5.0 - 3.0 * (p_i * std::f64::consts::PI / 2.0).sin();

                    // RSA: Aceleração da FC (inibição vagal transitória)
                    self.f_rsa = 0.12 * (p_i * std::f64::consts::PI).sin();
                } else {
                    // Fase Expiratória Passiva (Recuo Elástico)
                    let p_e = (phase - insp_fraction) / (1.0 - insp_fraction);
                    let target_vol = (crf + eff_vt) - eff_vt * (p_e * std::f64::consts::PI / 2.0).sin();
                    let d_vol = self.vol - target_vol;
                    self.flow = d_vol / dt_sec.max(1e-4); // Fluxo positivo na expiração
                    self.vol = target_vol;

                    // Pressão intrapleural retorna ao repouso: -8.0 -> -5.0 cmH2O
                    self.p_pl = -8.0 + 3.0 * (p_e * std::f64::consts::PI / 2.0).sin();

                    // RSA: Desaceleração da FC (eferência vagal)
                    self.f_rsa = -0.08 * (p_e * std::f64::consts::PI).sin();
                }
            }

            ManeuverState::InspiringMax => {
                // 1. Inspiração máxima forçada rápida até a CPT (~1.2 segundos)
                let elapsed_ms = self.time - self.t_maneuver_start;
                let insp_duration_ms = 1200.0;
                let progress = (elapsed_ms / insp_duration_ms).clamp(0.0, 1.0);
                
                let target_vol = crf + (cpt - crf) * (progress * std::f64::consts::PI / 2.0).sin();
                let d_vol = target_vol - self.vol;
                self.flow = -d_vol / dt_sec.max(1e-4);
                self.vol = target_vol;
                self.p_pl = -5.0 - 20.0 * progress; // Esforço inspiratório profundo (-25 cmH2O)
                self.f_rsa = 0.15;

                if elapsed_ms >= insp_duration_ms {
                    // CPT alcançada! Inicia expiração máxima explosiva
                    self.maneuver_state = ManeuverState::ExpiringForced;
                    self.t_exp_start = self.time;
                    self.vol_at_exp_start = self.vol;
                    self.pef = 0.0;
                }
            }

            ManeuverState::ExpiringForced => {
                // 2. Expiração máxima explosiva forçada da CPT até o Volume Residual (VR)
                let elapsed_exp_sec = (self.time - self.t_exp_start) / 1000.0;
                
                // Constante de tempo mecânica toracopulmonar tau = Raw * Crs
                // Calibrada para ~0.58s em condições normais (Tiffeneau ~82%),
                // aumentando linearmente sob broncoespasmo (asma/DPOC)
                let tau = (0.58 * (self.raw / 1.5) * (self.get_effective_crs() / 0.10)).max(0.15);
                
                // Modelo de esvaziamento alveolar sob compressão dinâmica das vias aéreas:
                // Volume acima do VR decai exponencialmente modulado por tau
                let vol_expiravel_total = self.vol_at_exp_start - self.vr;
                let vol_remanescente = vol_expiravel_total * (-elapsed_exp_sec / tau).exp();
                let new_vol = self.vr + vol_remanescente;

                // Fluxo instantâneo = -dV/dt = vol_expiravel_total / tau * exp(-t/tau)
                let instant_flow = (vol_remanescente / tau).max(0.0);
                self.flow = instant_flow; // Positivo (expiratório)
                self.vol = new_vol;

                if self.flow > self.pef {
                    self.pef = self.flow;
                }

                // Pressão intrapleural altamente positiva pelo esforço muscular expiratório expirando com força (+30 cmH2O)
                self.p_pl = 30.0 * (-elapsed_exp_sec / 0.8).exp() - 5.0;
                self.f_rsa = -0.10;

                // Mensuração do VEF1 no marco exato de 1.0 segundo
                if elapsed_exp_sec >= 1.0 && self.vef1 <= 0.01 {
                    self.vef1 = self.vol_at_exp_start - self.vol;
                }

                // Manobra dura ~4 a 6 segundos até esvaziamento próximo do VR
                if elapsed_exp_sec >= 5.0 || vol_remanescente < 0.05 {
                    self.cvf = self.vol_at_exp_start - self.vol;
                    if self.vef1 <= 0.01 {
                        self.vef1 = self.cvf * 0.80; // fallback se interrompido
                    }
                    if self.cvf > 0.1 {
                        self.tiffeneau = (self.vef1 / self.cvf) * 100.0;
                    }
                    self.maneuver_state = ManeuverState::Completed;
                }
            }

            ManeuverState::Completed => {
                // Retorno gradual e suave ao repouso eupneico (CRF)
                let d_crf = self.vol - crf;
                self.vol -= d_crf * dt_sec * 1.5;
                self.flow = 0.0;
                self.p_pl += (-5.0 - self.p_pl) * dt_sec * 2.0;
                self.f_rsa = 0.0;

                if (self.vol - crf).abs() < 0.05 {
                    self.vol = crf;
                    self.p_pl = -5.0;
                    self.maneuver_state = ManeuverState::Idle;
                }
            }
        }
    }

    /// Retorna as métricas completas para a interface
    pub fn get_metrics(&self) -> RespiratoryMetrics {
        RespiratoryMetrics {
            vol: self.vol,
            flow: self.flow,
            p_pl: self.p_pl,
            vef1: self.vef1,
            cvf: self.cvf,
            tiffeneau: self.tiffeneau,
            pef: self.pef,
            in_maneuver: self.maneuver_state != ManeuverState::Idle,
        }
    }
}
