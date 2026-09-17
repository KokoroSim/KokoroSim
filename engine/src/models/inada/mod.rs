pub struct InadaCell {
    pub v: f64,
    pub y_gate: f64,
    pub paf_gate: f64,
    pub pas_gate: f64,
    pub pik_gate: f64,
    pub casub: f64,
    pub m_gate: f64,
    pub h1_gate: f64,
    pub h2_gate: f64,
    pub d_gate: f64,
    pub f_gate: f64,
    pub f2: f64,
    pub r_gate: f64,
    pub q_fast: f64,
    pub q_slow: f64,
    pub qa_gate: f64,
    pub qi_gate: f64,
    pub achf_gate: f64,
    pub achs_gate: f64,
    pub cai: f64,
    pub ca_up: f64,
    pub ca_rel: f64,
    pub f_tc: f64,
    pub f_tmc: f64,
    pub f_tmm: f64,
    pub f_cmi: f64,
    pub f_cms: f64,
    pub f_cq: f64,
    pub f_csl: f64,
    pub r_gas: f64,
    pub t: f64,
    pub f_faraday: f64,
    pub c: f64,
    pub g_f: f64,
    pub ach: f64,
    pub g_kr: f64,
    pub ki: f64,
    pub kc: f64,
    pub g_k1: f64,
    pub g_b: f64,
    pub e_b: f64,
    pub i_p: f64,
    pub nai: f64,
    pub knaca: f64,
    pub qci: f64,
    pub qn: f64,
    pub qco: f64,
    pub kci: f64,
    pub k1ni: f64,
    pub k2ni: f64,
    pub k3ni: f64,
    pub kcni: f64,
    pub k3no: f64,
    pub k1no: f64,
    pub k2no: f64,
    pub kco: f64,
    pub cao: f64,
    pub nao: f64,
    pub g_na: f64,
    pub delta_m: f64,
    pub g_cal: f64,
    pub e_cal: f64,
    pub act_shift: f64,
    pub slope_factor_act: f64,
    pub inact_shift: f64,
    pub inact_shift_f2: f64,
    pub g_to: f64,
    pub e_st: f64,
    pub g_st: f64,
    pub g_ach_max: f64,
    pub k_ach: f64,
    pub alpha_achf: f64,
    pub alpha_achs: f64,
    pub v_cell: f64,
    pub p_rel: f64,
    pub k_up: f64,
    pub tau_tr: f64,
    pub rtonf: f64,
    pub k34: f64,
    pub e_k_kr: f64,
    pub e_na: f64,
    pub e_k_to: f64,
    pub k43: f64,
    pub v_up: f64,
    pub v_rel: f64,
    pub v_sub: f64,
    pub vi: f64,
}

impl Default for InadaCell {
    fn default() -> Self {
        let mut cell = Self {
            v: 0.0,
            y_gate: 0.0,
            paf_gate: 0.0,
            pas_gate: 0.0,
            pik_gate: 0.0,
            casub: 0.0,
            m_gate: 0.0,
            h1_gate: 0.0,
            h2_gate: 0.0,
            d_gate: 0.0,
            f_gate: 0.0,
            f2: 0.0,
            r_gate: 0.0,
            q_fast: 0.0,
            q_slow: 0.0,
            qa_gate: 0.0,
            qi_gate: 0.0,
            achf_gate: 0.0,
            achs_gate: 0.0,
            cai: 0.0,
            ca_up: 0.0,
            ca_rel: 0.0,
            f_tc: 0.0,
            f_tmc: 0.0,
            f_tmm: 0.0,
            f_cmi: 0.0,
            f_cms: 0.0,
            f_cq: 0.0,
            f_csl: 0.0,
            r_gas: 0.0,
            t: 0.0,
            f_faraday: 0.0,
            c: 0.0,
            g_f: 0.0,
            ach: 0.0,
            g_kr: 0.0,
            ki: 0.0,
            kc: 0.0,
            g_k1: 0.0,
            g_b: 0.0,
            e_b: 0.0,
            i_p: 0.0,
            nai: 0.0,
            knaca: 0.0,
            qci: 0.0,
            qn: 0.0,
            qco: 0.0,
            kci: 0.0,
            k1ni: 0.0,
            k2ni: 0.0,
            k3ni: 0.0,
            kcni: 0.0,
            k3no: 0.0,
            k1no: 0.0,
            k2no: 0.0,
            kco: 0.0,
            cao: 0.0,
            nao: 0.0,
            g_na: 0.0,
            delta_m: 0.0,
            g_cal: 0.0,
            e_cal: 0.0,
            act_shift: 0.0,
            slope_factor_act: 0.0,
            inact_shift: 0.0,
            inact_shift_f2: 0.0,
            g_to: 0.0,
            e_st: 0.0,
            g_st: 0.0,
            g_ach_max: 0.0,
            k_ach: 0.0,
            alpha_achf: 0.0,
            alpha_achs: 0.0,
            v_cell: 0.0,
            p_rel: 0.0,
            k_up: 0.0,
            tau_tr: 0.0,
            rtonf: 0.0,
            k34: 0.0,
            e_k_kr: 0.0,
            e_na: 0.0,
            e_k_to: 0.0,
            k43: 0.0,
            v_up: 0.0,
            v_rel: 0.0,
            v_sub: 0.0,
            vi: 0.0,
        };
        cell.v = -49.7094187908202;
        cell.r_gas = 8314.472;
        cell.t = 310.0;
        cell.f_faraday = 96485.3415;
        cell.c = 2.9e-5;
        cell.g_f = 0.001 * 0.15; // v1: Mantém apenas 15% da Corrente Funny original
        cell.ach = 0.0;
        cell.y_gate = 0.0462303183096481;
        cell.g_kr = 0.0035;
        cell.ki = 140.0;
        cell.kc = 5.4;
        cell.paf_gate = 0.192515363116553;
        cell.pas_gate = 0.0797182955833868;
        cell.pik_gate = 0.949023698965401;
        cell.g_k1 = 0.0;
        cell.g_b = 0.0012 * 0.45; // v1: Sweet-spot: 45% da Corrente de Fuga cria a rampa sem escapar
        cell.e_b = -22.5;
        cell.i_p = 0.14268;
        cell.nai = 8.0;
        cell.knaca = 2.14455;
        cell.qci = 0.1369;
        cell.qn = 0.4315;
        cell.qco = 0.0;
        cell.kci = 0.0207;
        cell.k1ni = 395.3;
        cell.k2ni = 2.289;
        cell.k3ni = 26.44;
        cell.kcni = 26.44;
        cell.k3no = 4.663;
        cell.k1no = 1628.0;
        cell.k2no = 561.4;
        cell.kco = 3.663;
        cell.cao = 2.0;
        cell.nao = 140.0;
        cell.casub = 0.000160310601192365;
        cell.g_na = 0.0;
        cell.m_gate = 0.143642247226618;
        cell.h1_gate = 0.0243210273637729;
        cell.h2_gate = 0.0157156121147801;
        cell.delta_m = 1e-5;
        cell.g_cal = 0.009;
        cell.e_cal = 62.0;
        cell.d_gate = 0.00179250298710316;
        cell.f_gate = 0.975550840189597;
        cell.f2 = 0.774394220125623;
        cell.act_shift = -15.0;
        cell.slope_factor_act = -5.0;
        cell.inact_shift = -5.0;
        cell.inact_shift_f2 = -5.0;
        cell.g_to = 0.0;
        cell.r_gate = 0.0296516611999521;
        cell.q_fast = 0.899732315818241;
        cell.q_slow = 0.190111737767474;
        cell.e_st = -37.4;
        cell.g_st = 0.0001;
        cell.qa_gate = 0.476404610622697;
        cell.qi_gate = 0.542303657353244;
        cell.g_ach_max = 0.0198;
        cell.k_ach = 0.00035;
        cell.achf_gate = 0.550559577208797;
        cell.achs_gate = 0.567277036232041;
        cell.alpha_achf = 73.1;
        cell.alpha_achs = 3.7;
        cell.cai = 0.000184969821581882;
        cell.v_cell = 3.18872e-6;
        cell.ca_up = 1.11092514657408;
        cell.ca_rel = 0.296249516481577;
        cell.p_rel = 1500.0;
        cell.k_up = 0.0006;
        cell.tau_tr = 0.06;
        cell.f_tc = 0.0356473236675985;
        cell.f_tmc = 0.443317425115817;
        cell.f_tmm = 0.491718960234865;
        cell.f_cmi = 0.0723007987059414;
        cell.f_cms = 0.0630771339141488;
        cell.f_cq = 0.261430602900137;
        cell.f_csl = 4.1497704886823e-5;
        cell.rtonf = ( cell.r_gas*cell.t)/cell.f_faraday;
        cell.k34 = cell.nao/(cell.k3no+cell.nao);
        cell.e_k_kr = cell.rtonf*f64::ln(cell.kc/cell.ki);
        cell.e_na = cell.rtonf*f64::ln(cell.nao/cell.nai);
        cell.e_k_to = cell.rtonf*f64::ln(cell.kc/cell.ki);
        cell.k43 = cell.nai/(cell.k3ni+cell.nai);
        cell.v_up = 0.0116000*cell.v_cell;
        cell.v_rel = 0.00120000*cell.v_cell;
        cell.v_sub = 0.0100000*cell.v_cell;
        cell.vi = 0.460000*cell.v_cell - cell.v_sub;
        cell
    }
}

impl InadaCell {
    pub fn step(&mut self, dt: f64, p: &crate::models::Pharmaco) {
        let e_k = self.rtonf * (p.effective_ko() / self.ki).ln();
        let e_na = self.rtonf * (p.nao / self.nai).ln();
        let a_15 = 120.000/(1.00000+(- (self.v+50.0000)/15.0000).exp());
        let r_17 =  self.alpha_achf*(1.00000 - self.achf_gate) -  a_15*self.achf_gate;
        let a_16 = 5.82000/(1.00000+(- (self.v+50.0000)/15.0000).exp());
        let r_18 =  self.alpha_achs*(1.00000 - self.achs_gate) -  a_16*self.achs_gate;
        let a_18 =  2277.00*2.50000*((1.00000 - self.f_tmc) - self.f_tmm) -  751.000*self.f_tmm;
        let r_24 = a_18;
        let a_0 = 1.00000/(1.00000+(((self.v+83.1900) - ( - 7.20000*(self.ach).powf(0.690000))/((1.26000e-05_f64).powf(0.690000)+(self.ach).powf(0.690000)))/13.5600).exp());
        let a_19 = 0.250000+ 2.00000*(- (self.v+70.0000).powf(2.00000)/500.000).exp();
        let r_1 = (a_0 - self.y_gate)/a_19;
        let a_1 = 1.00000/(1.00000+((self.v+10.2200)/- 8.50000).exp());
        let a_20 = 1.00000/( 17.0000*( 0.0398000*self.v).exp()+ 0.211000*( - 0.0510000*self.v).exp());
        let r_2 = (a_1 - self.paf_gate)/a_20;
        let a_2 = 1.00000/(1.00000+((self.v+10.2200)/- 8.50000).exp());
        let a_21 = 0.335810+ 0.906730*(- (self.v+10.0000).powf(2.00000)/988.050).exp();
        let r_3 = (a_2 - self.pas_gate)/a_21;
        let a_8 = 1.00000/(1.00000+((self.v - (- 24.0000+self.inact_shift))/6.31000).exp());
        let a_27 = 0.0100000+ 0.153900*(- (self.v+40.0000).powf(2.00000)/185.670).exp();
        let r_10 = (a_8 - self.f_gate)/a_27;
        let a_9 = 1.00000/(1.00000+((self.v - (- 24.0000+self.inact_shift_f2))/6.31000).exp());
        let a_28 = 0.0600000+ 0.480760*2.25000*(- (self.v - - 40.0000).powf(2.00000)/138.040).exp();
        let r_11 = (a_9 - self.f2)/a_28;
        let a_29 = 0.000596000+0.00311800/( 1.03700*( 0.0900000*(self.v+30.6100)).exp()+ 0.396000*( - 0.120000*(self.v+23.8400)).exp());
        let a_10 = 1.00000/(1.00000+((self.v - 7.44000)/- 16.4000).exp());
        let r_12 = (a_10 - self.r_gate)/a_29;
        let a_30 = 0.0126600+4.72716/(1.00000+((self.v+154.500)/23.9600).exp());
        let a_11 = 1.00000/(1.00000+((self.v+33.8000)/6.12000).exp());
        let r_13 = (a_11 - self.q_fast)/a_30;
        let a_31 = 0.100000+ 4.00000*(- (self.v+65.0000).powf(2.00000)/500.000).exp();
        let a_12 = 1.00000/(1.00000+((self.v+33.8000)/6.12000).exp());
        let r_14 = (a_12 - self.q_slow)/a_31;
        let a_4 = self.v+44.4000;
        let a_23 = if (a_4).abs() < self.delta_m {
            (-460.0 * -12.6730) / (a_4 / -12.6730).exp()
        } else {
            (-460.0 * a_4) / ((a_4 / -12.6730).exp() - 1.0)
        };
        let a_36 =  18400.0*(a_4/- 12.6730).exp();
        let r_6 =  a_23*(1.00000 - self.m_gate) -  a_36*self.m_gate;
        let a_3 =  (1.00000/(1.00000+((self.v+4.90000)/15.1400).exp()))*(1.00000 -  0.300000*(- (self.v).powf(2.00000)/500.000).exp());
        let a_22 =  92.0100*( - 0.0183000*self.v).exp();
        let a_35 =  603.600*( 0.00942000*self.v).exp();
        let a_43 = 1.00000/(a_22+a_35);
        let r_4 = (a_3 - self.pik_gate)/a_43;
        let a_5 =  44.9000*((self.v+66.9000)/- 5.57000).exp();
        let a_24 = 1491.00/(1.00000+ 323.300*((self.v+94.6000)/- 12.9000).exp());
        let a_37 = a_5/(a_5+a_24);
        let a_44 = 0.0300000/(1.00000+((self.v+40.0000)/6.00000).exp())+0.000350000;
        let r_7 = (a_37 - self.h1_gate)/a_44;
        let a_6 =  44.9000*((self.v+66.9000)/- 5.57000).exp();
        let a_25 = 1491.00/(1.00000+ 323.300*((self.v+94.6000)/- 12.9000).exp());
        let a_38 = a_6/(a_6+a_25);
        let a_45 = 0.120000/(1.00000+((self.v+60.0000)/2.00000).exp())+0.00295000;
        let r_8 = (a_38 - self.h2_gate)/a_45;
        let a_39 = 1.00000/(1.00000+((self.v - (- 3.20000+self.act_shift))/self.slope_factor_act).exp());
        let a_7 = ( - 26.1200*(self.v+35.0000))/(((self.v+35.0000)/- 2.50000).exp() - 1.00000)+( - 78.1100*self.v)/(( - 0.208000*self.v).exp() - 1.00000);
        let a_26 = ( 10.5200*(self.v - 5.00000))/(( 0.400000*(self.v - 5.00000)).exp() - 1.00000);
        let a_46 = 1.00000/(a_7+a_26);
        let r_9 = (a_39 - self.d_gate)/a_46;
        let a_32 = 1.00000/( 0.150000*(- self.v/11.0000).exp()+ 0.200000*(- self.v/700.000).exp());
        let a_40 = 1.00000/( 16.0000*(self.v/8.00000).exp()+ 15.0000*(self.v/50.0000).exp());
        let a_47 = 0.00100000/(a_32+a_40);
        let a_13 = 1.00000/(1.00000+((self.v - - 49.1000)/- 8.98000).exp());
        let r_15 = (a_13 - self.qa_gate)/a_47;
        let a_14 = 0.150400/( 3100.00*(self.v/13.0000).exp()+ 700.000*(self.v/70.0000).exp());
        let a_33 = 0.150400/( 95.0000*(- self.v/10.0000).exp()+ 50.0000*(- self.v/700.000).exp())+0.000229000/(1.00000+(- self.v/5.00000).exp());
        let a_48 = 0.00100000/(a_14+a_33);
        let a_41 = a_14/(a_14+a_33);
        let r_16 = (a_41 - self.qi_gate)/a_48;
        let mut a_65 = if self.v.abs() < 1e-4 {
            (self.g_na * self.m_gate.powf(3.0) * (0.635 * self.h1_gate + 0.365 * self.h2_gate) * p.nao * self.f_faraday)
                * (((0.0 - e_na) / self.rtonf).exp() - 1.0)
        } else {
            ((self.g_na * self.m_gate.powf(3.0) * (0.635 * self.h1_gate + 0.365 * self.h2_gate) * p.nao * self.v * self.f_faraday) / self.rtonf)
                * (((self.v - e_na) / self.rtonf).exp() - 1.0) / ((self.v / self.rtonf).exp() - 1.0)
        };
        a_65 = a_65 * p.block_na * p.isch_block();
        let a_68 = ( self.g_ach_max*self.achf_gate*self.achs_gate*(self.ach).powf(1.50000))/((self.k_ach).powf(1.50000)+(self.ach).powf(1.50000));
        let a_69 = ( (( a_68*p.effective_ko())/(10.0000+p.effective_ko()))*(self.v - e_k))/(1.00000+(((self.v - e_k) - 140.000)/( 2.50000*self.rtonf)).exp());
        let a_70 =   self.g_cal*self.d_gate*( 0.675000*self.f_gate+ 0.325000*self.f2)*(self.v - self.e_cal)*(1.00000 - (( a_69*self.ach)/(9.00000e-05+self.ach))/1.00000) * p.block_ca * p.isch_block() * p.ans_ca_modifier();
        let a_66 =   self.g_to*self.r_gate*( 0.450000*self.q_fast+ 0.550000*self.q_slow)*(self.v - e_k) * p.block_k;
        let a_34 =   self.g_kr*( 0.900000*self.paf_gate+ 0.100000*self.pas_gate)*self.pik_gate*(self.v - e_k) * p.block_k;
        let a_17 =  self.y_gate*self.g_f*(self.v - - 30.0000);
        let a_67 =  self.g_st*self.qa_gate*self.qi_gate*(self.v - self.e_st);
        let a_42 =  self.g_k1*(0.500000+0.500000/(1.00000+((self.v+30.0000)/5.00000).exp()));
        let a_49 = ( a_42*(p.effective_ko()/(p.effective_ko()+0.590000)).powf(3.00000)*(self.v+81.9000))/(1.00000+(( 1.39300*(self.v+81.9000+3.60000))/self.rtonf).exp());
        let a_57 = (( - self.qn*self.v)/( 2.00000*self.rtonf)).exp();
        let a_52 = 1.00000+ (p.cao/self.kco)*(1.00000+(( self.qco*self.v)/self.rtonf).exp())+p.nao/self.k1no+(p.nao).powf(2.00000)/( self.k1no*self.k2no)+(p.nao).powf(3.00000)/( self.k1no*self.k2no*self.k3no);
        let a_54 = ( ((p.nao).powf(2.00000)/( self.k1no*self.k2no)+(p.nao).powf(3.00000)/( self.k1no*self.k2no*self.k3no))*(( - self.qn*self.v)/( 2.00000*self.rtonf)).exp())/a_52;
        let a_55 = ( (p.cao/self.kco)*(( - self.qco*self.v)/self.rtonf).exp())/a_52;
        let a_53 = (( self.qn*self.v)/( 2.00000*self.rtonf)).exp();
        let a_60 =  a_57*self.k34*(a_54+a_55)+ a_55*a_53*(self.k43+a_57);
        let a_56 = 1.00000+ (self.casub/self.kci)*(1.00000+(( - self.qci*self.v)/self.rtonf).exp()+self.nai/self.kcni)+self.nai/self.k1ni+(self.nai).powf(2.00000)/( self.k1ni*self.k2ni)+(self.nai).powf(3.00000)/( self.k1ni*self.k2ni*self.k3ni);
        let a_59 = ( (self.casub/self.kci)*(( - self.qci*self.v)/self.rtonf).exp())/a_56;
        let a_58 = ( ((self.nai).powf(2.00000)/( self.k1ni*self.k2ni)+(self.nai).powf(3.00000)/( self.k1ni*self.k2ni*self.k3ni))*(( self.qn*self.v)/( 2.00000*self.rtonf)).exp())/a_56;
        let a_61 =  a_53*self.k43*(a_58+a_59)+ a_57*a_59*(self.k34+a_53);
        let a_62 =  a_58*self.k43*(a_54+a_55)+ a_59*a_54*(self.k43+a_57);
        let a_63 =  a_54*self.k34*(a_58+a_59)+ a_58*a_55*(self.k34+a_53);
        let a_64 = ( self.knaca*( a_61*a_55 -  a_60*a_59))/(a_60+a_61+a_62+a_63);
        let a_51 = ( self.i_p*(self.nai/(5.64000+self.nai)).powf(3.00000)*(p.effective_ko()/(0.621000+p.effective_ko())).powf(2.00000)*1.60000)/(1.50000+(- (self.v+60.0000)/40.0000).exp());
        let a_50 =  self.g_b*(self.v - self.e_b);
        let r_0 = - (a_65+a_70+a_66+a_34+a_17+a_67+a_49+a_64+a_51+a_50+a_69)/self.c;
        let a_72 = 5.00000/(1.00000+self.k_up/self.cai);
        let a_73 = (self.ca_up - self.ca_rel)/self.tau_tr;
        let r_20 = a_72 - ( a_73*self.v_rel)/self.v_up;
        let a_74 =  88800.0*self.cai*(1.00000 - self.f_tc) -  446.000*self.f_tc;
        let r_22 = a_74;
        let a_76 =  227700.*self.cai*((1.00000 - self.f_tmc) - self.f_tmm) -  7.51000*self.f_tmc;
        let r_23 = a_76;
        let a_75 = ( self.p_rel*(self.ca_rel - self.casub))/(1.00000+(0.00120000/self.casub).powf(2.00000));
        let a_79 =  534.000*self.ca_rel*(1.00000 - self.f_cq) -  445.000*self.f_cq;
        let r_21 = (a_73 - a_75) -  10.0000*a_79;
        let a_71 = (self.casub - self.cai)/4.00000e-05;
        let a_77 =  227700.*self.cai*(1.00000 - self.f_cmi) -  542.000*self.f_cmi;
        let r_19 = ( a_71*self.v_sub -  a_72*self.v_up)/self.vi - ( 0.0450000*a_77+ 0.0310000*a_74+ 0.0620000*a_76);
        let r_25 = a_77;
        let a_78 =  227700.*self.casub*(1.00000 - self.f_cms) -  542.000*self.f_cms;
        let r_26 = a_78;
        let r_27 = a_79;
        let a_80 =  0.00100000*( 115.000*self.casub*(1.00000 - self.f_csl) -  1000.00*self.f_csl);
        let r_5 = (((- (a_70 -  2.00000*a_64)/( 2.00000*self.f_faraday)+ a_75*self.v_rel)/self.v_sub - a_71) -  0.0450000*a_78) -  (0.0310000/1.20000)*a_80;
        let r_28 = a_80;

        // Atualização dos Estados (Forward Euler)
        self.v += r_0 * dt;
        self.y_gate += r_1 * dt;
        self.paf_gate += r_2 * dt;
        self.pas_gate += r_3 * dt;
        self.pik_gate += r_4 * dt;
        self.casub += r_5 * dt;
        self.m_gate += r_6 * dt;
        self.h1_gate += r_7 * dt;
        self.h2_gate += r_8 * dt;
        self.d_gate += r_9 * dt;
        self.f_gate += r_10 * dt;
        self.f2 += r_11 * dt;
        self.r_gate += r_12 * dt;
        self.q_fast += r_13 * dt;
        self.q_slow += r_14 * dt;
        self.qa_gate += r_15 * dt;
        self.qi_gate += r_16 * dt;
        self.achf_gate += r_17 * dt;
        self.achs_gate += r_18 * dt;
        self.cai += r_19 * dt;
        self.ca_up += r_20 * dt;
        self.ca_rel += r_21 * dt;
        self.f_tc += r_22 * dt;
        self.f_tmc += r_23 * dt;
        self.f_tmm += r_24 * dt;
        self.f_cmi += r_25 * dt;
        self.f_cms += r_26 * dt;
        self.f_cq += r_27 * dt;
        self.f_csl += r_28 * dt;
    }
}
