use crate::models::Pharmaco;

pub struct SeveriCell {
    pub v: f64,
    pub ca_sub: f64,
    pub nai: f64,
    pub y: f64,
    pub m: f64,
    pub h: f64,
    pub dl: f64,
    pub fl: f64,
    pub fca: f64,
    pub dt_gate: f64,
    pub ft_gate: f64,
    pub r_sr: f64,
    pub o_sr: f64,
    pub i_sr: f64,
    pub ri_sr: f64,
    pub ca_jsr: f64,
    pub ca_nsr: f64,
    pub cai: f64,
    pub ftmm: f64,
    pub fcmi: f64,
    pub fcms: f64,
    pub ftc: f64,
    pub ftmc: f64,
    pub fcq: f64,
    pub fbapta: f64,
    pub fbapta_sub: f64,
    pub q: f64,
    pub r: f64,
    pub pas: f64,
    pub paf: f64,
    pub piy: f64,
    pub n: f64,
    pub a: f64,
}

impl Default for SeveriCell {
    fn default() -> Self {
        Self {
            v: -52.0,
            ca_sub: 1e-5,
            nai: 7.5,
            y: 0.181334538702451,
            m: 0.440131579215766,
            h: 1.3676940140066e-5,
            dl: 0.0,
            fl: 0.497133507285601,
            fca: 0.697998543259722,
            dt_gate: 0.0,
            ft_gate: 0.0,
            r_sr: 0.912317231017262,
            o_sr: 1.7340201253e-7,
            i_sr: 7.86181717518e-8,
            ri_sr: 0.211148145512825,
            ca_jsr: 0.316762674605,
            ca_nsr: 1.05386465080816,
            cai: 1e-5,
            ftmm: 0.501049376634,
            fcmi: 0.0373817991524254,
            fcms: 0.054381370046,
            ftc: 0.0180519400676086,
            ftmc: 0.281244308217086,
            fcq: 0.299624275428735,
            fbapta: 0.0,
            fbapta_sub: 0.0,
            q: 0.506139850982478,
            r: 0.0144605370597924,
            pas: 0.322999177802891,
            paf: 0.0990510403258968,
            piy: 0.705410877258545,
            n: 0.0,
            a: 0.0,
        }
    }
}

impl SeveriCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compute_i_na(&self, p: &Pharmaco) -> f64 {
        let e_mh = 26.71 * ((p.nao + 0.12 * p.ko) / (self.nai + 0.12 * p.nao)).ln(); 
        let g_na = 0.0125 * p.block_na * p.isch_block();
        g_na * self.m.powi(3) * self.h * (self.v - e_mh)
    }

    pub fn compute_i_cal(&self, p: &Pharmaco, ach_block: f64, c_sev_93: f64) -> f64 {
        let p_cal = 0.2 * p.block_ca * p.isch_block() * (1.0 - ach_block) * c_sev_93;
        let v_norm = self.v / 26.71;
        let exp_2v = (-2.0 * v_norm).exp();
        let exp_v = (-v_norm).exp();
        
        let i_sica = (2.0 * p_cal * self.v / (26.71 * (1.0 - exp_2v))) 
            * (self.ca_sub - p.cao * exp_2v) * self.dl * self.fl * self.fca;
        
        let i_sika = (0.000365 * p_cal * self.v / (26.71 * (1.0 - exp_v))) 
            * (140.0 - p.ko * exp_v) * self.dl * self.fl * self.fca;
            
        let i_sina = (1.85e-5 * p_cal * self.v / (26.71 * (1.0 - exp_v))) 
            * (self.nai - p.nao * exp_v) * self.dl * self.fl * self.fca;
            
        i_sica + i_sika + i_sina
    }

    pub fn compute_i_kr(&self, p: &Pharmaco) -> f64 {
        let g_kr = 0.0021637 * p.block_k;
        let e_k = 26.71 * (p.effective_ko() / 140.0_f64).ln();
        g_kr * (self.v - e_k) * (0.9 * self.paf + 0.1 * self.pas) * self.piy
    }
    
    pub fn compute_i_ks(&self, p: &Pharmaco, c_sev_86: f64) -> f64 {
        let g_ks = c_sev_86 * p.block_k;
        let e_ks = 26.71 * ((p.effective_ko() + 0.0) / (140.0 + 0.0_f64)).ln();
        g_ks * (self.v - e_ks) * self.n.powi(2)
    }

    pub fn compute_i_to(&self, p: &Pharmaco) -> f64 {
        let g_to = 0.002;
        let e_k = 26.71 * (p.effective_ko() / 140.0_f64).ln();
        g_to * (self.v - e_k) * self.q * self.r
    }
    
    pub fn compute_i_f(&self, p: &Pharmaco) -> f64 {
        let e_k = 26.71 * (p.effective_ko() / 140.0_f64).ln();
        let e_na = 26.71 * (p.nao / self.nai).ln();
        let g_f_na = 0.03;
        let g_f_k = 0.03;
        
        let i_f_na = g_f_na * self.y.powi(2) * (self.v - e_na) * p.ko / (p.ko + 45.0);
        let i_f_k = g_f_k * self.y.powi(2) * (self.v - e_k) * p.ko / (p.ko + 45.0);
        i_f_na + i_f_k
    }

    pub fn step(&mut self, dt: f64, p: &Pharmaco) {

        let iso = p.symp;
        let ach = p.parasymp * 1e-3;

        let eff_iso_84 = iso * -0.25;
        let eff_ach_84 = if ach > 0.0 { (0.7 * ach) / (9e-5 + ach) } else { 0.0 };
        let c_sev_84 = eff_iso_84 + eff_ach_84;
        let b_up = 0.0006 * (1.0 - c_sev_84);

        let c_sev_86 = 0.0016576 * (1.0 + 0.2 * iso); // g_ks
        let c_sev_89 = if ach > 0.0 { -1.0 - (9.898 * ach.powf(0.618)) / (ach.powf(0.618) + 0.00122423) } else { 0.0 };
        let c_sev_90 = iso * 7.5;
        let c_sev_91 = 1.0 + 0.2 * iso; // i_nak multiplier
        let c_sev_93 = 1.0 + 0.23 * iso; // i_cal multiplier
        let c_sev_94 = (0.31 * ach) / (ach + 9e-5); // ach block for i_cal
        let c_sev_95 = iso * -8.0; // iso shift for dl gate
        let c_sev_96 = 1.0 - 0.31 * iso; // iso slope for dl gate
        let c_sev_102 = iso * -14.0; // iso shift for n gate
        let alpha_a_val = if ach > 0.0 { (3.59880 - 0.0256410) / (1.0 + 1.21550e-06 / ach.powf(1.69510)) + 0.0256410 } else { 0.0256410 };

        let e_na = 26.71 * (p.nao / self.nai).ln();
        let e_k = 26.71 * (p.effective_ko() / 140.0_f64).ln();

        // Constants for volumes
        let v_cell = 1e-9 * std::f64::consts::PI * 16.0 * 70.0;
        let v_sub = 1e-9 * 2.0 * std::f64::consts::PI * 0.02 * (4.0 - 0.01) * 70.0;
        let v_jsr = 0.0012 * v_cell;
        let v_i = 0.46 * v_cell - v_sub;
        let v_nsr = 0.0116 * v_cell;

        // NaK Current
        let i_nak = 0.063 * c_sev_91 * p.block_nak * (1.0 + (1.4 / p.ko).powf(1.2)).recip() * (1.0 + (14.0 / self.nai).powf(1.3)).recip() * (1.0 + (-(self.v - e_na) + 110.0) / 20.0).exp().recip();

        // NaCa Current
        let d_naca = 1.0 + (p.cao / 3.663) * (1.0 + (0.0 * self.v / 26.71).exp()) + (p.nao / 1628.0) * (1.0 + (p.nao / 561.4) * (1.0 + p.nao / 4.663));
        let k34 = p.nao / (4.663 + p.nao);
        let k23 = (p.nao / 1628.0 * p.nao / 561.4) * (1.0 + p.nao / 4.663) * (-0.4315 * self.v / (2.0 * 26.71)).exp() / d_naca;
        let k21 = (p.cao / 3.663) * (0.0 * self.v / 26.71).exp() / d_naca;
        let k32 = (0.4315 * self.v / (2.0 * 26.71)).exp();
        let k43 = self.nai / (26.44 + self.nai);
        let k41 = (-0.4315 * self.v / (2.0 * 26.71)).exp();
        let d_i = 1.0 + (self.ca_sub / 0.0207) * (1.0 + (-0.1369 * self.v / 26.71).exp() + self.nai / 26.44) + (self.nai / 395.3) * (1.0 + (self.nai / 2.289) * (1.0 + self.nai / 26.44));
        let k12 = (self.ca_sub / 0.0207) * (-0.1369 * self.v / 26.71).exp() / d_i;
        let k14 = ((self.nai / 395.3 * self.nai) / 2.289) * (1.0 + self.nai / 26.44) * (0.4315 * self.v / (2.0 * 26.71)).exp() / d_i;
        
        let x1 = k41 * k34 * (k23 + k21) + k21 * k32 * (k43 + k41);
        let x2 = k32 * k43 * (k14 + k12) + k41 * k12 * (k34 + k32);
        let x3 = k14 * k43 * (k23 + k21) + k12 * k23 * (k43 + k41);
        let x4 = k23 * k34 * (k14 + k12) + k14 * k21 * (k34 + k32);
        let i_naca = 4.0 * (x2 * k21 - x1 * k12) / (x1 + x2 + x3 + x4);

        // CaT Current
        let i_cat = (2.0 * 0.02 * self.v / (26.71 * (1.0 - (-2.0 * self.v / 26.71).exp()))) * (self.ca_sub - p.cao * (-2.0 * self.v / 26.71).exp()) * self.dt_gate * self.ft_gate;

        // KACh Current
        let g_kach = 0.00864;
        let i_kach = if ach > 0.0 { g_kach * (self.v - e_k) * (1.0 + ((self.v + 20.0) / 20.0).exp()) * self.a } else { 0.0 };

        // Other Currents
        let i_na = self.compute_i_na(p);
        let i_cal = self.compute_i_cal(p, c_sev_94, c_sev_93);
        let i_kr = self.compute_i_kr(p);
        let i_ks = self.compute_i_ks(p, c_sev_86);
        let i_to = self.compute_i_to(p);
        let i_f = self.compute_i_f(p);
        
        // Components of i_cal, i_f for Ca and Na dynamics
        let p_cal = 0.2 * p.block_ca * p.isch_block() * (1.0 - c_sev_94) * c_sev_93;
        let i_sica = (2.0 * p_cal * self.v / (26.71 * (1.0 - (-2.0 * self.v / 26.71).exp()))) * (self.ca_sub - p.cao * (-2.0 * self.v / 26.71).exp()) * self.dl * self.fl * self.fca;
        let i_sina = (1.85e-5 * p_cal * self.v / (26.71 * (1.0 - (-self.v / 26.71).exp()))) * (self.nai - p.nao * (-self.v / 26.71).exp()) * self.dl * self.fl * self.fca;
        let i_f_na = 0.03 * self.y.powi(2) * (self.v - e_na) * p.ko / (p.ko + 45.0);

        // Sum total current 
        let i_tot = i_na + i_cal + i_kr + i_ks + i_to + i_f + i_nak + i_naca + i_cat + i_kach;
        
        // Update membrane potential
        let c_m = 3.2e-5;
        self.v += (-i_tot / c_m) * dt;

        // i_f y gate
        let y_inf = 1.0 / (1.0 + (((self.v + 52.5) - c_sev_89 - c_sev_90) / 9.0).exp());
        let tau_y = 0.716653 / (0.0708 * (-((self.v + 5.0) - c_sev_89 - c_sev_90) / 20.2791).exp() + 10.6 * (((self.v - c_sev_89) - c_sev_90) / 18.0).exp());
        self.y += ((y_inf - self.y) / tau_y) * dt;
        
        // i_Na m & h gates
        let v_shift = self.v + 41.0;
        let alpha_m = if v_shift.abs() < 1e-5 { 2000.0 } else { (200.0 * v_shift) / (1.0 - (-0.1 * v_shift).exp()) };
        let beta_m = 8000.0 * (-0.056 * (self.v + 66.0)).exp();
        self.m += (alpha_m * (1.0 - self.m) - beta_m * self.m) * dt;

        let alpha_h = 20.0 * (-0.125 * (self.v + 75.0)).exp();
        let beta_h = 2000.0 / (320.0 * (-0.1 * (self.v + 75.0)).exp() + 1.0);
        self.h += (alpha_h * (1.0 - self.h) - beta_h * self.h) * dt;

        // i_Kr gates
        let pa_inf = 1.0 / (1.0 + (-(self.v + 14.8) / 8.5).exp());
        let tau_pas = 0.846554 / (4.2 * (self.v / 17.0).exp() + 0.15 * (-self.v / 21.6).exp());
        let tau_paf = 1.0 / (30.0 * (self.v / 10.0).exp() + (-self.v / 12.0).exp());
        self.pas += ((pa_inf - self.pas) / tau_pas) * dt;
        self.paf += ((pa_inf - self.paf) / tau_paf) * dt;
        
        let pi_inf = 1.0 / (1.0 + ((self.v + 28.6) / 17.1).exp());
        let tau_pi = 1.0 / (100.0 * (-self.v / 54.645).exp() + 656.0 * (self.v / 106.157).exp());
        self.piy += ((pi_inf - self.piy) / tau_pi) * dt;

        // i_Ks n gate
        let v_shifted_n = self.v - c_sev_102;
        let n_inf_term = 14.0 / (1.0 + (-(v_shifted_n - 40.0) / 12.0).exp());
        let n_inf = n_inf_term / (n_inf_term + (-v_shifted_n / 45.0).exp());
        let alpha_n = 28.0 / (1.0 + (-(v_shifted_n - 40.0) / 3.0).exp());
        let beta_n = (-(v_shifted_n - 5.0) / 25.0).exp();
        let tau_n = 1.0 / (alpha_n + beta_n);
        self.n += ((n_inf - self.n) / tau_n) * dt;
        
        // i_to q & r gates
        let q_inf = 1.0 / (1.0 + ((self.v + 49.0) / 13.0).exp());
        let tau_q = 0.001 * 0.6 * (65.17 / (0.57 * (-0.08 * (self.v + 44.0)).exp() + 0.065 * (0.1 * (self.v + 45.93)).exp()) + 10.1);
        self.q += ((q_inf - self.q) / tau_q) * dt;

        let r_inf = 1.0 / (1.0 + (-(self.v - 19.3) / 15.0).exp());
        let tau_r = 0.001 * 0.66 * 1.4 * (15.59 / (1.037 * (0.09 * (self.v + 30.61)).exp() + 0.369 * (-0.12 * (self.v + 23.84)).exp()) + 2.98);
        self.r += ((r_inf - self.r) / tau_r) * dt;

        // i_KACh a gate
        let beta_a = 10.0 * (0.0133 * (self.v + 40.0)).exp();
        let alpha_a = alpha_a_val;
        let a_inf = alpha_a / (alpha_a + beta_a);
        let tau_a = 1.0 / (alpha_a + beta_a);
        self.a += ((a_inf - self.a) / tau_a) * dt;

        // i_CaL gates
        let fca_inf = 0.00035 / (0.00035 + self.ca_sub);
        let tau_fca = (0.001 * fca_inf) / 0.01;
        self.fca += ((fca_inf - self.fca) / tau_fca) * dt;

        let fl_inf = 1.0 / (1.0 + ((self.v + 37.4) / 5.3).exp());
        let tau_fl = 0.001 * (44.3 + 230.0 * (-((self.v + 36.0) / 10.0).powi(2)).exp());
        self.fl += ((fl_inf - self.fl) / tau_fl) * dt;

        let dl_inf = 1.0 / (1.0 + (-(self.v + 20.3 - c_sev_95) / (c_sev_96 * 4.2)).exp());
        let v_dl_1 = if (self.v + 41.8).abs() < 1e-5 { -41.80001 } else if (self.v + 6.8).abs() < 1e-5 { -6.80001 } else { self.v };
        let alpha_dl = (-0.02839 * (v_dl_1 + 41.8 - c_sev_95)) / ((-(v_dl_1 + 41.8 - c_sev_95) / 2.5).exp() - 1.0) 
                     - (0.0849 * (v_dl_1 + 6.8 - c_sev_95)) / ((-(v_dl_1 + 6.8 - c_sev_95) / 4.8).exp() - 1.0);
        let v_dl_2 = if (self.v + 1.8).abs() < 1e-5 { -1.80001 } else { self.v };
        let beta_dl = (0.01143 * (v_dl_2 + 1.8 - c_sev_95)) / (((v_dl_2 + 1.8 - c_sev_95) / 2.5).exp() - 1.0);
        let tau_dl = 0.001 / (alpha_dl + beta_dl);
        self.dl += ((dl_inf - self.dl) / tau_dl) * dt;

        // i_CaT gates
        let dt_inf = 1.0 / (1.0 + (-(self.v + 38.3) / 5.5).exp());
        let tau_dt = 0.001 / (1.068 * ((self.v + 38.3) / 30.0).exp() + 1.068 * (-(self.v + 38.3) / 30.0).exp());
        self.dt_gate += ((dt_inf - self.dt_gate) / tau_dt) * dt;

        let ft_inf = 1.0 / (1.0 + ((self.v + 58.7) / 3.8).exp());
        let tau_ft = 1.0 / (16.67 * (-(self.v + 75.0) / 83.3).exp() + 16.67 * ((self.v + 75.0) / 15.38).exp());
        self.ft_gate += ((ft_inf - self.ft_gate) / tau_ft) * dt;

        // SR release gates
        let kcasr = 15.0 - (15.0 - 1.0) / (1.0 + (0.45 / self.ca_jsr).powf(2.5));
        let kosrca = 10000.0 / kcasr;
        let kisrca = 500.0 * kcasr;

        let d_r_sr = (5.0 * self.ri_sr - kisrca * self.ca_sub * self.r_sr) - (kosrca * self.ca_sub.powi(2) * self.r_sr - 60.0 * self.o_sr);
        let d_o_sr = (kosrca * self.ca_sub.powi(2) * self.r_sr - 60.0 * self.o_sr) - (kisrca * self.ca_sub * self.o_sr - 5.0 * self.i_sr);
        let d_i_sr = (kisrca * self.ca_sub * self.o_sr - 5.0 * self.i_sr) - (60.0 * self.i_sr - kosrca * self.ca_sub.powi(2) * self.ri_sr);
        let d_ri_sr = (60.0 * self.i_sr - kosrca * self.ca_sub.powi(2) * self.ri_sr) - (5.0 * self.ri_sr - kisrca * self.ca_sub * self.r_sr);

        self.r_sr += d_r_sr * dt;
        self.o_sr += d_o_sr * dt;
        self.i_sr += d_i_sr * dt;
        self.ri_sr += d_ri_sr * dt;

        // Ca buffering rates
        let d_ftmm = 2277.0 * 2.5 * (1.0 - (self.ftmc + self.ftmm)) - 751.0 * self.ftmm;
        let d_ftc = 88800.0 * self.cai * (1.0 - self.ftc) - 446.0 * self.ftc;
        let d_ftmc = 227700.0 * self.cai * (1.0 - (self.ftmc + self.ftmm)) - 7.51 * self.ftmc;
        let d_fcq = 534.0 * self.ca_jsr * (1.0 - self.fcq) - 445.0 * self.fcq;
        let d_fcmi = 227700.0 * self.cai * (1.0 - self.fcmi) - 542.0 * self.fcmi;
        let d_fcms = 227700.0 * self.ca_sub * (1.0 - self.fcms) - 542.0 * self.fcms;

        self.ftmm += d_ftmm * dt;
        self.ftc += d_ftc * dt;
        self.ftmc += d_ftmc * dt;
        self.fcq += d_fcq * dt;
        self.fcmi += d_fcmi * dt;
        self.fcms += d_fcms * dt;

        // Ca fluxes
        let j_up = 12.0 / (1.0 + b_up / self.cai);
        let j_tr = (self.ca_nsr - self.ca_jsr) / 0.04;
        let j_srcarel = 250000000.0 * self.o_sr * (self.ca_jsr - self.ca_sub);
        let j_ca_dif = (self.ca_sub - self.cai) / 4e-5;

        // Na and Ca concentrations
        let d_nai = -(i_na + i_f_na + i_sina + 3.0 * i_nak + 3.0 * i_naca) / ((v_i + v_sub) * 96485.3415);
        let d_ca_nsr = j_up - j_tr * v_jsr / v_nsr;
        let d_ca_jsr = j_tr - (j_srcarel + 10.0 * d_fcq);
        let d_cai = (j_ca_dif * v_sub - j_up * v_nsr) / v_i - (0.045 * d_fcmi + 0.031 * d_ftc + 0.062 * d_ftmc);
        let d_ca_sub = j_srcarel * v_jsr / v_sub - (i_sica + i_cat - 2.0 * i_naca) / (2.0 * 96485.3415 * v_sub) - j_ca_dif - 0.045 * d_fcms;

        self.nai += d_nai * dt;
        self.ca_nsr += d_ca_nsr * dt;
        self.ca_jsr += d_ca_jsr * dt;
        self.cai += d_cai * dt;
        self.ca_sub += d_ca_sub * dt;
    }
}
