#![allow(non_snake_case)]

use crate::models::Pharmaco;

// =============================================================================
// MacCannell et al. (2007) — Mathematical Model of Cardiac Fibroblasts
// Reference: MacCannell KA, Bazzazi H, Chilton L, Shibukawa Y, Clark RB, Giles WR.
// "A mathematical model of electrotonic interactions between ventricular myocytes
// and fibroblasts." Biophysical Journal, 92(9): 2863–2875, 2007.
// =============================================================================

const C_F: f64 = 6.3; // pF (Capacitância de membrana do fibroblasto cardíaco)
const R: f64 = 8.314;
const T: f64 = 310.0;
const F: f64 = 96.485;

// Condutâncias iônicas intrínsecas do fibroblasto
const G_KV: f64 = 0.25; // nS/pF (Condutância máxima de potássio ativado por voltagem)
const G_K1: f64 = 0.4822; // nS (Condutância retificadora de entrada)
const G_B: f64 = 0.0095; // nS/pF (Condutância de fuga de fundo)
const E_B: f64 = -1.0; // mV (Potencial de reversão da corrente de fuga)
const I_NAK_MAX: f64 = 0.702; // pA/pF (Densidade máxima da bomba Na+/K+)
const K_MK: f64 = 1.0; // mM
const K_MNA: f64 = 11.0; // mM
const NA_I_FIB: f64 = 8.55; // mM (Sódio intracelular no fibroblasto)
const K_I_FIB: f64 = 140.0; // mM (Potássio intracelular no fibroblasto)

#[derive(Clone, Debug)]
pub struct FibroblastCell {
    pub time: f64,
    pub v: f64, // Potencial transmembrana do fibroblasto (mV)
    pub r: f64, // Portão de ativação de I_Kv
    pub s: f64, // Portão de inativação de I_Kv
}

impl Default for FibroblastCell {
    fn default() -> Self {
        Self {
            time: 0.0,
            v: -39.5, // Potencial de repouso despolarizado característico (-35 a -45 mV)
            r: 0.08,
            s: 0.85,
        }
    }
}

impl FibroblastCell {
    pub fn new() -> Self {
        Self::default()
    }

    /// Executa um passo de integração Forward Euler (dt em milissegundos)
    /// e calcula a corrente eletrotônica bidirecional com o miócito acoplado.
    /// Retorna a corrente de junção comunicante `i_gap` em picoamperes (pA).
    pub fn step(&mut self, dt: f64, v_myo: f64, g_gap: f64, p: &Pharmaco) -> f64 {
        let v = self.v;

        // Potencial de reversão de Potássio (Nernst)
        let e_k = (R * T / F) * (p.effective_ko() / K_I_FIB).ln();

        // 1. Corrente de Potássio Ativada por Voltagem (I_Kv)
        let r_inf = 1.0 / (1.0 + f64::exp(-(v + 20.0) / 11.0));
        let tau_r = 20.3 + 138.0 * f64::exp(-((v + 20.0) / 25.9).powi(2));
        
        let s_inf = 1.0 / (1.0 + f64::exp((v + 23.0) / 16.0));
        let tau_s = 1574.0 + 5268.0 * f64::exp(-((v + 23.0) / 22.7).powi(2));

        let i_kv = G_KV * C_F * self.r * self.s * (v - e_k) * p.block_k;

        // 2. Corrente Retificadora de Entrada de Potássio (I_K1)
        let i_k1 = (G_K1 * (v - e_k) * (p.effective_ko() / 5.4).sqrt() /
            (1.0 + f64::exp((v - e_k - 10.0) / 15.0))) * p.block_k;

        // 3. Corrente de Fuga Não-Seletiva (I_b)
        let i_b = G_B * C_F * (v - E_B);

        // 4. Bomba de Sódio/Potássio (I_NaK)
        let f_nak = 1.0 / (1.0 + 0.1245 * f64::exp(-0.1 * v * F / (R * T)) + 0.0353 * f64::exp(-v * F / (R * T)));
        let i_nak = I_NAK_MAX * C_F * (p.effective_ko() / (p.effective_ko() + K_MK)) *
            (NA_I_FIB.powf(1.5) / (NA_I_FIB.powf(1.5) + K_MNA.powf(1.5))) * f_nak * p.block_nak;

        // Corrente total de canais iônicos do fibroblasto (pA)
        let i_ion_fib = i_kv + i_k1 + i_b + i_nak;

        // 5. Corrente de Acoplamento Eletrotônico via Junção Comunicante (I_gap em pA)
        // Positiva se v_myo > v (dreno de corrente saindo do miócito para o fibroblasto)
        let i_gap = g_gap * (v_myo - v);

        // Equação diferencial do fibroblasto (dV_fib/dt em mV/ms = V/s)
        let dv_fib = -(i_ion_fib - i_gap) / C_F;

        // Atualização dos estados por Forward Euler
        self.r += dt * (r_inf - self.r) / tau_r;
        self.s += dt * (s_inf - self.s) / tau_s;
        self.v += dt * dv_fib;
        self.time += dt;

        i_gap
    }
}
