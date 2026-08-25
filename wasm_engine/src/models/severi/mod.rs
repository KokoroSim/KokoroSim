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

    pub fn compute_i_na(&self) -> f64 {
        let e_mh = 26.71 * ((140.0 + 0.12 * 5.4) / (self.nai + 0.12 * 140.0)).ln(); 
        let g_na = 0.0125;
        g_na * self.m.powi(3) * self.h * (self.v - e_mh)
    }

    pub fn compute_i_cal(&self) -> f64 {
        let p_cal = 0.2;
        let v_norm = self.v / 26.71;
        let exp_2v = (-2.0 * v_norm).exp();
        let i_sica = (2.0 * p_cal * self.v / (26.71 * (1.0 - exp_2v))) 
            * (self.ca_sub - 1.8 * exp_2v) * self.dl * self.fl * self.fca;
        
        let i_sika = (0.000365 * p_cal * self.v / (26.71 * (1.0 - (-v_norm).exp()))) 
            * (140.0 - 5.4 * (-v_norm).exp()) * self.dl * self.fl * self.fca;
            
        i_sica + i_sika
    }

    pub fn compute_i_kr(&self) -> f64 {
        let g_kr = 0.0021637;
        let e_k = 26.71 * (5.4 / 140.0_f64).ln();
        g_kr * (self.v - e_k) * (0.9 * self.paf + 0.1 * self.pas) * self.piy
    }
    
    pub fn compute_i_ks(&self) -> f64 {
        let g_ks = 0.0016576;
        let e_ks = 26.71 * ((5.4 + 0.0) / (140.0 + 0.0_f64)).ln();
        g_ks * (self.v - e_ks) * self.n.powi(2)
    }

    pub fn compute_i_to(&self) -> f64 {
        let g_to = 0.002;
        let e_k = 26.71 * (5.4 / 140.0_f64).ln();
        g_to * (self.v - e_k) * self.q * self.r
    }
    
    pub fn compute_i_f(&self) -> f64 {
        let e_k = 26.71 * (5.4 / 140.0_f64).ln();
        let e_na = 26.71 * (140.0 / self.nai).ln();
        let g_f_na = 0.03;
        let g_f_k = 0.03;
        
        let i_f_na = g_f_na * self.y.powi(2) * (self.v - e_na) * 5.4 / (5.4 + 45.0);
        let i_f_k = g_f_k * self.y.powi(2) * (self.v - e_k) * 5.4 / (5.4 + 45.0);
        i_f_na + i_f_k
    }

    pub fn step(&mut self, dt: f64) {
        // Currents
        let i_na = self.compute_i_na();
        let i_cal = self.compute_i_cal();
        let i_kr = self.compute_i_kr();
        let i_ks = self.compute_i_ks();
        let i_to = self.compute_i_to();
        let i_f = self.compute_i_f();
        
        // Sum total current 
        let i_tot = i_na + i_cal + i_kr + i_ks + i_to + i_f;
        
        // Update membrane potential
        let c_m = 3.2e-5;
        self.v += (-i_tot / c_m) * dt;
        
        // i_f y gate
        let y_inf = 1.0 / (1.0 + ((self.v + 52.5) / 9.0).exp());
        let tau_y = 0.716653 / (0.0708 * (-(self.v + 5.0) / 20.2791).exp() + 10.6 * (self.v / 18.0).exp());
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
        let n_inf_term = 14.0 / (1.0 + (-(self.v - 40.0) / 12.0).exp());
        let n_inf = n_inf_term / (n_inf_term + (-self.v / 45.0).exp());
        let alpha_n = 28.0 / (1.0 + (-(self.v - 40.0) / 3.0).exp());
        let beta_n = (-(self.v - 5.0) / 25.0).exp();
        let tau_n = 1.0 / (alpha_n + beta_n);
        self.n += ((n_inf - self.n) / tau_n) * dt;
        
        // i_to q & r gates
        let q_inf = 1.0 / (1.0 + ((self.v + 49.0) / 13.0).exp());
        let tau_q = 0.001 * 0.6 * (65.17 / (0.57 * (-0.08 * (self.v + 44.0)).exp() + 0.065 * (0.1 * (self.v + 45.93)).exp()) + 10.1);
        self.q += ((q_inf - self.q) / tau_q) * dt;

        let r_inf = 1.0 / (1.0 + (-(self.v - 19.3) / 15.0).exp());
        let tau_r = 0.001 * 0.66 * 1.4 * (15.59 / (1.037 * (0.09 * (self.v + 30.61)).exp() + 0.369 * (-0.12 * (self.v + 23.84)).exp()) + 2.98);
        self.r += ((r_inf - self.r) / tau_r) * dt;
        
        // More updates can be appended here for Calcium dynamics, NaK, NaCa, etc.
    }
}
