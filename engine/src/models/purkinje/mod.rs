#![allow(non_snake_case)]

use crate::models::Pharmaco;

// =============================================================================
// Stewart et al. (2009) — Mathematical Model of the Human Purkinje Action Potential
// Transposed to pure Rust from CellML / PMR Exposure 38cf8387b0707f0ef6947f009710aeb5
// =============================================================================

const R: f64 = 8314.472;
const T: f64 = 310.0;
const F: f64 = 96485.3415;
const CM: f64 = 0.185; // microF
const V_C: f64 = 0.016404; // micrometre3
const P_KNA: f64 = 0.03;
const G_F_NA: f64 = 0.0145654;
const G_F_K: f64 = 0.0234346;
const G_K1: f64 = 0.065;
const G_KR: f64 = 0.0918;
const G_KS: f64 = 0.2352;
const G_NA: f64 = 130.5744;
const G_BNA: f64 = 0.00029;
const G_CAL: f64 = 3.98e-5;
const G_BCA: f64 = 0.000592;
const G_TO: f64 = 0.08184;
const G_SUS: f64 = 0.0227;
const P_NAK: f64 = 2.724;
const K_MK: f64 = 1.0;
const K_MNA: f64 = 40.0;
const K_NACA: f64 = 1000.0;
const K_SAT: f64 = 0.1;
const ALPHA: f64 = 2.5;
const GAMMA: f64 = 0.35;
const KM_CA: f64 = 1.38;
const KM_NAI: f64 = 87.5;
const G_PCA: f64 = 0.1238;
const K_PCA: f64 = 0.0005;
const G_PK: f64 = 0.0146;
const K1_PRIME: f64 = 0.15;
const K2_PRIME: f64 = 0.045;
const K3: f64 = 0.06;
const K4: f64 = 0.005;
const EC: f64 = 1.5;
const MAX_SR: f64 = 2.5;
const MIN_SR: f64 = 1.0;
const V_REL: f64 = 0.102;
const V_XFER: f64 = 0.0038;
const K_UP: f64 = 0.00025;
const V_LEAK: f64 = 0.00036;
const VMAX_UP: f64 = 0.006375;
const BUF_C: f64 = 0.2;
const K_BUF_C: f64 = 0.001;
const BUF_SR: f64 = 10.0;
const K_BUF_SR: f64 = 0.3;
const BUF_SS: f64 = 0.4;
const K_BUF_SS: f64 = 0.00025;
const V_SR: f64 = 0.001094;
const V_SS: f64 = 5.468e-5;

#[derive(Clone, Debug)]
pub struct PurkinjeCell {
    pub time: f64,
    pub v: f64,
    pub k_i: f64,
    pub na_i: f64,
    pub ca_i: f64,
    pub y: f64,
    pub xr1: f64,
    pub xr2: f64,
    pub xs: f64,
    pub m: f64,
    pub h: f64,
    pub j: f64,
    pub ca_ss: f64,
    pub d: f64,
    pub f: f64,
    pub f2: f64,
    pub f_cass: f64,
    pub s: f64,
    pub r: f64,
    pub ca_sr: f64,
    pub r_prime: f64,
    pub i_stim: f64,
}

impl Default for PurkinjeCell {
    fn default() -> Self {
        Self {
            time: 0.0,
            v: -69.1370441635924,
            k_i: 136.781894160227,
            na_i: 8.80420286531673,
            ca_i: 0.000101878186157052,
            y: 0.0457562667986602,
            xr1: 0.00550281999719088,
            xr2: 0.313213286437995,
            xs: 0.00953708522974789,
            m: 0.0417391656294997,
            h: 0.190678733735145,
            j: 0.238219836154029,
            ca_ss: 0.000446818714055411,
            d: 0.000287906256206415,
            f: 0.989328560287987,
            f2: 0.995474890442185,
            f_cass: 0.999955429598213,
            s: 0.96386101799501,
            r: 0.00103618091196912,
            ca_sr: 3.10836886659417,
            r_prime: 0.991580051907845,
            i_stim: 0.0,
        }
    }
}

impl PurkinjeCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compute_e_na(&self, p: &Pharmaco) -> f64 {
        (R * T / F) * (p.nao / self.na_i).ln()
    }

    pub fn compute_e_k(&self, p: &Pharmaco) -> f64 {
        (R * T / F) * (p.effective_ko() / self.k_i).ln()
    }

    pub fn compute_e_ks(&self, p: &Pharmaco) -> f64 {
        (R * T / F) * ((p.ko + P_KNA * p.nao) / (self.k_i + P_KNA * self.na_i)).ln()
    }

    pub fn compute_e_ca(&self, p: &Pharmaco) -> f64 {
        (0.5 * R * T / F) * (p.cao / self.ca_i).ln()
    }

    pub fn step(&mut self, dt: f64, p: &Pharmaco) {
        let v = self.v;

        // Reversal potentials
        let e_na = self.compute_e_na(p);
        let e_k = self.compute_e_k(p);
        let e_ks = self.compute_e_ks(p);
        let e_ca = self.compute_e_ca(p);

        // --- Gating Variables ---
        // L-type calcium inactivation: f
        let f_inf = 1.0 / (1.0 + f64::exp((v + 20.0) / 7.0));
        let tau_f = 1102.5 * f64::exp(-(v + 27.0).powi(2) / 225.0)
            + 200.0 / (1.0 + f64::exp((13.0 - v) / 10.0))
            + 180.0 / (1.0 + f64::exp((v + 30.0) / 10.0))
            + 20.0;
        let df = (f_inf - self.f) / tau_f;

        // L-type calcium inactivation: f2
        let f2_inf = 0.67 / (1.0 + f64::exp((v + 35.0) / 7.0)) + 0.33;
        let tau_f2 = 562.0 * f64::exp(-(v + 27.0).powi(2) / 240.0)
            + 31.0 / (1.0 + f64::exp((25.0 - v) / 10.0))
            + 80.0 / (1.0 + f64::exp((v + 30.0) / 10.0));
        let df2 = (f2_inf - self.f2) / tau_f2;

        // Calcium-dependent inactivation: fCass
        let fcass_inf = 0.6 / (1.0 + (self.ca_ss / 0.05).powi(2)) + 0.4;
        let tau_fcass = 80.0 / (1.0 + (self.ca_ss / 0.05).powi(2)) + 2.0;
        let df_cass = (fcass_inf - self.f_cass) / tau_fcass;

        // Transient outward current: s gate
        let s_inf = 1.0 / (1.0 + f64::exp((v + 27.0) / 13.0));
        let tau_s = 85.0 * f64::exp(-(v + 25.0).powi(2) / 320.0)
            + 5.0 / (1.0 + f64::exp((v - 40.0) / 5.0))
            + 42.0;
        let ds = (s_inf - self.s) / tau_s;

        // Transient outward current: r gate
        let r_inf = 1.0 / (1.0 + f64::exp((20.0 - v) / 13.0));
        let tau_r = 10.45 * f64::exp(-(v + 40.0).powi(2) / 1800.0) + 7.3;
        let dr = (r_inf - self.r) / tau_r;

        // Hyperpolarization-activated (funny) current: y gate
        let y_inf = 1.0 / (1.0 + f64::exp((v + 80.6) / 6.8));
        let alpha_y = f64::exp(-2.9 - 0.04 * v);
        let beta_y = f64::exp(3.6 + 0.11 * v);
        let tau_y = 4000.0 / (alpha_y + beta_y);
        let dy = (y_inf - self.y) / tau_y;

        // Rapid delayed rectifier: Xr1 gate
        let xr1_inf = 1.0 / (1.0 + f64::exp((-26.0 - v) / 7.0));
        let alpha_xr1 = 450.0 / (1.0 + f64::exp((-45.0 - v) / 10.0));
        let beta_xr1 = 6.0 / (1.0 + f64::exp((v + 30.0) / 11.5));
        let tau_xr1 = alpha_xr1 * beta_xr1;
        let dxr1 = (xr1_inf - self.xr1) / tau_xr1;

        // Rapid delayed rectifier: Xr2 gate
        let xr2_inf = 1.0 / (1.0 + f64::exp((v + 88.0) / 24.0));
        let alpha_xr2 = 3.0 / (1.0 + f64::exp((-60.0 - v) / 20.0));
        let beta_xr2 = 1.12 / (1.0 + f64::exp((v - 60.0) / 20.0));
        let tau_xr2 = alpha_xr2 * beta_xr2;
        let dxr2 = (xr2_inf - self.xr2) / tau_xr2;

        // Slow delayed rectifier: Xs gate
        let xs_inf = 1.0 / (1.0 + f64::exp((-5.0 - v) / 14.0));
        let alpha_xs = 1400.0 / (1.0 + f64::exp((5.0 - v) / 6.0)).sqrt();
        let beta_xs = 1.0 / (1.0 + f64::exp((v - 35.0) / 15.0));
        let tau_xs = alpha_xs * beta_xs + 80.0;
        let dxs = (xs_inf - self.xs) / tau_xs;

        // Fast sodium current: m gate
        let m_inf = 1.0 / (1.0 + f64::exp((-56.86 - v) / 9.03)).powi(2);
        let alpha_m = 1.0 / (1.0 + f64::exp((-60.0 - v) / 5.0));
        let beta_m = 0.1 / (1.0 + f64::exp((v + 35.0) / 5.0)) + 0.1 / (1.0 + f64::exp((v - 50.0) / 200.0));
        let tau_m = alpha_m * beta_m;
        let dm = (m_inf - self.m) / tau_m;

        // Fast sodium current: h gate
        let h_inf = 1.0 / (1.0 + f64::exp((v + 71.55) / 7.43)).powi(2);
        let alpha_h = if v < -40.0 { 0.057 * f64::exp(-(v + 80.0) / 6.8) } else { 0.0 };
        let beta_h = if v < -40.0 {
            2.7 * f64::exp(0.079 * v) + 310000.0 * f64::exp(0.3485 * v)
        } else {
            0.77 / (0.13 * (1.0 + f64::exp((v + 10.66) / -11.1)))
        };
        let tau_h = 1.0 / (alpha_h + beta_h);
        let dh = (h_inf - self.h) / tau_h;

        // Fast sodium current: j gate
        let j_inf = 1.0 / (1.0 + f64::exp((v + 71.55) / 7.43)).powi(2);
        let alpha_j = if v < -40.0 {
            ((-25428.0 * f64::exp(0.2444 * v) - 6.948e-6 * f64::exp(-0.04391 * v)) * (v + 37.78))
                / (1.0 + f64::exp(0.311 * (v + 79.23)))
        } else {
            0.0
        };
        let beta_j = if v < -40.0 {
            (0.02424 * f64::exp(-0.01052 * v)) / (1.0 + f64::exp(-0.1378 * (v + 40.14)))
        } else {
            (0.6 * f64::exp(0.057 * v)) / (1.0 + f64::exp(-0.1 * (v + 32.0)))
        };
        let tau_j = 1.0 / (alpha_j + beta_j);
        let dj = (j_inf - self.j) / tau_j;

        // L-type calcium activation: d gate
        let d_inf = 1.0 / (1.0 + f64::exp((-8.0 - v) / 7.5));
        let alpha_d = 1.4 / (1.0 + f64::exp((-35.0 - v) / 13.0)) + 0.25;
        let beta_d = 1.4 / (1.0 + f64::exp((v + 5.0) / 5.0));
        let gamma_d = 1.0 / (1.0 + f64::exp((50.0 - v) / 20.0));
        let tau_d = alpha_d * beta_d + gamma_d;
        let dd = (d_inf - self.d) / tau_d;

        // --- Transmembrane Currents ---
        // Na+/K+ pump
        let i_nak = (((P_NAK * p.ko / (p.ko + K_MK)) * self.na_i / (self.na_i + K_MNA))
            / (1.0 + 0.1245 * f64::exp(-0.1 * v * F / (R * T)) + 0.0353 * f64::exp(-v * F / (R * T))))
            * p.block_nak;

        // Fast Na+
        let i_na = G_NA * self.m.powi(3) * self.h * self.j * (v - e_na) * p.block_na * p.isch_block();

        // Background Na+
        let i_b_na = G_BNA * (v - e_na);

        // Na+/Ca2+ Exchanger
        let i_naca = (K_NACA
            * (f64::exp(GAMMA * v * F / (R * T)) * self.na_i.powi(3) * p.cao
                - f64::exp((GAMMA - 1.0) * v * F / (R * T)) * p.nao.powi(3) * self.ca_i * ALPHA))
            / ((KM_NAI.powi(3) + p.nao.powi(3))
                * (KM_CA + p.cao)
                * (1.0 + K_SAT * f64::exp((GAMMA - 1.0) * v * F / (R * T))));

        // Hyperpolarization-activated funny current (If)
        // Modulated by autonomic sympathetic/parasympathetic tone
        let if_mod = 1.0 + (p.symp * 0.2) - (p.parasymp * 0.2);
        let i_f_na = self.y * G_F_NA * (v - e_na) * p.isch_block() * if_mod;
        let i_f_k = self.y * G_F_K * (v - e_k) * p.block_k * if_mod;
        let i_f = i_f_na + i_f_k;

        // Inward rectifier K+
        let xk1_inf = 1.0 / (1.0 + f64::exp(0.1 * (v + 75.44)));
        let i_k1 = G_K1 * xk1_inf * ((v - 8.0) - e_k) * p.block_k;

        // Transient outward K+
        let i_to = G_TO * self.r * self.s * (v - e_k) * p.block_k;

        // Sustained outward K+
        let a_sus = 1.0 / (1.0 + f64::exp((5.0 - v) / 17.0));
        let i_sus = G_SUS * a_sus * (v - e_k) * p.block_k;

        // Rapid delayed rectifier K+
        let i_kr = G_KR * (p.ko / 5.4).sqrt() * self.xr1 * self.xr2 * (v - e_k) * p.block_k;

        // Slow delayed rectifier K+
        let i_ks = G_KS * self.xs.powi(2) * (v - e_ks) * p.block_k;

        // L-type calcium current
        let i_cal = (((G_CAL * self.d * self.f * self.f2 * self.f_cass * 4.0 * (v - 15.0) * F.powi(2) / (R * T))
            * (0.25 * self.ca_ss * f64::exp(2.0 * (v - 15.0) * F / (R * T)) - p.cao))
            / (f64::exp(2.0 * (v - 15.0) * F / (R * T)) - 1.0))
            * p.block_ca * p.isch_block() * p.ans_ca_modifier();

        // Background Ca2+
        let i_b_ca = G_BCA * (v - e_ca);

        // Potassium pump
        let i_p_k = (G_PK * (v - e_k)) / (1.0 + f64::exp((25.0 - v) / 5.98));

        // Calcium pump
        let i_p_ca = (G_PCA * self.ca_i) / (self.ca_i + K_PCA);

        // Stimulus (controlled by conduction system)
        let i_stim = self.i_stim;

        // Membrane potential derivative dv/dt
        let dv = -(i_k1 + i_to + i_sus + i_kr + i_ks + i_cal + i_nak + i_na + i_b_na + i_naca + i_b_ca + i_p_k + i_p_ca + i_f + i_stim);

        // Ion concentrations derivatives
        let dna_i = (-(i_na + i_b_na + i_f_na + 3.0 * i_nak + 3.0 * i_naca) / (V_C * F)) * CM;
        let dk_i = (-(i_k1 + i_to + i_f_k + i_sus + i_kr + i_ks + i_p_k + i_stim - 2.0 * i_nak) / (V_C * F)) * CM;

        // Sarcoplasmic reticulum and subspace calcium fluxes
        let i_up = VMAX_UP / (1.0 + K_UP.powi(2) / self.ca_i.powi(2));
        let i_leak = V_LEAK * (self.ca_sr - self.ca_i);
        let i_xfer = V_XFER * (self.ca_ss - self.ca_i);

        let kcasr = MAX_SR - (MAX_SR - MIN_SR) / (1.0 + (EC / self.ca_sr).powi(2));
        let k1 = K1_PRIME / kcasr;
        let k2 = K2_PRIME * kcasr;
        let dr_prime = -k2 * self.ca_ss * self.r_prime + K4 * (1.0 - self.r_prime);
        let o_ryr = (k1 * self.ca_ss.powi(2) * self.r_prime) / (K3 + k1 * self.ca_ss.powi(2));
        let i_rel = V_REL * o_ryr * (self.ca_sr - self.ca_ss);

        // Calcium buffering
        let ca_i_bufc = 1.0 / (1.0 + (BUF_C * K_BUF_C) / (self.ca_i + K_BUF_C).powi(2));
        let dca_i = ca_i_bufc * (((i_leak - i_up) * V_SR) / V_C + i_xfer - ((i_b_ca + i_p_ca - 2.0 * i_naca) * CM) / (2.0 * V_C * F));

        let ca_sr_bufsr = 1.0 / (1.0 + (BUF_SR * K_BUF_SR) / (self.ca_sr + K_BUF_SR).powi(2));
        let dca_sr = ca_sr_bufsr * (i_up - (i_rel + i_leak));

        let ca_ss_bufss = 1.0 / (1.0 + (BUF_SS * K_BUF_SS) / (self.ca_ss + K_BUF_SS).powi(2));
        let dca_ss = ca_ss_bufss * (-(i_cal * CM) / (2.0 * V_SS * F) + (i_rel * V_SR) / V_SS - (i_xfer * V_C) / V_SS);

        // Forward Euler Integration Step
        self.v += dv * dt;
        self.f += df * dt;
        self.f2 += df2 * dt;
        self.f_cass += df_cass * dt;
        self.s += ds * dt;
        self.r += dr * dt;
        self.y += dy * dt;
        self.xr1 += dxr1 * dt;
        self.xr2 += dxr2 * dt;
        self.xs += dxs * dt;
        self.m += dm * dt;
        self.h += dh * dt;
        self.j += dj * dt;
        self.d += dd * dt;
        self.na_i += dna_i * dt;
        self.k_i += dk_i * dt;
        self.ca_i += dca_i * dt;
        self.ca_sr += dca_sr * dt;
        self.ca_ss += dca_ss * dt;
        self.r_prime += dr_prime * dt;

        self.time += dt;
    }
}
