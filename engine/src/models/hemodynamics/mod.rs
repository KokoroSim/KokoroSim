// Modelagem Hemodinâmica e Acoplamento Eletromecânico Ventricular
// Baseado na teoria de elastância ventricular variável no tempo (Suga & Sagawa, 1974)
// e no modelo de circulação arterial Windkessel de 3 elementos (Westerhof et al., 2009).

pub struct HemodynamicsModel {
    // Estados hemodinâmicos
    pub v_lv: f64,       // Volume Ventricular Esquerdo (mL), repouso ~120 mL (EDV)
    pub p_lv: f64,       // Pressão Ventricular Esquerda (mmHg), diastólica ~6-8, sistólica ~120
    pub p_ao: f64,       // Pressão Aórtica (mmHg), diastólica ~80, sistólica ~120
    pub p_la: f64,       // Pressão Atrial Esquerda (mmHg), ~6-12 mmHg
    pub f_active: f64,   // Tensão / Força ativa normalizada [0.0, 1.0]
    
    // Estados valvares
    pub mitral_open: bool,
    pub aortic_open: bool,
    
    // Disparadores de eventos acústicos (limpos após cada leitura ou mantidos transitórios)
    pub event_b1: bool,       // Fechamento da valva Mitral (Início da contração isovolumétrica)
    pub event_b2: bool,       // Fechamento da valva Aórtica (Início do relaxamento isovolumétrico)
    pub event_r_peak: bool,   // Pico R / Despolarização Ventricular rápida (Bip da UTI)
    
    // Parâmetros fisiológicos
    v0: f64,           // Volume não-estressado (mL) ~15 mL
    e_min: f64,        // Elastância diastólica passiva (mmHg/mL) ~0.07
    e_max_base: f64,   // Elastância sistólica máxima basal (mmHg/mL) ~2.4
    c_ao: f64,         // Complacência aórtica (mL/mmHg) ~1.0
    r_tpr_base: f64,   // Resistência vascular periférica total (mmHg*ms/mL) ~1.0 s -> 1000 ms
    r_valve_in: f64,   // Resistência da valva mitral aberta (mmHg*ms/mL) ~10.0
    r_valve_out: f64,  // Resistência da valva aórtica aberta (mmHg*ms/mL) ~8.0
    
    // Memória interna para detecção de flanco
    prev_v_endo: f64,
}

impl HemodynamicsModel {
    pub fn new() -> Self {
        Self {
            v_lv: 120.0,        // End-Diastolic Volume inicial
            p_lv: 8.0,          // Pressão diastólica ventricular
            p_ao: 80.0,         // Pressão diastólica aórtica inicial
            p_la: 8.0,          // Pressão atrial inicial
            f_active: 0.0,
            mitral_open: true,
            aortic_open: false,
            event_b1: false,
            event_b2: false,
            event_r_peak: false,
            v0: 15.0,
            e_min: 0.07,
            e_max_base: 2.3,
            c_ao: 1.0,
            r_tpr_base: 1300.0, // ms * mmHg / mL (~1.3 mmHg*s/mL)
            r_valve_in: 15.0,
            r_valve_out: 10.0,
            prev_v_endo: -85.0,
        }
    }

    pub fn step(
        &mut self,
        dt: f64,          // em milissegundos (ex: 0.001 ms)
        ca_i: f64,        // [Ca²⁺]ᵢ em mM do modelo de ten Tusscher
        v_atrium: f64,    // Potencial de membrana do átrio (mV)
        v_endo: f64,      // Potencial de membrana do endocárdio (mV)
        symp: f64,        // Modulação simpática [0.0, 1.0]
        parasymp: f64,    // Modulação parassimpática [0.0, 1.0]
    ) {
        // Reset eventos acústicos instantâneos por passo
        self.event_b1 = false;
        self.event_b2 = false;
        self.event_r_peak = false;

        // 1. Detecção do Pico R / Despolarização Ventricular (Bip UTI)
        // Dispara quando cruza -10 mV com dV/dt fortemente positivo
        if self.prev_v_endo < -10.0 && v_endo >= -10.0 {
            self.event_r_peak = true;
        }
        self.prev_v_endo = v_endo;

        // 2. Acoplamento Eletromecânico (Transiente de Cálcio -> Força Ativa)
        // Cinética de Hill com cooperatividade de troponina
        let kd: f64 = 0.00055; // 0.55 µM
        let n: f64 = 3.0;
        let ca_pow = ca_i.powf(n);
        let f_inf = ca_pow / (kd.powf(n) + ca_pow);
        
        // Cinética assimétrica: fixação rápida de pontes (~25ms), relaxamento dependente de SERCA (~45ms)
        let tau_f = if f_inf > self.f_active { 25.0 } else { 45.0 };
        let df = (f_inf - self.f_active) / tau_f;
        self.f_active = (self.f_active + df * dt).clamp(0.0, 1.0);

        // 3. Modulação Inotrópica Autonômica da Elastância Sistólica
        let inotropy = 1.0 + (symp * 0.45) - (parasymp * 0.20);
        let e_max = self.e_max_base * inotropy;
        let elastance = self.e_min + (e_max - self.e_min) * self.f_active;

        // 4. Pressão Atrial Esquerda (LAP) - reflete a contração atrial (onda 'a')
        let p_la_base = 7.0 + (symp * 2.0);
        let atrial_kick = if v_atrium > -20.0 { 6.0 } else { 0.0 };
        self.p_la = p_la_base + atrial_kick;

        // 5. Pressão Isovolumétrica Ventricular
        let p_iso = elastance * (self.v_lv - self.v0).max(0.0);

        // 6. Dinâmica Valvar e Fluxos (Mitral e Aórtica)
        let mut q_in = 0.0;
        let mut q_out = 0.0;

        // --- Valva Mitral ---
        if p_iso < self.p_la && !self.aortic_open {
            // Mitral abre (enchimento diastólico)
            self.mitral_open = true;
            q_in = (self.p_la - p_iso) / self.r_valve_in;
        } else {
            // Se estava aberta e a pressão ventricular superou a atrial -> FECHOU (B1!)
            if self.mitral_open {
                self.mitral_open = false;
                self.event_b1 = true;
            }
        }

        // --- Valva Aórtica ---
        if p_iso > self.p_ao && !self.mitral_open {
            // Aórtica abre (ejeção sistólica)
            self.aortic_open = true;
            q_out = (p_iso - self.p_ao) / self.r_valve_out;
        } else {
            // Se estava aberta e a pressão ventricular caiu abaixo da aórtica -> FECHOU (B2!)
            if self.aortic_open {
                self.aortic_open = false;
                self.event_b2 = true;
            }
        }

        // 7. Atualização do Volume Ventricular Esquerdo
        let dv = q_in - q_out;
        self.v_lv = (self.v_lv + dv * dt).clamp(40.0, 150.0);

        // Pressão ventricular efetiva
        self.p_lv = p_iso;

        // 8. Modelo Arterial Windkessel de 3 Elementos para a Pressão Aórtica
        // dP_ao/dt = Q_out / C_ao - (P_ao - P_venous) / (R_tpr * C_ao)
        let p_venous = 4.0;
        let r_tpr = self.r_tpr_base * (1.0 + (symp * 0.2) - (parasymp * 0.1));
        let dp_ao = (q_out / self.c_ao) - ((self.p_ao - p_venous) / (r_tpr * self.c_ao));
        self.p_ao = (self.p_ao + dp_ao * dt).clamp(30.0, 220.0);
    }
}
