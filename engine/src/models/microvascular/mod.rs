//! Módulo de Microcirculação, Equilíbrio de Starling e Dinâmica de Edema
//! Modela a troca capilar-intersticial de fluidos baseada na equação clássica de Starling:
//! J_v = K_f * [(P_c - P_i) - sigma * (pi_c - pi_i)]
//! e na equação de Landis-Pappenheimer para a pressão coloidosmótica plasmática.

#[derive(Clone, Debug)]
pub struct MicrovascularModel {
    // Parâmetros ajustáveis
    pub albumin: f64,               // Concentração sérica de albumina (g/dL, basal ~4.0, intervalo 1.0 a 6.0)
    pub capillary_permeability: f64, // Permeabilidade capilar patológica / Lesão endotelial / Sepse [0.0, 1.0]

    // Estados de dinâmica microvascular pulmonar
    pub pcp: f64,                   // Pressão capilar pulmonar estimada (mmHg)
    pub pi_c: f64,                  // Pressão oncótica plasmática (mmHg, Landis-Pappenheimer)
    pub jv_pulm: f64,               // Taxa de filtração capilar pulmonar líquida (mL/min)
    pub j_lymph_pulm: f64,          // Drenagem linfática pulmonar (mL/min)
    pub v_edema_pulm: f64,          // Fração de acúmulo de edema pulmonar [0.0 = seco, 1.0 = alveolar grave]

    // Estados de dinâmica microvascular sistêmica
    pub pc_syst: f64,               // Pressão capilar sistêmica estimada (mmHg)
    pub jv_syst: f64,               // Taxa de filtração capilar sistêmica líquida (mL/min)
    pub j_lymph_syst: f64,          // Drenagem linfática sistêmica (mL/min)
    pub v_edema_syst: f64,          // Fração de acúmulo de edema sistêmico [0.0, 1.0]
    pub godet_grade: u8,            // Grau de edema de cacifo / Godet (0 a 4+)
    pub spo2: f64,                  // Saturação de oxigênio estimada (%)
}

impl Default for MicrovascularModel {
    fn default() -> Self {
        Self::new()
    }
}

impl MicrovascularModel {
    pub fn new() -> Self {
        let mut model = Self {
            albumin: 4.0,
            capillary_permeability: 0.0,
            pcp: 8.0,
            pi_c: 28.0,
            jv_pulm: 0.0,
            j_lymph_pulm: 0.0,
            v_edema_pulm: 0.0,
            pc_syst: 15.0,
            jv_syst: 0.0,
            j_lymph_syst: 0.0,
            v_edema_syst: 0.0,
            godet_grade: 0,
            spo2: 98.0,
        };
        model.recalculate_oncotic();
        model
    }

    /// Calcula a pressão coloidosmótica do plasma (mmHg) pela fórmula calibrada para albumina humana:
    /// pi_c = 4.0 * Alb + 0.60 * Alb^2 + 0.03 * Alb^3 (para Alb = 4.0 g/dL -> pi_c = 27.5 mmHg)
    pub fn recalculate_oncotic(&mut self) {
        let a = self.albumin.clamp(0.5, 7.0);
        self.pi_c = 4.0 * a + 0.60 * a * a + 0.03 * a * a * a;
    }

    pub fn set_params(&mut self, albumin: f64, permeability: f64) {
        self.albumin = albumin.clamp(1.0, 6.0);
        self.capillary_permeability = permeability.clamp(0.0, 1.0);
        self.recalculate_oncotic();
    }

    pub fn step(
        &mut self,
        dt_ms: f64,
        p_la: f64,       // Pressão atrial esquerda (mmHg, acoplamento de Wiggers)
        cvp: f64,        // Pressão venosa central (mmHg, acoplamento de Guyton)
    ) {
        let dt_sec = dt_ms / 1000.0;

        // 1. Pressão Oncótica Plasmática atual
        self.recalculate_oncotic();

        // 2. Coeficiente de reflexão osmótica (sigma) e condutividade hidráulica (K_f)
        // Em repouso: sigma ~ 0.92 (endotélio íntegro quase impermeável à albumina).
        // Na lesão endotelial / sepse (permeabilidade > 0): sigma cai para até 0.45 e Kf multiplica.
        let sigma_pulm = 0.92 - 0.45 * self.capillary_permeability;
        let kf_pulm = 1.0 + 3.0 * self.capillary_permeability;

        let sigma_syst = 0.88 - 0.40 * self.capillary_permeability;
        let kf_syst = 1.0 + 2.5 * self.capillary_permeability;

        // 3. MICROCIRCULAÇÃO PULMONAR
        // P_cp estimada a partir da Pressão Atrial Esquerda (LAP) e gradiente capilar
        self.pcp = (0.85 * p_la + 3.0).clamp(2.0, 45.0);

        // Pressões intersticiais pulmonares basais
        // P_i pulmonar é subatmosférica (-3.0 mmHg) mantendo o pulmão seco.
        let pi_pulm = -3.0 + 4.0 * self.v_edema_pulm;
        let pi_int_pulm = (12.0 - 5.0 * self.v_edema_pulm).max(4.0);

        let delta_p_pulm = self.pcp - pi_pulm;
        let delta_pi_pulm = (self.pi_c - pi_int_pulm).max(0.0);

        let net_starling_pulm = delta_p_pulm - sigma_pulm * delta_pi_pulm;
        self.jv_pulm = kf_pulm * net_starling_pulm;

        // Fator de segurança linfático pulmonar (~3.5 mmHg de pressão de filtração)
        let lymph_capacity_pulm = 3.5;
        self.j_lymph_pulm = self.jv_pulm.clamp(0.0, lymph_capacity_pulm);

        let net_fluid_pulm = (self.jv_pulm - lymph_capacity_pulm).max(0.0);
        let d_edema_pulm = if net_fluid_pulm > 0.0 {
            net_fluid_pulm / 35.0
        } else if self.jv_pulm < 0.0 {
            self.jv_pulm / 60.0 // Reabsorção ativa
        } else {
            -0.01 // Depuração basal lenta
        };
        self.v_edema_pulm = (self.v_edema_pulm + d_edema_pulm * dt_sec).clamp(0.0, 1.0);

        // Saturação de O2 estimada (SpO2)
        // 98.5% basal, decaindo para até 75% em edema alveolar grave
        self.spo2 = (98.5 - 20.0 * self.v_edema_pulm.powf(1.3)).clamp(72.0, 99.0);

        // 4. MICROCIRCULAÇÃO SISTÊMICA
        // P_c sistêmica estimada a partir da Pressão Venosa Central (PVC)
        self.pc_syst = (0.75 * cvp + 10.0).clamp(5.0, 35.0);

        let pi_syst = -1.0 + 5.0 * self.v_edema_syst;
        let pi_int_syst = (6.0 - 2.5 * self.v_edema_syst).max(2.0);

        let delta_p_syst = self.pc_syst - pi_syst;
        let delta_pi_syst = (self.pi_c - pi_int_syst).max(0.0);

        let net_starling_syst = delta_p_syst - sigma_syst * delta_pi_syst;
        self.jv_syst = kf_syst * net_starling_syst;

        let lymph_capacity_syst = 2.5;
        self.j_lymph_syst = self.jv_syst.clamp(0.0, lymph_capacity_syst);

        let net_fluid_syst = (self.jv_syst - lymph_capacity_syst).max(0.0);
        let d_edema_syst = if net_fluid_syst > 0.0 {
            net_fluid_syst / 30.0
        } else if self.jv_syst < 0.0 {
            self.jv_syst / 50.0
        } else {
            -0.01
        };
        self.v_edema_syst = (self.v_edema_syst + d_edema_syst * dt_sec).clamp(0.0, 1.0);

        // Grau de Godet / Cacifo clínico (0 a 4+)
        self.godet_grade = if self.v_edema_syst < 0.08 {
            0
        } else if self.v_edema_syst < 0.28 {
            1
        } else if self.v_edema_syst < 0.55 {
            2
        } else if self.v_edema_syst < 0.80 {
            3
        } else {
            4
        };
    }

    /// Retorna o fator multiplicador de rigidez toracopulmonar (1.0 = normal, até 3.5 em edema grave)
    pub fn get_crs_stiffness_factor(&self) -> f64 {
        1.0 + 2.5 * self.v_edema_pulm
    }
}
