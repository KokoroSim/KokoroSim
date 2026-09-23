// Modelagem Hemodinâmica e Acoplamento Eletromecânico Ventricular
// Baseado na teoria de elastância ventricular variável no tempo (Suga & Sagawa, 1974)
// e no modelo de circulação arterial Windkessel de 3 elementos (Westerhof et al., 2009).

pub struct HemodynamicsModel {
    // Estados hemodinâmicos
    pub v_lv: f64,       // Volume Ventricular Esquerdo (mL), repouso ~120 mL (EDV)
    pub p_lv: f64,       // Pressão Ventricular Esquerda (mmHg), diastólica ~6-8, sistólica ~120
    pub p_ao: f64,       // Pressão Aórtica (mmHg), diastólica ~80, sistólica ~120
    pub p_la: f64,       // Pressão Atrial Esquerda (mmHg), ~6-12 mmHg com ondas a, c, v
    pub f_active: f64,   // Tensão / Força ativa normalizada [0.0, 1.0]
    
    // Métricas volumétricas de ciclo fechado
    pub v_edv: f64,             // Volume Diastólico Final (VDF/EDV em mL, medido em B1)
    pub v_esv: f64,             // Volume Sistólico Final (VSF/ESV em mL, medido em B2)
    pub stroke_volume: f64,     // Volume Sistólico (VS = VDF - VSF em mL)
    pub ejection_fraction: f64, // Fração de Ejeção (FE = VS / VDF * 100 em %)
    v_la_filling: f64,          // Componente de enchimento venoso atrial para a onda v
    
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
    r_valve_in: f64,   // Resistência da valva mitral aberta (mmHg*ms/mL) ~7.5
    r_valve_out: f64,  // Resistência da valva aórtica aberta (mmHg*ms/mL) ~3.8
    
    // Memória interna para detecção de flanco
    prev_v_endo: f64,

    // Parâmetros de valvopatias [0.0 = normal, 1.0 = patologia severa]
    pub aortic_stenosis: f64,
    pub aortic_regurgitation: f64,
    pub mitral_stenosis: f64,
    pub mitral_regurgitation: f64,

    // Modulação exógena de resistência vascular periférica (ex: vasodilatadores, choque distributivo)
    pub r_tpr_mod: f64,

    // Acoplamento Sistêmico de Guyton & Retorno Venoso
    pub pmes_base: f64,         // Pressão Média de Enchimento Sistêmico basal (mmHg, repouso ~7.5)
    pub pmes_effective: f64,    // PMES instantânea (modulada por ortostase e venoconstrição simpática)
    pub pmes_volume_mod: f64,   // Modulação por volume extracelular renal (mmHg)
    pub cvp: f64,               // Pressão Venosa Central (PVC / Átrio Direito, mmHg ~2.5-4.0)
    pub venous_return: f64,     // Retorno Venoso (L/min)
    pub r_rv_base: f64,         // Resistência ao retorno venoso (mmHg*min/L, ~0.85)
    pub r_rv_effective: f64,    // Resistência efetiva ao retorno venoso
    pub inotropy: f64,          // Modulação inotrópica da função ventricular
    pub orthostasis: bool,      // Posição ortostática (em pé / pooling venoso gravitacional)
}

impl HemodynamicsModel {
    pub fn new() -> Self {
        Self {
            v_lv: 120.0,        // End-Diastolic Volume inicial
            p_lv: 8.0,          // Pressão diastólica ventricular
            p_ao: 80.0,         // Pressão diastólica aórtica inicial
            p_la: 8.0,          // Pressão atrial inicial
            f_active: 0.0,
            v_edv: 120.0,
            v_esv: 50.0,
            stroke_volume: 70.0,
            ejection_fraction: 58.33,
            v_la_filling: 0.0,
            mitral_open: true,
            aortic_open: false,
            event_b1: false,
            event_b2: false,
            event_r_peak: false,
            v0: 15.0,
            e_min: 0.065,
            e_max_base: 2.85,
            c_ao: 1.15,
            r_tpr_base: 1100.0, // ms * mmHg / mL (~1.10 mmHg*s/mL)
            r_valve_in: 7.5,
            r_valve_out: 3.8,
            prev_v_endo: -85.0,
            aortic_stenosis: 0.0,
            aortic_regurgitation: 0.0,
            mitral_stenosis: 0.0,
            mitral_regurgitation: 0.0,
            r_tpr_mod: 1.0,
            pmes_base: 7.5,
            pmes_effective: 7.5,
            pmes_volume_mod: 0.0,
            cvp: 3.2,
            venous_return: 5.0,
            r_rv_base: 0.85,
            r_rv_effective: 0.85,
            inotropy: 1.0,
            orthostasis: false,
        }
    }

    pub fn set_pmes_volume_mod(&mut self, delta: f64) {
        self.pmes_volume_mod = delta;
    }

    pub fn get_pmes_volume_mod(&self) -> f64 {
        self.pmes_volume_mod
    }

    pub fn set_orthostasis(&mut self, enabled: bool) {
        self.orthostasis = enabled;
    }

    pub fn get_orthostasis(&self) -> bool {
        self.orthostasis
    }

    pub fn set_pmes(&mut self, pmes: f64) {
        self.pmes_base = pmes.clamp(1.0, 16.0);
    }

    pub fn get_pmes(&self) -> f64 {
        self.pmes_effective
    }

    pub fn get_cvp(&self) -> f64 {
        self.cvp
    }

    pub fn get_venous_return(&self) -> f64 {
        self.venous_return
    }

    pub fn get_r_rv(&self) -> f64 {
        self.r_rv_effective
    }

    pub fn get_inotropy(&self) -> f64 {
        self.inotropy
    }

    pub fn set_peripheral_resistance_ratio(&mut self, ratio: f64) {
        self.r_tpr_mod = ratio.clamp(0.2, 5.0);
    }

    pub fn set_valvopathies(
        &mut self,
        aortic_stenosis: f64,
        aortic_regurg: f64,
        mitral_stenosis: f64,
        mitral_regurg: f64,
    ) {
        self.aortic_stenosis = aortic_stenosis.clamp(0.0, 1.0);
        self.aortic_regurgitation = aortic_regurg.clamp(0.0, 1.0);
        self.mitral_stenosis = mitral_stenosis.clamp(0.0, 1.0);
        self.mitral_regurgitation = mitral_regurg.clamp(0.0, 1.0);
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
        // Cinética cooperativa de Hill com sustentação durante o platô da sístole ventricular
        let kd: f64 = 0.00055; // 0.55 µM
        let n: f64 = 3.0;
        let ca_pow = ca_i.powf(n);
        let f_inf = ca_pow / (kd.powf(n) + ca_pow);
        
        // Pontes cruzadas de actina-miosina sustentam a força durante o platô sistólico (Fase 2)
        let f_target = if v_endo > -35.0 || ca_i > 0.0004 {
            f_inf.max(0.85)
        } else {
            f_inf
        };
        let tau_relax = 85.0 / (1.0 + symp * 0.6);
        let tau_f = if f_target > self.f_active { 22.0 } else { tau_relax };
        let df = (f_target - self.f_active) / tau_f;
        self.f_active = (self.f_active + df * dt).clamp(0.0, 1.0);

        // 3. Modulação Inotrópica Autonômica da Elastância Sistólica
        // Efeito Anrep: autorregulação miocárdica em resposta ao aumento abrupto de pós-carga (estenose aórtica)
        let anrep = 1.0 + self.aortic_stenosis * 0.40;
        let inotropy = (1.0 + (symp * 1.15) - (parasymp * 0.30)) * anrep;
        self.inotropy = inotropy;
        let e_max = self.e_max_base * inotropy;
        
        // Elastância passiva não-linear (EDPVR com rigidez elástica progressiva acima de 115 mL)
        let v_excess = (self.v_lv - 112.0).max(0.0);
        let e_diast = self.e_min + 0.0028 * v_excess;
        let elastance = e_diast + (e_max - e_diast) * self.f_active;

        // 4. Acoplamento Sistêmico de Guyton: Retorno Venoso e Pressão Média de Enchimento Sistêmico (PMES)
        // PMES reflete a volemia total e a capacitância vascular periférica.
        // Na ortostase (em pé), o acúmulo hidrostático venoso em membros inferiores (pooling ~500 mL)
        // reduz a PMES efetiva. A venoconstrição simpática reflexa alfa-1 recruta sangue dos leitos
        // capacitivos (esplâncnico/cutâneo), restaurando parcialmente a PMES.
        let pooling_factor = if self.orthostasis { 0.32 } else { 0.0 };
        let venoconstriction = symp * 0.28 - parasymp * 0.08;
        let pmes_total = (self.pmes_base + self.pmes_volume_mod).max(1.0);
        self.pmes_effective = (pmes_total * (1.0 - pooling_factor + venoconstriction)).clamp(1.0, 25.0);

        // Resistência ao Retorno Venoso (R_rv)
        let r_rv = (self.r_rv_base * (1.0 + (if self.orthostasis { 0.18 } else { 0.0 })) / (1.0 + symp * 0.15)).max(0.2);
        self.r_rv_effective = r_rv;

        // Retorno Venoso instantâneo (Guyton): RV = (PMES - PVC) / R_rv
        self.venous_return = ((self.pmes_effective - self.cvp) / r_rv).max(0.0);

        // Dinâmica de Conservação de Massa no Leito Venoso Central:
        // d(PVC)/dt = (RV - DC) / C_venous
        // Em repouso eutrófico, a PVC se estabiliza exatamente no ponto onde RV == DC.
        let est_co = (self.stroke_volume * 75.0) / 1000.0; // Débito cardíaco atual em L/min
        let d_cvp = (self.venous_return - est_co) / 2500.0; // Constante de tempo de complacência venosa (2.5s)
        self.cvp = (self.cvp + d_cvp * dt).clamp(0.5, 12.0);

        // Pressão Atrial Esquerda (LAP) - Curva fisiológica do Diagrama de Wiggers
        // Linha de base diastólica modulada pelo retorno venoso (Guyton), simpático e valvopatias
        let ms_backlog = self.mitral_stenosis * 15.0;
        let volume_drive = (self.pmes_effective / 7.5).clamp(0.1, 2.5);
        let p_la_base = (1.5 + 5.5 * volume_drive + (symp * 2.5) + ms_backlog).clamp(1.0, 35.0);
        
        // Onda 'a' (contração atrial ativa / sístole atrial pós-onda P, hipertrofiada na estenose mitral)
        let a_gain = 1.0 + self.mitral_stenosis * 0.6;
        let atrial_kick = if v_atrium > -20.0 { 6.5 * a_gain } else { 0.0 };
        
        // Onda 'c' (protrusão valvar no início da sístole isovolumétrica ventricular)
        let c_wave = if !self.mitral_open && !self.aortic_open && self.f_active > 0.15 && self.f_active < 0.70 {
            3.0 * ((self.f_active - 0.15) / 0.55)
        } else {
            0.0
        };
        
        // 5. Pressão Isovolumétrica Ventricular preliminar para fluxos
        let p_iso = elastance * (self.v_lv - self.v0).max(0.0);

        // Regurgitação sistólica mitral (Insuficiência Mitral)
        let q_regurg_mi = if !self.mitral_open && self.mitral_regurgitation > 0.0 && p_iso > self.p_la {
            (p_iso - self.p_la) * (self.mitral_regurgitation * 0.025)
        } else {
            0.0
        };

        // Onda 'v' (enchimento venoso pulmonar passivo com a valva mitral fechada + jato de refluxo na insuficiência mitral)
        if !self.mitral_open {
            let mr_jet = q_regurg_mi * 15.0;
            self.v_la_filling = (self.v_la_filling + (0.015 + mr_jet) * dt).min(35.0);
        } else {
            // Descenso 'y' rápido quando a valva mitral abre e esvazia no ventrículo
            self.v_la_filling = (self.v_la_filling - 0.06 * dt).max(0.0);
        }
        
        self.p_la = p_la_base + atrial_kick + c_wave + self.v_la_filling;

        // 6. Dinâmica Valvar e Fluxos com histerese e valvopatias
        let mut q_in = 0.0;
        let mut q_out = 0.0;

        // --- Valva Mitral ---
        let r_in_stenotic = self.r_valve_in * (1.0 + 5.0 * self.mitral_stenosis);
        let r_in_eff = r_in_stenotic / (1.0 + symp * 0.6);

        if !self.mitral_open {
            if !self.aortic_open && self.f_active < 0.04 && p_iso < self.p_la {
                self.mitral_open = true;
                q_in = (self.p_la - p_iso).max(0.0) / r_in_eff;
            }
        } else {
            if self.f_active >= 0.08 && p_iso >= self.p_la {
                // Fechamento sistólico da valva mitral pelo início da contração ventricular isovolumétrica -> B1!
                self.mitral_open = false;
                self.event_b1 = true;
                // Registro do Volume Diastólico Final (EDV) no instante do B1
                self.v_edv = self.v_lv;
            } else {
                // Durante a diástole, enchimento ventricular passivo e ativo
                q_in = (self.p_la - p_iso).max(0.0) / r_in_eff;
            }
        }

        // --- Valva Aórtica ---
        let r_out_stenotic = self.r_valve_out * (1.0 + 7.0 * self.aortic_stenosis);

        if p_iso > self.p_ao && !self.mitral_open && self.f_active > 0.15 {
            self.aortic_open = true;
            q_out = (p_iso - self.p_ao) / r_out_stenotic;
        } else if self.aortic_open && p_iso <= self.p_ao {
            // Aórtica fecha no início do relaxamento isovolumétrico -> B2!
            self.aortic_open = false;
            self.event_b2 = true;
            // Registro do Volume Sistólico Final (ESV) no instante do B2
            self.v_esv = self.v_lv;
            self.stroke_volume = (self.v_edv - self.v_esv).max(0.0);
            if self.v_edv > 10.0 {
                self.ejection_fraction = (self.stroke_volume / self.v_edv) * 100.0;
            }
        }

        // Regurgitação diastólica aórtica (Insuficiência Aórtica)
        let q_regurg_ao = if !self.aortic_open && self.aortic_regurgitation > 0.0 && self.p_ao > p_iso {
            (self.p_ao - p_iso) * (self.aortic_regurgitation * 0.035)
        } else {
            0.0
        };

        // 7. Atualização do Volume Ventricular Esquerdo
        let dv = (q_in + q_regurg_ao) - (q_out + q_regurg_mi);
        self.v_lv = (self.v_lv + dv * dt).clamp(25.0, 220.0);

        // Pressão ventricular efetiva
        self.p_lv = p_iso;

        // 8. Modelo Arterial Windkessel de 3 Elementos para a Pressão Aórtica
        // dP_ao/dt = (Q_out - Q_regurg_ao) / C_ao - (P_ao - P_venous) / (R_tpr * C_ao)
        let p_venous = 4.0;
        let r_tpr = (self.r_tpr_base * self.r_tpr_mod * (1.0 + (symp * 0.60) - (parasymp * 0.15))).clamp(self.r_tpr_base * 0.25, self.r_tpr_base * 3.0);
        let dp_ao = ((q_out - q_regurg_ao) / self.c_ao) - ((self.p_ao - p_venous) / (r_tpr * self.c_ao));
        self.p_ao = (self.p_ao + dp_ao * dt).clamp(20.0, 260.0);
    }
}
