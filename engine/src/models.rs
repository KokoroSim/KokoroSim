pub mod severi;
pub mod inada;
pub mod courtemanche;
pub mod tentusscher;

// Aqui ficará o gerenciador de estado global que orquestra as 4 células
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Pharmaco {
    pub ko: f64,
    pub cao: f64,
    pub nao: f64,
    pub block_na: f64,
    pub block_k: f64,
    pub block_ca: f64,
    pub block_nak: f64,
    pub symp: f64,
    pub parasymp: f64,
    pub isch: f64,
}

impl Pharmaco {
    pub fn effective_ko(&self) -> f64 {
        if self.isch > 0.0 { self.ko + (self.isch * 6.0) } else { self.ko }
    }
    
    pub fn isch_block(&self) -> f64 {
        1.0 - (self.isch * 0.5)
    }

    pub fn ans_ca_modifier(&self) -> f64 {
        1.0 + (self.symp * 0.1) - (self.parasymp * 0.1)
    }
}

impl Default for Pharmaco {
    fn default() -> Self {
        Self {
            ko: 5.4,
            cao: 2.0,
            nao: 140.0,
            block_na: 1.0,
            block_k: 1.0,
            block_ca: 1.0,
            block_nak: 1.0,
            symp: 0.0,
            parasymp: 0.0,
            isch: 0.0,
        }
    }
}

#[wasm_bindgen]
pub struct HeartSystem {
    sa_node: severi::SeveriCell,
    av_node: inada::InadaCell,
    atrium: courtemanche::AtriumCell,
    ventricle: tentusscher::VentricleCell,
    time: f64,
    
    // Condução (Pingers)
    timer_atrium: f64,
    timer_av: f64,
    timer_vent: f64,
    sa_fired: bool,
    atrium_fired: bool,
    av_fired: bool,

    // Farmacologia
    pharm: Pharmaco,
}

#[wasm_bindgen]
impl HeartSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sa_node: severi::SeveriCell::new(),
            av_node: inada::InadaCell::default(),
            atrium: courtemanche::AtriumCell::default(),
            ventricle: tentusscher::VentricleCell::default(),
            time: 0.0,
            timer_atrium: -1.0,
            timer_av: -1.0,
            timer_vent: -1.0,
            sa_fired: false,
            atrium_fired: false,
            av_fired: false,
            pharm: Pharmaco::default(),
        }
    }

    pub fn update_params(&mut self, 
        ko: f64, cao: f64, nao: f64, 
        block_na: f64, block_k: f64, block_ca: f64, block_nak: f64,
        symp: f64, parasymp: f64, isch: f64
    ) {
        self.pharm.ko = ko;
        self.pharm.cao = cao;
        self.pharm.nao = nao;
        self.pharm.block_na = block_na;
        self.pharm.block_k = block_k;
        self.pharm.block_ca = block_ca;
        self.pharm.block_nak = block_nak;
        self.pharm.symp = symp;
        self.pharm.parasymp = parasymp;
        self.pharm.isch = isch;
    }

    pub fn step(&mut self, dt: f64) {
        // Passo de Integração (Forward Euler)
        // O nó SA (Severi) e Nó AV (Inada) foram modelados no artigo original em SEGUNDOS
        let dt_sec = dt / 1000.0;
        self.sa_node.step(dt_sec, &self.pharm);
        self.av_node.step(dt_sec, &self.pharm);
        
        // O Átrio (Courtemanche) e Ventrículo (Ten Tusscher) foram modelados em MILISSEGUNDOS
        self.atrium.step(dt, &self.pharm);
        self.ventricle.step(dt, &self.pharm);
        
        self.time += dt;
        
        // --- SISTEMA DE CONDUÇÃO (Acoplamento Direto) ---

        // Pinger 1: SA -> Átrio (Latência ~10ms)
        if self.sa_node.v >= -30.0 && !self.sa_fired {
            self.timer_atrium = self.time + 10.0;
            self.sa_fired = true;
        } else if self.sa_node.v < -75.0 {
            self.sa_fired = false;
        }

        if self.timer_atrium > 0.0 && self.time >= self.timer_atrium {
            self.atrium.v = -30.0; // Gatilho de Despolarização do Átrio
            self.timer_atrium = -1.0;
        }

        // Pinger 2: Átrio -> AV (Latência ~30ms)
        if self.atrium.v >= -30.0 && !self.atrium_fired {
            self.timer_av = self.time + 30.0;
            self.atrium_fired = true;
        } else if self.atrium.v < -75.0 {
            self.atrium_fired = false;
        }

        if self.timer_av > 0.0 && self.time >= self.timer_av {
            self.av_node.v = -30.0; // Gatilho de Despolarização do Nó AV
            self.timer_av = -1.0;
        }

        // Pinger 3: AV -> Ventrículo (Latência Feixe de His ~60ms)
        if self.av_node.v >= -30.0 && !self.av_fired {
            self.timer_vent = self.time + 60.0;
            self.av_fired = true;
        } else if self.av_node.v < -75.0 {
            self.av_fired = false;
        }

        if self.timer_vent > 0.0 && self.time >= self.timer_vent {
            self.ventricle.v = -30.0; // Gatilho de Despolarização do Ventrículo
            self.timer_vent = -1.0;
        }
    }

    // Roda um lote completo de cálculos no lado do Rust e retorna um array f64 achatado!
    pub fn run_batch(&mut self, dt: f64, steps: usize, downsample: usize) -> Vec<f64> {
        let mut batch = Vec::with_capacity((steps / downsample) * 6);
        for i in 0..steps {
            self.step(dt);
            if i % downsample == 0 {
                batch.push(self.sa_node.v);
                batch.push(self.av_node.v);
                batch.push(self.atrium.v);
                batch.push(self.ventricle.v);
                batch.push(self.ventricle.ca_i); // 4
                batch.push(self.ventricle.force); // 5
            }
        }
        batch
    }

    // Métodos de leitura individuais
    pub fn get_sa_v(&self) -> f64 { self.sa_node.v }
    pub fn get_av_v(&self) -> f64 { self.av_node.v }
    pub fn get_atrium_v(&self) -> f64 { self.atrium.v }
    pub fn get_ventricle_v(&self) -> f64 { self.ventricle.v }
    pub fn get_time(&self) -> f64 { self.time }
}
