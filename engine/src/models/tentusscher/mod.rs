#![allow(non_snake_case)]


const R: f64 = 8.314;
const T: f64 = 310.0;
const F: f64 = 96.485;
const CM: f64 = 185.0;
const V_C: f64 = 16404.0;
const P_KNA: f64 = 0.03;
const G_K1: f64 = 5.405;
const G_KR: f64 = 0.096;
const G_KS: f64 = 0.245;
const G_NA: f64 = 14.838;
const G_BNA: f64 = 0.00029;
const G_CAL: f64 = 0.175;
const G_BCA: f64 = 0.000592;
const G_TO: f64 = 0.294;
const P_NAK: f64 = 1.362;
const K_MK: f64 = 1.0;
const K_MNA: f64 = 40.0;
const K_NACA: f64 = 1000.0;
const K_SAT: f64 = 0.1;
const ALPHA: f64 = 2.5;
const GAMMA: f64 = 0.35;
const KM_CA: f64 = 1.38;
const KM_NAI: f64 = 87.5;
const G_PCA: f64 = 0.825;
const K_PCA: f64 = 0.0005;
const G_PK: f64 = 0.0146;
const TAU_G: f64 = 2.0;
const A_REL: f64 = 0.016464;
const B_REL: f64 = 0.25;
const C_REL: f64 = 0.008232;
const K_UP: f64 = 0.00025;
const V_LEAK: f64 = 8e-5;
const VMAX_UP: f64 = 0.000425;
const BUF_C: f64 = 0.15;
const K_BUF_C: f64 = 0.001;
const BUF_SR: f64 = 10.0;
const K_BUF_SR: f64 = 0.3;
const V_SR: f64 = 1094.0;
const TAU_FCA: f64 = 2.0;

#[derive(Clone, Debug)]
pub struct VentricleCell {
    pub time: f64,
    pub v: f64,
    pub k_i: f64,
    pub na_i: f64,
    pub ca_i: f64,
    pub force: f64,
    pub xr1: f64,
    pub xr2: f64,
    pub xs: f64,
    pub m: f64,
    pub h: f64,
    pub j: f64,
    pub d: f64,
    pub f: f64,
    pub f_ca: f64,
    pub s: f64,
    pub r: f64,
    pub ca_sr: f64,
    pub g: f64,
    pub i_stim: f64,
}

impl Default for VentricleCell {
    fn default() -> Self {
        Self {
            time: 0.0,
            v: -86.2,
            k_i: 138.3,
            na_i: 11.6,
            ca_i: 0.0002,
            force: 0.0,
            xr1: 0.0,
            xr2: 1.0,
            xs: 0.0,
            m: 0.0,
            h: 0.75,
            j: 0.75,
            d: 0.0,
            f: 1.0,
            f_ca: 1.0,
            s: 1.0,
            r: 0.0,
            ca_sr: 0.2,
            g: 1.0,
            i_stim: 0.0,
        }
    }
}

impl VentricleCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compute_e_na(&self, p: &crate::models::Pharmaco) -> f64 {
        (R * T / F) * (p.nao / self.na_i).ln()
    }

    pub fn compute_e_k(&self, p: &crate::models::Pharmaco) -> f64 {
        (R * T / F) * (p.effective_ko() / self.k_i).ln()
    }

    pub fn compute_e_ks(&self, p: &crate::models::Pharmaco) -> f64 {
        (R * T / F) * ((p.ko + P_KNA * p.nao) / (self.k_i + P_KNA * self.na_i)).ln()
    }

    pub fn compute_e_ca(&self, p: &crate::models::Pharmaco) -> f64 {
        (0.5 * R * T / F) * (p.cao / self.ca_i).ln()
    }

    pub fn step(&mut self, dt: f64, p: &crate::models::Pharmaco) {
        // Compute reversal potentials
        let e_na = self.compute_e_na(p);
        let e_k = self.compute_e_k(p);
        let e_ks = self.compute_e_ks(p);
        let e_ca = self.compute_e_ca(p);

        let v = self.v;

        // Inward Rectifier K+
        let alpha_k1 = 0.1 / (1.0 + f64::exp(0.06 * (v - e_k - 200.0)));
        let beta_k1 = (3.0 * f64::exp(0.0002 * (v - e_k + 100.0)) + f64::exp(0.1 * (v - e_k - 10.0))) / (1.0 + f64::exp(-0.5 * (v - e_k)));
        let xk1_inf = alpha_k1 / (alpha_k1 + beta_k1);
        let i_k1 = G_K1 * xk1_inf * (p.ko / 5.4).sqrt() * (v - e_k) * p.block_k;

        // Transient Outward
        let i_to = G_TO * self.r * self.s * (v - e_k) * p.block_k;

        // Rapid Time Dependent K+
        let i_kr = G_KR * (p.ko / 5.4).sqrt() * self.xr1 * self.xr2 * (v - e_k) * p.block_k;

        // Slow Time Dependent K+
        let i_ks = G_KS * self.xs.powi(2) * (v - e_ks) * p.block_k;

        // L-type Ca2+ Current
        let i_cal = ((G_CAL * self.d * self.f * self.f_ca * 4.0 * v * F.powi(2) / (R * T)) *
            (self.ca_i * f64::exp(2.0 * v * F / (R * T)) - 0.341 * p.cao) /
            (f64::exp(2.0 * v * F / (R * T)) - 1.0)) * p.block_ca * p.isch_block() * p.ans_ca_modifier();

        // Na+/K+ Pump
        let i_nak = ((P_NAK * p.ko / (p.ko + K_MK) * self.na_i / (self.na_i + K_MNA)) /
            (1.0 + 0.1245 * f64::exp(-0.1 * v * F / (R * T)) + 0.0353 * f64::exp(-v * F / (R * T)))) * p.block_nak;

        // Fast Na+
        let i_na = G_NA * self.m.powi(3) * self.h * self.j * (v - e_na) * p.block_na * p.isch_block();

        // Background Na+
        let i_b_na = G_BNA * (v - e_na);

        // Na+/Ca2+ Exchanger
        let i_naca = K_NACA *
            (f64::exp(GAMMA * v * F / (R * T)) * self.na_i.powi(3) * p.cao -
             f64::exp((GAMMA - 1.0) * v * F / (R * T)) * p.nao.powi(3) * self.ca_i * ALPHA) /
            ((KM_NAI.powi(3) + p.nao.powi(3)) * (KM_CA + p.cao) *
             (1.0 + K_SAT * f64::exp((GAMMA - 1.0) * v * F / (R * T))));

        // Background Ca2+
        let i_b_ca = G_BCA * (v - e_ca);

        // Potassium Pump
        let i_p_k = G_PK * (v - e_k) / (1.0 + f64::exp((25.0 - v) / 5.98));

        // Calcium Pump
        let i_p_ca = G_PCA * self.ca_i / (self.ca_i + K_PCA);

        // Stimulus (controlled externally by conduction system)
        let i_stim = self.i_stim;

        // Calculate state derivatives
        // dv/dt
        let dv = -(i_k1 + i_to + i_kr + i_ks + i_cal + i_nak + i_na + i_b_na + i_naca + i_b_ca + i_p_k + i_p_ca + i_stim);

        // Gates
        let xr1_inf = 1.0 / (1.0 + f64::exp((-26.0 - v) / 7.0));
        let alpha_xr1 = 450.0 / (1.0 + f64::exp((-45.0 - v) / 10.0));
        let beta_xr1 = 6.0 / (1.0 + f64::exp((v + 30.0) / 11.5));
        let tau_xr1 = alpha_xr1 * beta_xr1;
        let dxr1 = (xr1_inf - self.xr1) / tau_xr1;

        let xr2_inf = 1.0 / (1.0 + f64::exp((v + 88.0) / 24.0));
        let alpha_xr2 = 3.0 / (1.0 + f64::exp((-60.0 - v) / 20.0));
        let beta_xr2 = 1.12 / (1.0 + f64::exp((v - 60.0) / 20.0));
        let tau_xr2 = alpha_xr2 * beta_xr2;
        let dxr2 = (xr2_inf - self.xr2) / tau_xr2;

        let xs_inf = 1.0 / (1.0 + f64::exp((-5.0 - v) / 14.0));
        let alpha_xs = 1100.0 / (1.0 + f64::exp((-10.0 - v) / 6.0)).sqrt();
        let beta_xs = 1.0 / (1.0 + f64::exp((v - 60.0) / 20.0));
        let tau_xs = alpha_xs * beta_xs;
        let dxs = (xs_inf - self.xs) / tau_xs;

        let m_inf = 1.0 / (1.0 + f64::exp((-56.86 - v) / 9.03)).powi(2);
        let alpha_m = 1.0 / (1.0 + f64::exp((-60.0 - v) / 5.0));
        let beta_m = 0.1 / (1.0 + f64::exp((v + 35.0) / 5.0)) + 0.1 / (1.0 + f64::exp((v - 50.0) / 200.0));
        let tau_m = alpha_m * beta_m;
        let dm = (m_inf - self.m) / tau_m;

        let h_inf = 1.0 / (1.0 + f64::exp((v + 71.55) / 7.43)).powi(2);
        let alpha_h = if v < -40.0 { 0.057 * f64::exp(-(v + 80.0) / 6.8) } else { 0.0 };
        let beta_h = if v < -40.0 { 2.7 * f64::exp(0.079 * v) + 310000.0 * f64::exp(0.3485 * v) } else { 0.77 / (0.13 * (1.0 + f64::exp((v + 10.66) / -11.1))) };
        let tau_h = 1.0 / (alpha_h + beta_h);
        let dh = (h_inf - self.h) / tau_h;

        let j_inf = 1.0 / (1.0 + f64::exp((v + 71.55) / 7.43)).powi(2);
        let alpha_j = if v < -40.0 { (-25428.0 * f64::exp(0.2444 * v) - 6.948e-6 * f64::exp(-0.04391 * v)) * (v + 37.78) / (1.0 + f64::exp(0.311 * (v + 79.23))) } else { 0.0 };
        let beta_j = if v < -40.0 { 0.02424 * f64::exp(-0.01052 * v) / (1.0 + f64::exp(-0.1378 * (v + 40.14))) } else { 0.6 * f64::exp(0.057 * v) / (1.0 + f64::exp(-0.1 * (v + 32.0))) };
        let tau_j = 1.0 / (alpha_j + beta_j);
        let dj = (j_inf - self.j) / tau_j;

        let d_inf = 1.0 / (1.0 + f64::exp((-5.0 - v) / 7.5));
        let alpha_d = 1.4 / (1.0 + f64::exp((-35.0 - v) / 13.0)) + 0.25;
        let beta_d = 1.4 / (1.0 + f64::exp((v + 5.0) / 5.0));
        let gamma_d = 1.0 / (1.0 + f64::exp((50.0 - v) / 20.0));
        let tau_d = alpha_d * beta_d + gamma_d;
        let dd = (d_inf - self.d) / tau_d;

        let f_inf = 1.0 / (1.0 + f64::exp((v + 20.0) / 7.0));
        let tau_f = 1125.0 * f64::exp(-(v + 27.0).powi(2) / 240.0) + 80.0 + 165.0 / (1.0 + f64::exp((25.0 - v) / 10.0));
        let df = (f_inf - self.f) / tau_f;

        let alpha_fca = 1.0 / (1.0 + (self.ca_i / 0.000325).powi(8));
        let beta_fca = 0.1 / (1.0 + f64::exp((self.ca_i - 0.0005) / 0.0001));
        let gama_fca = 0.2 / (1.0 + f64::exp((self.ca_i - 0.00075) / 0.0008));
        let fca_inf = (alpha_fca + beta_fca + gama_fca + 0.23) / 1.46;
        let df_ca = if fca_inf > self.f_ca && v > -60.0 { 0.0 } else { (fca_inf - self.f_ca) / TAU_FCA };

        let s_inf = 1.0 / (1.0 + f64::exp((v + 20.0) / 5.0));
        let tau_s = 85.0 * f64::exp(-(v + 45.0).powi(2) / 320.0) + 5.0 / (1.0 + f64::exp((v - 20.0) / 5.0)) + 3.0;
        let ds = (s_inf - self.s) / tau_s;

        let r_inf = 1.0 / (1.0 + f64::exp((20.0 - v) / 6.0));
        let tau_r = 9.5 * f64::exp(-(v + 40.0).powi(2) / 1800.0) + 0.8;
        let dr = (r_inf - self.r) / tau_r;

        // Intracellular Calcium Dynamics
        let g_inf = if self.ca_i < 0.00035 { 1.0 / (1.0 + (self.ca_i / 0.00035).powi(6)) } else { 1.0 / (1.0 + (self.ca_i / 0.00035).powi(16)) };
        let dg = if g_inf > self.g && v > -60.0 { 0.0 } else { (g_inf - self.g) / TAU_G };

        let i_rel = (A_REL * self.ca_sr.powi(2) / (B_REL.powi(2) + self.ca_sr.powi(2)) + C_REL) * self.d * self.g;
        let i_up = VMAX_UP / (1.0 + K_UP.powi(2) / self.ca_i.powi(2));
        let i_leak = V_LEAK * (self.ca_sr - self.ca_i);

        let dca_i_total = -(i_cal + i_b_ca + i_p_ca - 2.0 * i_naca) / (2.0 * V_C * F) * CM + i_leak - i_up + i_rel;
        let f_jca_i_free = 1.0 / (1.0 + (BUF_C * K_BUF_C) / (self.ca_i + K_BUF_C).powi(2));
        let dca_i = dca_i_total * f_jca_i_free;

        let dca_sr_total = (V_C / V_SR) * (i_up - (i_rel + i_leak));
        let f_jca_sr_free = 1.0 / (1.0 + (BUF_SR * K_BUF_SR) / (self.ca_sr + K_BUF_SR).powi(2));
        let dca_sr = dca_sr_total * f_jca_sr_free;

        // Ion concentrations
        let dna_i = -(i_na + i_b_na + 3.0 * i_nak + 3.0 * i_naca) * CM / (V_C * F);
        let dk_i = -(i_k1 + i_to + i_kr + i_ks + i_p_k + i_stim - 2.0 * i_nak) * CM / (V_C * F);

        // Update states (Forward Euler)
        self.v += dv * dt;
        self.xr1 += dxr1 * dt;
        self.xr2 += dxr2 * dt;
        self.xs += dxs * dt;
        self.m += dm * dt;
        self.h += dh * dt;
        self.j += dj * dt;
        self.d += dd * dt;
        self.f += df * dt;
        self.f_ca += df_ca * dt;
        self.s += ds * dt;
        self.r += dr * dt;
        self.g += dg * dt;
        self.ca_i += dca_i * dt;
        self.ca_sr += dca_sr * dt;
        self.na_i += dna_i * dt;
        self.k_i += dk_i * dt;
        // Phenomenological Force Model (Hill-type curve with delay)
        let kd: f64 = 0.0006; // mM, dissociation constant
        let n: f64 = 3.0; // Hill coefficient
        let f_steady = (self.ca_i.powf(n)) / (kd.powf(n) + self.ca_i.powf(n));
        let tau_force = 50.0; // ms, delay for cross-bridge attachment/detachment
        let dforce = (f_steady - self.force) / tau_force;
        self.force += dforce * dt;

        self.time += dt;
    }
}
