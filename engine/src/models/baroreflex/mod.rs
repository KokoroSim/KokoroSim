//! Módulo de Regulação Autonômica por Barorreflexo Arterial em Malha Fechada
//!
//! Modela os barorreceptores do seio carotídeo (nervo glossofaríngeo IX) e do
//! arco aórtico (nervo vago X), a integração bulbar (NTS, RVLM e Núcleo Ambíguo)
//! e o controle de feedback negativo sobre a frequência cardíaca (nó SA),
//! dromotropismo (nó AV), inotropismo ventricular (Suga-Sagawa) e resistência
//! vascular periférica total (TPR Windkessel).

#[derive(Clone, Copy, Debug)]
pub struct BaroreflexModel {
    pub enabled: bool,
    pub p_setpoint: f64,     // Setpoint alvo de PAM (mmHg, padrão 92.0)
    pub pam: f64,            // Pressão Arterial Média filtrada (mmHg)
    pub s_baro: f64,         // Disparo barorreceptor aferente normalizado [-1.0, 1.0]
    pub delta_symp: f64,     // Modulação eferente simpática [-0.5, 0.8]
    pub delta_parasymp: f64, // Modulação eferente vagal [-0.2, 0.35]
    
    // Constantes dinâmicas biofísicas
    pub tau_map: f64,        // Constante de tempo do filtro de PAM (ms, ~2000.0)
    pub tau_symp: f64,       // Constante do ramo simpático (ms, ~3500.0)
    pub tau_vagus: f64,      // Constante do ramo vagal (ms, ~800.0)
    pub k_sens: f64,         // Sensibilidade da curva sigmoide (mmHg, ~15.0)
}

impl Default for BaroreflexModel {
    fn default() -> Self {
        Self::new()
    }
}

impl BaroreflexModel {
    pub fn new() -> Self {
        Self {
            enabled: false,     // Desabilitado por padrão no construtor (ativado na UI ou sob demanda)
            p_setpoint: 96.0,   // PAM eutrófica de estado estacionário (~96 mmHg para 122/71 mmHg)
            pam: 80.0,          // Inicializado com p_ao inicial do HemodynamicsModel (evita spike de startup)
            s_baro: 0.0,
            delta_symp: 0.0,
            delta_parasymp: 0.0,
            tau_map: 3000.0,    // 3.0 segundos — mais lento p/ suavizar transientes de ciclo a ciclo
            tau_symp: 5000.0,   // 5.0 segundos de resposta simpática adrenérgica (mais conservador)
            tau_vagus: 1000.0,  // 1.0 segundo de resposta vagal colinérgica
            k_sens: 12.0,       // Sensibilidade calibrada (tanh satura em ±3 k_sens = ±36 mmHg)
        }
    }

    pub fn step(&mut self, dt: f64, p_ao: f64) {
        // 1. Filtro passa-baixa de 1ª ordem para a Pressão Arterial Média (PAM)
        let d_pam = (p_ao - self.pam) / self.tau_map;
        self.pam += d_pam * dt;

        if self.enabled {
            // 2. Transdução Aferente Barorreceptora (Curva sigmoide de Scher & Young / Ottesen)
            let err = (self.pam - self.p_setpoint) / self.k_sens;
            self.s_baro = err.tanh();

            // 3. Eferência Integrada Bulbar (Feedback Negativo):
            // Ganho simpático conservador: a cronotropia forte de Severi pode causar
            // runaway taquicárdico → usar ganho pequeno e deixar vasoconstrição (R_tpr)
            // ser o principal mecanismo de restauração de pressão.
            let target_symp = (-self.s_baro * 0.15).clamp(-0.10, 0.25);

            // Parasimpático moderado: limitar a 0.015 para não hiperpolarizar Severi
            let target_parasymp = if self.s_baro > 0.0 {
                self.s_baro * 0.015
            } else {
                0.0
            };

            // Dinâmica temporal criticamente amortecida de 1ª ordem
            let d_symp = (target_symp - self.delta_symp) / self.tau_symp;
            self.delta_symp = (self.delta_symp + d_symp * dt).clamp(-0.10, 0.25);

            let d_parasymp = (target_parasymp - self.delta_parasymp) / self.tau_vagus;
            self.delta_parasymp = (self.delta_parasymp + d_parasymp * dt).clamp(0.0, 0.02);
        } else {
            // Relaxamento suave a zero quando desativado (denervação/ablação)
            let d_symp = (0.0 - self.delta_symp) / self.tau_symp;
            self.delta_symp += d_symp * dt;

            let d_parasymp = (0.0 - self.delta_parasymp) / self.tau_vagus;
            self.delta_parasymp += d_parasymp * dt;

            self.s_baro = 0.0;
        }
    }
}
