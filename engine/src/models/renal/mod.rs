//! Modelo de Fisiologia Renal, Hemodinâmica Glomerular e Balanço Hídrico de Guyton
//!
//! Implementa os mecanismos biofísicos de:
//! 1. Autoregulação da Taxa de Filtração Glomerular (TFG) e Fluxo Plasmático Renal (FPR)
//! 2. Curva de Função Renal de Natriurese e Diurese de Pressão de Arthur Guyton
//! 3. Balanço Hídrico de Longo Prazo e Modulação do Volume de Líquido Extracelular (VLEC)
//! 4. Farmacologia Tubular (Diuréticos de Alça, Tiazídicos e Bloqueio SRAA) e Estenose de Artéria Renal

#[derive(Debug, Clone)]
pub struct RenalModel {
    // Variáveis de estado hemodinâmico e renal
    pub vlec: f64,                  // Volume de Líquido Extracelular (L, basal: 15.0 L)
    pub tfg: f64,                   // Taxa de Filtração Glomerular (mL/min, basal: ~125.0)
    pub rbf: f64,                   // Fluxo Sanguíneo Renal (mL/min, basal: ~1200.0)
    pub rpf: f64,                   // Fluxo Plasmático Renal (mL/min, basal: ~660.0)
    pub ff: f64,                    // Fração de Filtração (TFG / RPF, basal: ~0.19 a 0.21)
    pub diuresis_rate_ml_min: f64,  // Taxa de diurese instantânea (mL/min, basal: ~1.0)
    pub diuresis_ml_h: f64,         // Débito urinário instantâneo (mL/h, basal: ~60.0)
    pub sodium_excretion: f64,      // Excreção de sódio (mEq/dia, basal: ~150.0)
    pub renal_map: f64,             // Pressão de perfusão arterial renal pós-estenose (mmHg)
    pub accumulated_urine_ml: f64,  // Volume total de urina acumulado (mL)

    // Parâmetros e controles clínicos
    pub renal_artery_stenosis: f64, // Estenose da artéria renal (0.0 a 1.0)
    pub afferent_tone: f64,         // Tônus / Resistência da arteríola aferente (0.5 a 3.0, basal: 1.0)
    pub raas_block: bool,           // Bloqueio do SRAA (IECA / BRA) -> vasodilatação eferente
    pub loop_diuretic: f64,         // Diurético de alça / Furosemida (0.0 a 1.0)
    pub thiazide_diuretic: f64,     // Diurético tiazídico (0.0 a 1.0)
    pub water_intake_ml_day: f64,   // Ingesta hídrica diária (mL/dia, basal: 2000.0)
    pub insensible_loss_ml_day: f64,// Perdas insensíveis por pele/respiração (mL/dia, basal: 560.0)

    // Fator de aceleração metabólica renal para visualização em tempo real (1s real = 1min biológico)
    pub time_acceleration: f64,
}

impl Default for RenalModel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenalModel {
    pub fn new() -> Self {
        Self {
            vlec: 15.0,
            tfg: 125.0,
            rbf: 1200.0,
            rpf: 660.0,
            ff: 0.189,
            diuresis_rate_ml_min: 1.0,
            diuresis_ml_h: 60.0,
            sodium_excretion: 150.0,
            renal_map: 90.0,
            accumulated_urine_ml: 0.0,

            renal_artery_stenosis: 0.0,
            afferent_tone: 1.0,
            raas_block: false,
            loop_diuretic: 0.0,
            thiazide_diuretic: 0.0,
            water_intake_ml_day: 2000.0,
            insensible_loss_ml_day: 560.0,

            time_acceleration: 180.0, // 1s de simulação equivale a 3min de dinâmica hídrica (responsiva em 10-30s)
        }
    }

    /// Passo de integração temporal da fisiologia renal
    /// `dt`: passo de tempo em segundos (ex: 0.016s por frame ou passo individual)
    /// `pam_aortica`: Pressão Arterial Média sistêmica (mmHg)
    pub fn step(&mut self, dt: f64, pam_aortica: f64) {
        // 1. Pressão de Perfusão Renal (considerando estenose de artéria renal de Goldblatt)
        // Estenose reduz linearmente a pressão a jusante para os glomérulos
        let p_renal = pam_aortica * (1.0 - 0.70 * self.renal_artery_stenosis).max(0.05);
        self.renal_map = p_renal;

        // 2. Hemodinâmica Glomerular & Autoregulação Miogênica (75 a 160 mmHg)
        // A arteríola aferente contrai com aumento de pressão e dilata com queda (feedback tubuloglomerular e miogênico)
        let autoreg_factor = if p_renal < 55.0 {
            // Choque / Hipotensão severa: colapso da pressão capilar glomerular e NFP <= 0
            ((p_renal - 35.0).max(0.0) / 20.0).powf(2.5)
        } else if p_renal < 75.0 {
            // Zona transicional pré-autoregulada (55 a 75 mmHg)
            (p_renal - 55.0) / 20.0
        } else if p_renal <= 160.0 {
            // Platô de autoregulação clássico (75 a 160 mmHg): TFG e FPR praticamente constantes
            1.0 + 0.001 * (p_renal - 90.0)
        } else {
            // Quebra da autoregulação por hipertensão extrema (> 160 mmHg)
            1.07 + 0.003 * (p_renal - 160.0)
        };

        // Efeito da resistência aferente (simpático / tônus)
        let afferent_resistance_mod = 1.0 / self.afferent_tone.clamp(0.5, 3.0);

        // Efeito do bloqueio do SRAA (IECA / BRA): vasodilatação eferente
        // Reduz a pressão intraglomerular e a fração de filtração em ~12%, aliviando hiperfiltração
        let eferent_sraa_factor = if self.raas_block { 0.88 } else { 1.0 };

        // Cálculo da TFG e RPF
        let tfg_target = (125.0 * autoreg_factor * afferent_resistance_mod * eferent_sraa_factor).max(0.0);
        let rpf_target = (660.0 * autoreg_factor * afferent_resistance_mod).max(0.0);

        // Suavização temporal da TFG e RPF (cinética de resposta miogênica em ~2-5s)
        let tau_hemo = 0.50; // segundos
        let alpha = (-dt / tau_hemo).exp();
        self.tfg = self.tfg * alpha + tfg_target * (1.0 - alpha);
        self.rpf = self.rpf * alpha + rpf_target * (1.0 - alpha);
        self.rbf = self.rpf / 0.55; // Hematócrito 45% -> RBF = RPF / (1 - Hct)
        self.ff = if self.rpf > 10.0 { (self.tfg / self.rpf).clamp(0.0, 0.40) } else { 0.0 };

        // 3. Curva de Guyton de Natriurese e Diurese de Pressão
        // Excreção urinária de água e sódio em função da pressão de perfusão renal
        // Em PAM = 90 mmHg: excreção basal normalizada = 1.0 (60 mL/h)
        // Se PAM < 55 mmHg: oligúria/anúria pré-renal
        let guyton_pressure_factor = if p_renal < 55.0 {
            (p_renal / 55.0).powf(2.5).max(0.0)
        } else {
            let delta_p = p_renal - 90.0;
            (1.0 + 0.045 * delta_p + 0.0006 * delta_p.powi(2)).max(0.05)
        };

        // 4. Farmacologia Tubular & Diuréticos
        // - Furosemida (Alça de Henle / NKCC2): até 5x a taxa de filtração excretada
        // - Hidroclorotiazida (Túbulo Distal / NCC): até 2.5x
        let diuretic_factor = 1.0 + 4.0 * self.loop_diuretic.clamp(0.0, 1.0) + 1.5 * self.thiazide_diuretic.clamp(0.0, 1.0);

        // Taxa instantânea de diurese (mL/min)
        // Basal eutrófico: 1.0 mL/min = 60.0 mL/h
        let diuresis_target = (1.0 * (self.tfg / 125.0) * guyton_pressure_factor * diuretic_factor).max(0.0);
        let tau_diur = 0.40;
        let alpha_d = (-dt / tau_diur).exp();
        self.diuresis_rate_ml_min = self.diuresis_rate_ml_min * alpha_d + diuresis_target * (1.0 - alpha_d);
        self.diuresis_ml_h = self.diuresis_rate_ml_min * 60.0;

        // Excreção de sódio proporcional à diurese e à pressão
        self.sodium_excretion = (150.0 * (self.diuresis_rate_ml_min / 1.0) * (p_renal / 90.0).sqrt()).max(0.0);

        // 5. Balanço Hídrico & Dinâmica do VLEC
        // Ingesta em mL/s e perdas em mL/s (escalonadas pelo fator de aceleração temporal)
        let intake_ml_s = (self.water_intake_ml_day / 86400.0) * self.time_acceleration;
        let insensible_loss_ml_s = (self.insensible_loss_ml_day / 86400.0) * self.time_acceleration;
        let urine_loss_ml_s = (self.diuresis_rate_ml_min / 60.0) * self.time_acceleration;

        let delta_v_ml = (intake_ml_s - insensible_loss_ml_s - urine_loss_ml_s) * dt;
        let delta_v_liters = delta_v_ml / 1000.0;

        self.vlec = (self.vlec + delta_v_liters).clamp(8.0, 25.0);
        self.accumulated_urine_ml += (urine_loss_ml_s * dt).max(0.0);
    }

    /// Retorna a modulação na Pressão Média de Enchimento Sistêmico (PMES em mmHg)
    /// derivada das variações do Volume de Líquido Extracelular (VLEC)
    /// Em VLEC = 15.0 L (basal), delta_pmes = 0.0 mmHg.
    /// Em hipervolemia (+2 L de VLEC), delta_pmes = +3.0 mmHg.
    /// Em hipovolemia (-2 L de VLEC), delta_pmes = -3.0 mmHg.
    pub fn get_pmes_modulation(&self) -> f64 {
        let delta_vlec = self.vlec - 15.0;
        // Elastância venosa sistêmica de acoplamento: ~1.5 mmHg por litro de VLEC
        delta_vlec * 1.5
    }

    /// Infunde um bolus rápido de fluido cristalóide intravenoso (ex: 500 mL)
    pub fn infuse_bolus(&mut self, volume_ml: f64) {
        self.vlec = (self.vlec + volume_ml / 1000.0).clamp(8.0, 25.0);
    }

    /// Configura os parâmetros clínicos e farmacológicos renais
    pub fn set_params(
        &mut self,
        stenosis: f64,
        afferent_tone: f64,
        raas_block: bool,
        loop_diuretic: f64,
        thiazide: f64,
        water_intake: f64,
    ) {
        self.renal_artery_stenosis = stenosis.clamp(0.0, 1.0);
        self.afferent_tone = afferent_tone.clamp(0.5, 3.0);
        self.raas_block = raas_block;
        self.loop_diuretic = loop_diuretic.clamp(0.0, 1.0);
        self.thiazide_diuretic = thiazide.clamp(0.0, 1.0);
        self.water_intake_ml_day = water_intake.clamp(500.0, 5000.0);
    }
}
