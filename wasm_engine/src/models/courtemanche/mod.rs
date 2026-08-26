// Courtemanche Atrium Cell Model - Rust Refactoring

#[derive(Debug, Clone)]
pub struct AtriumCell {
    pub v: f64,
    pub na_i: f64,
    pub m: f64,
    pub h: f64,
    pub j: f64,
    pub k_i: f64,
    pub oa: f64,
    pub oi: f64,
    pub ua: f64,
    pub ui: f64,
    pub xr: f64,
    pub xs: f64,
    pub ca_i: f64,
    pub d: f64,
    pub f: f64,
    pub f_ca: f64,
    pub ca_rel: f64,
    pub u: f64,
    pub v_gate: f64,
    pub w: f64,
    pub ca_up: f64,
}

impl Default for AtriumCell {
    fn default() -> Self {
        Self {
            v: -81.18,
            na_i: 11.17,
            m: 0.002908,
            h: 0.9649,
            j: 0.9775,
            k_i: 139.0,
            oa: 0.03043,
            oi: 0.9992,
            ua: 0.004966,
            ui: 0.9986,
            xr: 0.00003296,
            xs: 0.01869,
            ca_i: 0.0001013,
            d: 0.0001367,
            f: 0.9996,
            f_ca: 0.7755,
            ca_rel: 1.488,
            u: 0.0,
            v_gate: 1.0,
            w: 0.9992,
            ca_up: 1.488,
        }
    }
}

pub struct Constants {
    pub r: f64,
    pub t: f64,
    pub f_faraday: f64,
    pub cm: f64,
    pub g_na: f64,
    pub na_o: f64,
    pub g_k1: f64,
    pub k_o: f64,
    pub k_q10: f64,
    pub g_to: f64,
    pub g_kr: f64,
    pub g_ks: f64,
    pub g_ca_l: f64,
    pub km_na_i: f64,
    pub km_k_o: f64,
    pub i_nak_max: f64,
    pub g_b_na: f64,
    pub g_b_ca: f64,
    pub g_b_k: f64,
    pub ca_o: f64,
    pub i_naca_max: f64,
    pub k_mna: f64,
    pub k_mca: f64,
    pub k_sat: f64,
    pub gamma: f64,
    pub i_cap_max: f64,
    pub k_rel: f64,
    pub tau_tr: f64,
    pub i_up_max: f64,
    pub k_up: f64,
    pub ca_up_max: f64,
    pub cmdn_max: f64,
    pub trpn_max: f64,
    pub csqn_max: f64,
    pub km_cmdn: f64,
    pub km_trpn: f64,
    pub km_csqn: f64,
    pub v_cell: f64,
    pub v_i: f64,
    pub tau_f_ca: f64,
    pub sigma: f64,
    pub tau_u: f64,
    pub v_rel: f64,
    pub v_up: f64,
}

impl Constants {
    pub fn new(p: &crate::models::Pharmaco) -> Self {
        let na_o = p.nao;
        let v_cell = 20100.0;
        Self {
            r: 8.3143,
            t: 310.0,
            f_faraday: 96.4867,
            cm: 100.0,
            g_na: 7.8,
            na_o,
            g_k1: 0.09,
            k_o: p.ko,
            k_q10: 3.0,
            g_to: 0.1652,
            g_kr: 0.029411765,
            g_ks: 0.12941176,
            g_ca_l: 0.12375,
            km_na_i: 10.0,
            km_k_o: 1.5,
            i_nak_max: 0.59933874,
            g_b_na: 0.0006744375,
            g_b_ca: 0.001131,
            g_b_k: 0.0,
            ca_o: p.cao,
            i_naca_max: 1600.0,
            k_mna: 87.5,
            k_mca: 1.38,
            k_sat: 0.1,
            gamma: 0.35,
            i_cap_max: 0.275,
            k_rel: 30.0,
            tau_tr: 180.0,
            i_up_max: 0.005,
            k_up: 0.00092,
            ca_up_max: 15.0,
            cmdn_max: 0.05,
            trpn_max: 0.07,
            csqn_max: 10.0,
            km_cmdn: 0.00238,
            km_trpn: 0.0005,
            km_csqn: 0.8,
            v_cell,
            v_i: v_cell * 0.68,
            tau_f_ca: 2.0,
            sigma: (1.0 / 7.0) * ((na_o / 67.3).exp() - 1.0),
            tau_u: 8.0,
            v_rel: 0.0048 * v_cell,
            v_up: 0.0552 * v_cell,
        }
    }
}

impl AtriumCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, dt: f64, p: &crate::models::Pharmaco) {
        let c = Constants::new(p);
        
        let f_ca_inf = (1.0 + self.ca_i / 0.00035).powi(-1);
        let df_ca = (f_ca_inf - self.f_ca) / c.tau_f_ca;
        
        let d_inf = (1.0 + ((self.v + 10.0) / -8.0).exp()).powi(-1);
        let tau_d = if (self.v + 10.0).abs() < 1e-10 {
            4.579 / (1.0 + ((self.v + 10.0) / -6.24).exp())
        } else {
            (1.0 - ((self.v + 10.0) / -6.24).exp()) / (0.035 * (self.v + 10.0) * (1.0 + ((self.v + 10.0) / -6.24).exp()))
        };
        let dd = (d_inf - self.d) / tau_d;
        
        let f_inf = (-(self.v + 28.0) / 6.9).exp() / (1.0 + (-(self.v + 28.0) / 6.9).exp());
        let tau_f = 9.0 * (0.0197 * (-(0.0337_f64).powi(2) * (self.v + 10.0).powi(2)).exp() + 0.02).powi(-1);
        let df = (f_inf - self.f) / tau_f;
        
        let tau_w = if (self.v - 7.9).abs() < 1e-10 {
            6.0 * 0.2 / 1.3
        } else {
            6.0 * (1.0 - (-(self.v - 7.9) / 5.0).exp()) / ((1.0 + 0.3 * (-(self.v - 7.9) / 5.0).exp()) * 1.0 * (self.v - 7.9))
        };
        let w_inf = 1.0 - (1.0 + (-(self.v - 40.0) / 17.0).exp()).powi(-1);
        let dw = (w_inf - self.w) / tau_w;
        
        let alpha_m = if self.v == -47.13 {
            3.2
        } else {
            0.32 * (self.v + 47.13) / (1.0 - (-0.1 * (self.v + 47.13)).exp())
        };
        let beta_m = 0.08 * (-self.v / 11.0).exp();
        let m_inf = alpha_m / (alpha_m + beta_m);
        let tau_m = 1.0 / (alpha_m + beta_m);
        let dm = (m_inf - self.m) / tau_m;
        
        let alpha_h = if self.v < -40.0 {
            0.135 * ((self.v + 80.0) / -6.8).exp()
        } else {
            0.0
        };
        let beta_h = if self.v < -40.0 {
            3.56 * (0.079 * self.v).exp() + 310000.0 * (0.35 * self.v).exp()
        } else {
            1.0 / (0.13 * (1.0 + ((self.v + 10.66) / -11.1).exp()))
        };
        let h_inf = alpha_h / (alpha_h + beta_h);
        let tau_h = 1.0 / (alpha_h + beta_h);
        let dh = (h_inf - self.h) / tau_h;
        
        let alpha_j = if self.v < -40.0 {
            (-127140.0 * (0.2444 * self.v).exp() - 3.474e-5 * (-0.04391 * self.v).exp()) * (self.v + 37.78) / (1.0 + (0.311 * (self.v + 79.23)).exp())
        } else {
            0.0
        };
        let beta_j = if self.v < -40.0 {
            0.1212 * (-0.01052 * self.v).exp() / (1.0 + (-0.1378 * (self.v + 40.14)).exp())
        } else {
            0.3 * (-2.535e-7 * self.v).exp() / (1.0 + (-0.1 * (self.v + 32.0)).exp())
        };
        let j_inf = alpha_j / (alpha_j + beta_j);
        let tau_j = 1.0 / (alpha_j + beta_j);
        let dj = (j_inf - self.j) / tau_j;
        
        let alpha_oa = 0.65 * (((self.v - -10.0) / -8.5).exp() + ((self.v - -10.0 - 40.0) / -59.0).exp()).powi(-1);
        let beta_oa = 0.65 * (2.5 + ((self.v - -10.0 + 72.0) / 17.0).exp()).powi(-1);
        let tau_oa = (alpha_oa + beta_oa).powi(-1) / c.k_q10;
        let oa_inf = (1.0 + ((self.v - -10.0 + 10.47) / -17.54).exp()).powi(-1);
        let doa = (oa_inf - self.oa) / tau_oa;
        
        let alpha_oi = (18.53 + 1.0 * ((self.v - -10.0 + 103.7) / 10.95).exp()).powi(-1);
        let beta_oi = (35.56 + 1.0 * ((self.v - -10.0 - 8.74) / -7.44).exp()).powi(-1);
        let tau_oi = (alpha_oi + beta_oi).powi(-1) / c.k_q10;
        let oi_inf = (1.0 + ((self.v - -10.0 + 33.1) / 5.3).exp()).powi(-1);
        let doi = (oi_inf - self.oi) / tau_oi;
        
        let alpha_ua = 0.65 * (((self.v - -10.0) / -8.5).exp() + ((self.v - -10.0 - 40.0) / -59.0).exp()).powi(-1);
        let beta_ua = 0.65 * (2.5 + ((self.v - -10.0 + 72.0) / 17.0).exp()).powi(-1);
        let tau_ua = (alpha_ua + beta_ua).powi(-1) / c.k_q10;
        let ua_inf = (1.0 + ((self.v - -10.0 + 20.3) / -9.6).exp()).powi(-1);
        let dua = (ua_inf - self.ua) / tau_ua;
        
        let alpha_ui = (21.0 + 1.0 * ((self.v - -10.0 - 195.0) / -28.0).exp()).powi(-1);
        let beta_ui = 1.0 / ((self.v - -10.0 - 168.0) / -16.0).exp();
        let tau_ui = (alpha_ui + beta_ui).powi(-1) / c.k_q10;
        let ui_inf = (1.0 + ((self.v - -10.0 - 109.45) / 27.48).exp()).powi(-1);
        let dui = (ui_inf - self.ui) / tau_ui;
        
        let alpha_xr = if (self.v + 14.1).abs() < 1e-10 {
            0.0015
        } else {
            0.0003 * (self.v + 14.1) / (1.0 - ((self.v + 14.1) / -5.0).exp())
        };
        let beta_xr = if (self.v - 3.3328).abs() < 1e-10 {
            0.000378361
        } else {
            7.3898e-5 * (self.v - 3.3328) / (((self.v - 3.3328) / 5.1237).exp() - 1.0)
        };
        let tau_xr = (alpha_xr + beta_xr).powi(-1);
        let xr_inf = (1.0 + ((self.v + 14.1) / -6.5).exp()).powi(-1);
        let dxr = (xr_inf - self.xr) / tau_xr;
        
        let alpha_xs = if (self.v - 19.9).abs() < 1e-10 {
            0.00068
        } else {
            4e-5 * (self.v - 19.9) / (1.0 - ((self.v - 19.9) / -17.0).exp())
        };
        let beta_xs = if (self.v - 19.9).abs() < 1e-10 {
            0.000315
        } else {
            3.5e-5 * (self.v - 19.9) / (((self.v - 19.9) / 9.0).exp() - 1.0)
        };
        let tau_xs = 0.5 * (alpha_xs + beta_xs).powi(-1);
        let xs_inf = (1.0 + ((self.v - 19.9) / -12.7).exp()).powf(-0.5);
        let dxs = (xs_inf - self.xs) / tau_xs;
        
        let e_k = (c.r * c.t / c.f_faraday) * (p.effective_ko() / self.k_i).ln();
        let i_k1 = (c.cm * c.g_k1 * (self.v - e_k)) / (1.0 + (0.07 * (self.v + 80.0)).exp());
        let i_to = c.cm * c.g_to * self.oa.powi(3) * self.oi * (self.v - e_k) * p.block_k;
        let g_kur = 0.005 + 0.05 / (1.0 + ((self.v - 15.0) / -13.0).exp());
        let i_kur = c.cm * g_kur * self.ua.powi(3) * self.ui * (self.v - e_k) * p.block_k;
        let i_kr = (c.cm * c.g_kr * self.xr * (self.v - e_k)) / (1.0 + ((self.v + 15.0) / 22.4).exp()) * p.block_k;
        let i_ks = c.cm * c.g_ks * self.xs.powi(2) * (self.v - e_k) * p.block_k;
        
        let f_nak = (1.0 + 0.1245 * (-0.1 * c.f_faraday * self.v / (c.r * c.t)).exp() + 0.0365 * c.sigma * (-c.f_faraday * self.v / (c.r * c.t)).exp()).powi(-1);
        let i_nak = (c.cm * c.i_nak_max * f_nak / (1.0 + (c.km_na_i / self.na_i).powf(1.5))) * c.k_o / (c.k_o + c.km_k_o) * p.block_nak;
        let i_b_k = c.cm * c.g_b_k * (self.v - e_k);
        let dk_i = (2.0 * i_nak - (i_k1 + i_to + i_kur + i_kr + i_ks + i_b_k)) / (c.v_i * c.f_faraday);
        
        let e_na = (c.r * c.t / c.f_faraday) * (c.na_o / self.na_i).ln();
        let i_na = c.cm * c.g_na * self.m.powi(3) * self.h * self.j * (self.v - e_na) * p.block_na * p.isch_block();
        
        let exp_v = (c.gamma * c.f_faraday * self.v / (c.r * c.t)).exp();
        let exp_v_min1 = ((c.gamma - 1.0) * c.f_faraday * self.v / (c.r * c.t)).exp();
        
        let i_naca = (c.cm * c.i_naca_max * (exp_v * self.na_i.powi(3) * c.ca_o - exp_v_min1 * c.na_o.powi(3) * self.ca_i)) / ((c.k_mna.powi(3) + c.na_o.powi(3)) * (c.k_mca + c.ca_o) * (1.0 + c.k_sat * exp_v_min1));
        
        let i_b_na = c.cm * c.g_b_na * (self.v - e_na);
        let dna_i = (-3.0 * i_nak - (3.0 * i_naca + i_b_na + i_na)) / (c.v_i * c.f_faraday);
        
        let i_st = 0.0;
        
        let i_ca_l = c.cm * c.g_ca_l * self.d * self.f * self.f_ca * (self.v - 65.0) * p.block_ca * p.isch_block() * p.ans_ca_modifier();
        let i_cap = (c.cm * c.i_cap_max * self.ca_i) / (0.0005 + self.ca_i);
        let e_ca = (c.r * c.t / (2.0 * c.f_faraday)) * (c.ca_o / self.ca_i).ln();
        let i_b_ca = c.cm * c.g_b_ca * (self.v - e_ca);
        
        let dv = -(i_na + i_k1 + i_to + i_kur + i_kr + i_ks + i_b_na + i_b_ca + i_nak + i_cap + i_naca + i_ca_l + i_st) / c.cm;
        
        let i_rel = c.k_rel * self.u.powi(2) * self.v_gate * self.w * (self.ca_rel - self.ca_i);
        let i_tr = (self.ca_up - self.ca_rel) / c.tau_tr;
        
        let dca_rel = (i_tr - i_rel) * (1.0 + (c.csqn_max * c.km_csqn) / (self.ca_rel + c.km_csqn).powi(2)).powi(-1);
        
        let fn_val = 1000.0 * (1e-15 * c.v_rel * i_rel - (1e-15 / (2.0 * c.f_faraday)) * (0.5 * i_ca_l - 0.2 * i_naca));
        
        let u_inf = (1.0 + (-(fn_val - 3.4175e-13) / 1.367e-15).exp()).powi(-1);
        let du = (u_inf - self.u) / c.tau_u;
        
        let tau_v_gate = 1.91 + 2.09 * (1.0 + (-(fn_val - 3.4175e-13) / 1.367e-15).exp()).powi(-1);
        let v_inf = 1.0 - (1.0 + (-(fn_val - 6.835e-14) / 1.367e-15).exp()).powi(-1);
        let dv_gate = (v_inf - self.v_gate) / tau_v_gate;
        
        let i_up = c.i_up_max / (1.0 + c.k_up / self.ca_i);
        let i_up_leak = (c.i_up_max * self.ca_up) / c.ca_up_max;
        
        let dca_up = i_up - (i_up_leak + (i_tr * c.v_rel) / c.v_up);
        
        let b1 = (2.0 * i_naca - (i_cap + i_ca_l + i_b_ca)) / (2.0 * c.v_i * c.f_faraday) + (c.v_up * (i_up_leak - i_up) + i_rel * c.v_rel) / c.v_i;
        let b2 = 1.0 + (c.trpn_max * c.km_trpn) / (self.ca_i + c.km_trpn).powi(2) + (c.cmdn_max * c.km_cmdn) / (self.ca_i + c.km_cmdn).powi(2);
        
        let dca_i = b1 / b2;
        
        // Forward Euler updates
        self.v += dv * dt;
        self.na_i += dna_i * dt;
        self.m += dm * dt;
        self.h += dh * dt;
        self.j += dj * dt;
        self.k_i += dk_i * dt;
        self.oa += doa * dt;
        self.oi += doi * dt;
        self.ua += dua * dt;
        self.ui += dui * dt;
        self.xr += dxr * dt;
        self.xs += dxs * dt;
        self.ca_i += dca_i * dt;
        self.d += dd * dt;
        self.f += df * dt;
        self.f_ca += df_ca * dt;
        self.ca_rel += dca_rel * dt;
        self.u += du * dt;
        self.v_gate += dv_gate * dt;
        self.w += dw * dt;
        self.ca_up += dca_up * dt;
    }
}
