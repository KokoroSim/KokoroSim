use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    Rolling,         // Fita Deslizante (Padrão)
    Sweep,           // Varredura Contínua (Monitor UTI com barra apagadora)
    Paged,           // Paginação Sincronizada (disparo do Nó SA)
    TriggeredAuto,   // Gatilho Automático ciclo a ciclo
    TriggeredSingle, // Gatilho Único (Single-Shot congelado)
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::Rolling
    }
}

impl ViewMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "sweep" => ViewMode::Sweep,
            "paged" => ViewMode::Paged,
            "triggered_auto" => ViewMode::TriggeredAuto,
            "triggered_single" => ViewMode::TriggeredSingle,
            _ => ViewMode::Rolling,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SingleState {
    Armed,
    Recording,
    Frozen,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GhostCaptureState {
    Idle,
    Armed,
    Recording,
    Ready,
}

pub struct Plotter {
    canvas_id: String,
    buffer: Vec<f64>,
    capacity: usize,
    ghost_state: GhostCaptureState,
    ghost_beat: Vec<f64>,
    ghost_recording: Vec<f64>,
    ghost_rest_val: f64,
    ghost_playback_idx: usize,
    ghost_rolling_buffer: Vec<f64>,
    mode: ViewMode,
    head_idx: usize,
    paged_frozen: bool,
    single_state: SingleState,
    sound_markers: Vec<(usize, u8)>, // (index, sound_code)
    total_pushed: u64, // Contador total de amostras recebidas (usado para offset de grade)
}

impl Plotter {
    pub fn new(canvas_id: &str, capacity: usize) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            buffer: Vec::with_capacity(capacity),
            capacity,
            ghost_state: GhostCaptureState::Idle,
            ghost_beat: Vec::new(),
            ghost_recording: Vec::new(),
            ghost_rest_val: 0.0,
            ghost_playback_idx: 0,
            ghost_rolling_buffer: Vec::with_capacity(capacity),
            mode: ViewMode::Rolling,
            head_idx: 0,
            paged_frozen: false,
            single_state: SingleState::Armed,
            sound_markers: Vec::new(),
            total_pushed: 0,
        }
    }

    pub fn set_mode(&mut self, mode: ViewMode) {
        if self.mode != mode {
            self.mode = mode;
            self.head_idx = 0;
            self.paged_frozen = false;
            self.single_state = SingleState::Armed;
            self.sound_markers.clear();
            // Reset completo do buffer para eliminar rastros de outros modos
            self.buffer.clear();
            if mode != ViewMode::Rolling {
                self.buffer.resize(self.capacity, f64::NAN);
            }
            if mode == ViewMode::Rolling && self.ghost_state == GhostCaptureState::Ready {
                self.ghost_playback_idx = 0;
                self.ghost_rolling_buffer.clear();
                self.ghost_rolling_buffer.resize(self.capacity, self.ghost_rest_val);
            }
        }
    }

    pub fn capture_ghost(&mut self) {
        self.ghost_state = GhostCaptureState::Armed;
        self.ghost_recording.clear();
    }

    pub fn ghost_state(&self) -> GhostCaptureState {
        self.ghost_state
    }

    pub fn clear_ghost(&mut self) {
        self.ghost_state = GhostCaptureState::Idle;
        self.ghost_beat.clear();
        self.ghost_recording.clear();
        self.ghost_rolling_buffer.clear();
        self.ghost_playback_idx = 0;
    }

    pub fn arm_single(&mut self) {
        self.single_state = SingleState::Armed;
        self.head_idx = 0;
        self.sound_markers.clear();
    }

    pub fn push(&mut self, value: f64, sound_code: u32, is_sa_fire: bool) {
        let sound_flags = (sound_code & 7) as u8;

        // 1. Gravação bio-disparada do ciclo basal de referência (Fase 0 do Nó SA)
        match self.ghost_state {
            GhostCaptureState::Armed => {
                if is_sa_fire {
                    self.ghost_state = GhostCaptureState::Recording;
                    self.ghost_recording.clear();
                    self.ghost_recording.push(value);
                }
            }
            GhostCaptureState::Recording => {
                if is_sa_fire && self.ghost_recording.len() > 10 {
                    // O ciclo cardíaco RR completou 1 batimento inteiro
                    self.ghost_beat = self.ghost_recording.clone();
                    self.ghost_rest_val = value;
                    self.ghost_state = GhostCaptureState::Ready;
                    self.ghost_playback_idx = 0;
                    self.ghost_recording.clear();
                    if self.mode == ViewMode::Rolling {
                        self.ghost_rolling_buffer.clear();
                        self.ghost_rolling_buffer.resize(self.capacity, self.ghost_rest_val);
                    }
                } else {
                    self.ghost_recording.push(value);
                    if self.ghost_recording.len() >= self.capacity {
                        self.ghost_beat = self.ghost_recording.clone();
                        self.ghost_rest_val = value;
                        self.ghost_state = GhostCaptureState::Ready;
                        self.ghost_playback_idx = 0;
                        self.ghost_recording.clear();
                        if self.mode == ViewMode::Rolling {
                            self.ghost_rolling_buffer.clear();
                            self.ghost_rolling_buffer.resize(self.capacity, self.ghost_rest_val);
                        }
                    }
                }
            }
            _ => {}
        }

        match self.mode {
            ViewMode::Rolling => {
                if self.buffer.len() >= self.capacity {
                    self.buffer.remove(0);
                }
                self.buffer.push(value);

                // Projeção e sincronização da onda fantasma
                if self.ghost_state == GhostCaptureState::Ready && !self.ghost_beat.is_empty() {
                    if is_sa_fire {
                        // Novo batimento: reancora a onda fantasma na Fase 0
                        self.ghost_playback_idx = 0;
                    }

                    let ghost_val = if self.ghost_playback_idx < self.ghost_beat.len() {
                        let v = self.ghost_beat[self.ghost_playback_idx];
                        self.ghost_playback_idx += 1;
                        v
                    } else {
                        self.ghost_rest_val
                    };

                    if self.ghost_rolling_buffer.len() >= self.capacity {
                        self.ghost_rolling_buffer.remove(0);
                    }
                    self.ghost_rolling_buffer.push(ghost_val);
                }

                // Desloca marcadores sonoros existentes 1 índice à esquerda
                self.sound_markers.retain_mut(|(idx, _)| {
                    if *idx > 0 {
                        *idx -= 1;
                        true
                    } else {
                        false
                    }
                });
                if sound_flags != 0 {
                    self.sound_markers.push((self.buffer.len().saturating_sub(1), sound_flags));
                }
            }

            ViewMode::Sweep => {
                if self.buffer.len() < self.capacity {
                    self.buffer.resize(self.capacity, value);
                }
                self.buffer[self.head_idx] = value;

                // Limpa marcadores sonoros na janela de apagamento à frente da caneta
                let cur = self.head_idx;
                let erase_samples = 25;
                let erase_end = (cur + erase_samples).min(self.capacity);
                self.sound_markers.retain(|(idx, _)| *idx < cur || *idx >= erase_end);

                if sound_flags != 0 {
                    self.sound_markers.push((cur, sound_flags));
                }
                self.head_idx = (self.head_idx + 1) % self.capacity;
            }

            ViewMode::Paged => {
                if self.buffer.len() < self.capacity {
                    self.buffer.resize(self.capacity, value);
                }

                if is_sa_fire && self.paged_frozen {
                    // Novo ciclo do Nó SA: limpa a tela e começa nova página
                    self.head_idx = 0;
                    self.paged_frozen = false;
                    self.sound_markers.clear();
                }

                if !self.paged_frozen {
                    self.buffer[self.head_idx] = value;
                    if sound_flags != 0 {
                        self.sound_markers.push((self.head_idx, sound_flags));
                    }
                    self.head_idx += 1;
                    if self.head_idx >= self.capacity {
                        self.paged_frozen = true; // Página completa: congela até o próximo marco
                    }
                }
            }

            ViewMode::TriggeredAuto => {
                if self.buffer.len() < self.capacity {
                    self.buffer.resize(self.capacity, f64::NAN);
                }

                if is_sa_fire {
                    // Novo batimento: reseta caneta para X=0 e limpa rastro anterior
                    self.head_idx = 0;
                    self.sound_markers.clear();
                    let erase_samples = 25;
                    let end = erase_samples.min(self.capacity);
                    for i in 0..end {
                        self.buffer[i] = f64::NAN;
                    }
                }

                self.buffer[self.head_idx] = value;
                let cur = self.head_idx;
                let erase_samples = 25;
                let erase_end = (cur + erase_samples).min(self.capacity);
                self.sound_markers.retain(|(idx, _)| *idx < cur || *idx >= erase_end);
                for i in (cur + 1)..erase_end {
                    self.buffer[i] = f64::NAN;
                }

                if sound_flags != 0 {
                    self.sound_markers.push((cur, sound_flags));
                }
                self.head_idx = (self.head_idx + 1) % self.capacity;
            }

            ViewMode::TriggeredSingle => {
                if self.buffer.len() < self.capacity {
                    self.buffer.resize(self.capacity, f64::NAN);
                }

                match self.single_state {
                    SingleState::Armed => {
                        if is_sa_fire {
                            self.head_idx = 0;
                            self.single_state = SingleState::Recording;
                            self.sound_markers.clear();
                            for i in 0..self.capacity {
                                self.buffer[i] = f64::NAN;
                            }
                        }
                    }
                    SingleState::Recording => {
                        self.buffer[self.head_idx] = value;
                        if sound_flags != 0 {
                            self.sound_markers.push((self.head_idx, sound_flags));
                        }
                        self.head_idx += 1;
                        if self.head_idx >= self.capacity {
                            self.single_state = SingleState::Frozen; // Gravação de 1 ciclo finalizada
                        }
                    }
                    SingleState::Frozen => {
                        // Permanece estático aguardando clique em armar
                    }
                }
            }
        }
        self.total_pushed = self.total_pushed.wrapping_add(1);
    }

    pub fn draw(
        &self,
        min_y: f64,
        max_y: f64,
        color: &str,
        clear: bool,
        show_ghost: bool,
        show_uti_markers: bool,
        show_bulhas_markers: bool,
        is_ecg_grid: bool,
        scale_labels: Option<(&str, &str, &str)>,
    ) {
        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };

        if let Some(canvas) = document.get_element_by_id(&self.canvas_id) {
            if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                let width = canvas.client_width() as f64;
                let height = canvas.client_height() as f64;

                if canvas.width() as f64 != width || canvas.height() as f64 != height {
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);
                }

                if let Ok(Some(ctx)) = canvas.get_context("2d") {
                    let ctx = ctx.dyn_into::<CanvasRenderingContext2d>().unwrap();

                    let range_y = max_y - min_y;
                    let dx = width / (self.capacity as f64 - 1.0);

                    // Parâmetros da grade milimetrada isotrópica (40ms x 0.1mV)
                    // Janela de 500 amostras a cada 5ms = 2500ms totais (62.5 quadradinhos de 40ms)
                    let s = width / 62.5; // Tamanho de 1 quadradinho em pixels

                    // Posição da isoline (0 mV) no canvas.
                    // Em modo ECG isotrópico, y_zero é calculado para garantir que os picos caibam:
                    // - Pico positivo máximo esperado: ~1.10 mV (R em V5 com kr=1.45 × 0.75)
                    // - Pico negativo máximo esperado: ~0.25 mV (S em V1 com ks=1.30 × 0.17)
                    // - Margem: 10% de 's' em cada lado
                    let y_zero = if is_ecg_grid {
                        let peak_pos_mv = 1.15_f64; // mV acima da isoline (headroom positivo)
                        let peak_neg_mv = 0.30_f64; // mV abaixo da isoline (headroom negativo)
                        let margin_px = s * 0.5;    // margem extra em pixels (0.5 quadradinhos)
                        let px_above = peak_pos_mv * 10.0 * s + margin_px;
                        let px_below = peak_neg_mv * 10.0 * s + margin_px;
                        let total_px = px_above + px_below;
                        if total_px <= height {
                            // Há espaço suficiente: posiciona a isoline exatamente
                            px_above
                        } else {
                            // Canvas muito pequeno: escala para caber (mantém proporção acima/abaixo)
                            height * (px_above / total_px)
                        }
                    } else {
                        height - ((0.0 - min_y) / range_y) * height
                    };

                    // Mapeamento vertical de voltagem -> pixel
                    let val_to_y = |val: f64| -> f64 {
                        if is_ecg_grid {
                            y_zero - (val * 10.0 * s)
                        } else {
                            height - ((val - min_y) / range_y) * height
                        }
                    };

                    // Limpa fundo e desenha linhas da grade e eventos de áudio (se for o plotter primário do canvas)
                    if clear {
                        ctx.set_fill_style_str("#000000");
                        ctx.fill_rect(0.0, 0.0, width, height);

                        // Grade Milimetrada Isotrópica de ECG (40ms x 0.1mV)
                        if is_ecg_grid {
                            ctx.save();

                            // Offset horizontal da grade para rolar junto com o traçado.
                            // 1 quadradinho = 8 amostras (40ms a 5ms/amostra).
                            // A grade SÓ começa a rolar quando o buffer está cheio e samples
                            // começam a ser descartados (pushed > capacity). Antes disso, fica parada.
                            let grid_offset_x = match self.mode {
                                ViewMode::Rolling => {
                                    if self.total_pushed <= self.capacity as u64 {
                                        // Buffer ainda enchendo: grade estática
                                        0.0
                                    } else {
                                        // Buffer cheio: cada sample extra desloca a grade
                                        let overflow = self.total_pushed - self.capacity as u64;
                                        (overflow as f64 * dx).rem_euclid(s)
                                    }
                                }
                                ViewMode::Sweep | ViewMode::TriggeredAuto | ViewMode::TriggeredSingle => {
                                    // Nos modos com caneta, ancora a grade na posição atual da caneta
                                    (self.head_idx as f64 * dx).rem_euclid(s)
                                }
                                ViewMode::Paged => {
                                    // Paged: grade fixa, ancorada no início da página
                                    (self.head_idx as f64 * dx).rem_euclid(s)
                                }
                            };

                            // phase_count para determinar qual linha é quadratão (200ms)
                            let phase_count = match self.mode {
                                ViewMode::Rolling => {
                                    if self.total_pushed <= self.capacity as u64 {
                                        0u64
                                    } else {
                                        self.total_pushed - self.capacity as u64
                                    }
                                }
                                _ => self.head_idx as u64,
                            };
                            // k_base: quantos quadradinhos inteiros foram "consumidos" até o início da janela
                            let k_base_offset = ((phase_count as f64 * dx / s).floor() as i64).rem_euclid(5) as usize;
                            let mut k_rel = 0usize;

                            // 1. Linhas verticais isotrópicas de tempo (a cada s = 40ms)
                            // Começamos em x negativo para cobrir a borda esquerda com o offset
                            let start_x = -s + (s - grid_offset_x).rem_euclid(s);
                            let mut x = start_x;
                            while x <= width + 1.0 {
                                ctx.begin_path();
                                // k_mod: posição relativa no ciclo de 5 quadradinhos (0 = quadratão)
                                let k_mod = (k_base_offset + k_rel) % 5;
                                if k_mod == 0 {
                                    // Quadratão de tempo (200ms)
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.22)");
                                    ctx.set_line_width(1.0);
                                } else {
                                    // Quadradinho pequeno (40ms)
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.07)");
                                    ctx.set_line_width(0.5);
                                }
                                ctx.move_to(x, 0.0);
                                ctx.line_to(x, height);
                                ctx.stroke();
                                x += s;
                                k_rel += 1;
                            }

                            // 2. Linhas horizontais isotrópicas de voltagem (a cada s = 0.1mV) a partir de y_zero
                            // Acima da linha de base (voltagens positivas)
                            let mut j = 0;
                            while y_zero - (j as f64) * s >= -1.0 {
                                let y = y_zero - (j as f64) * s;
                                ctx.begin_path();
                                if j == 0 {
                                    // Linha isoelétrica (0.0 mV)
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.40)");
                                    ctx.set_line_width(1.2);
                                } else if j % 5 == 0 {
                                    // Quadratão de voltagem (0.5 mV)
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.22)");
                                    ctx.set_line_width(1.0);
                                } else {
                                    // Quadradinho de voltagem (0.1 mV)
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.07)");
                                    ctx.set_line_width(0.5);
                                }
                                ctx.move_to(0.0, y);
                                ctx.line_to(width, y);
                                ctx.stroke();
                                j += 1;
                            }

                            // Abaixo da linha de base (voltagens negativas)
                            let mut j_neg = 1;
                            while y_zero + (j_neg as f64) * s <= height + 1.0 {
                                let y = y_zero + (j_neg as f64) * s;
                                ctx.begin_path();
                                if j_neg % 5 == 0 {
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.22)");
                                    ctx.set_line_width(1.0);
                                } else {
                                    ctx.set_stroke_style_str("rgba(0, 242, 254, 0.07)");
                                    ctx.set_line_width(0.5);
                                }
                                ctx.move_to(0.0, y);
                                ctx.line_to(width, y);
                                ctx.stroke();
                                j_neg += 1;
                            }

                            ctx.restore();
                        }

                        // Eventos acústicos sincronizados
                        for &(idx, flags) in &self.sound_markers {
                            let x = idx as f64 * dx;
                            // 1. Onda R / Bip da UTI (amarelo tracejado)
                            if (flags & 1) != 0 && show_uti_markers {
                                ctx.save();
                                ctx.begin_path();
                                ctx.set_stroke_style_str("rgba(241, 196, 15, 0.75)");
                                ctx.set_line_width(1.5);
                                let _ = ctx.set_line_dash(&js_sys::Array::of2(
                                    &wasm_bindgen::JsValue::from(4),
                                    &wasm_bindgen::JsValue::from(4),
                                ));
                                ctx.move_to(x, 0.0);
                                ctx.line_to(x, height);
                                ctx.stroke();
                                ctx.restore();
                            }
                            // 2. Bulha B1 (Mitral - verde esmeralda)
                            if (flags & 2) != 0 && show_bulhas_markers {
                                ctx.save();
                                ctx.begin_path();
                                ctx.set_stroke_style_str("rgba(46, 204, 113, 0.75)");
                                ctx.set_line_width(1.5);
                                ctx.move_to(x, 0.0);
                                ctx.line_to(x, height);
                                ctx.stroke();
                                ctx.restore();
                            }
                            // 3. Bulha B2 (Aórtica - coral)
                            if (flags & 4) != 0 && show_bulhas_markers {
                                ctx.save();
                                ctx.begin_path();
                                ctx.set_stroke_style_str("rgba(255, 118, 117, 0.75)");
                                ctx.set_line_width(1.5);
                                ctx.move_to(x, 0.0);
                                ctx.line_to(x, height);
                                ctx.stroke();
                                ctx.restore();
                            }
                        }
                    }

                    if self.buffer.is_empty() {
                        return;
                    }

                    // 1. Desenha Onda Fantasma: Linha contínua sólida esmaecida (sem pontilhado, anti-fadiga visual)
                    if show_ghost && self.ghost_state == GhostCaptureState::Ready && !self.ghost_beat.is_empty() {
                        ctx.save();
                        ctx.begin_path();
                        ctx.set_stroke_style_str(&format_ghost_color(color));
                        ctx.set_line_width(1.2);
                        ctx.set_shadow_blur(0.0);

                        let mut first = true;
                        match self.mode {
                            ViewMode::Rolling => {
                                for (i, &val) in self.ghost_rolling_buffer.iter().enumerate() {
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                            }
                            ViewMode::Sweep | ViewMode::TriggeredAuto | ViewMode::Paged | ViewMode::TriggeredSingle => {
                                let beat_len = self.ghost_beat.len();
                                for i in 0..self.capacity {
                                    let val = if i < beat_len {
                                        self.ghost_beat[i]
                                    } else {
                                        self.ghost_rest_val
                                    };
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                            }
                        }
                        ctx.stroke();
                        ctx.restore();
                    }

                    // 2. Desenha o traçado ativo de acordo com o ViewMode
                    ctx.set_stroke_style_str(color);
                    ctx.set_line_width(2.0);
                    ctx.set_shadow_color(color);
                    ctx.set_shadow_blur(5.0);

                    match self.mode {
                        ViewMode::Rolling => {
                            ctx.begin_path();
                            let mut first = true;
                            for (i, &val) in self.buffer.iter().enumerate() {
                                if !val.is_nan() {
                                    let x = i as f64 * dx;
                                    let y = val_to_y(val);
                                    if first {
                                        ctx.move_to(x, y);
                                        first = false;
                                    } else {
                                        ctx.line_to(x, y);
                                    }
                                } else {
                                    first = true;
                                }
                            }
                            ctx.stroke();
                        }

                        ViewMode::Sweep | ViewMode::TriggeredAuto => {
                            let erase_samples = 25;
                            let cur = self.head_idx;
                            let erase_end = (cur + erase_samples).min(self.capacity);

                            // Segmento A: da barra apagadora até o final (traço do ciclo anterior)
                            if erase_end < self.capacity {
                                ctx.begin_path();
                                let mut first = true;
                                for i in erase_end..self.capacity {
                                    let val = self.buffer[i];
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                                ctx.stroke();
                            }

                            // Segmento B: do início da tela até a caneta atual (traço do ciclo novo)
                            if cur > 0 {
                                ctx.begin_path();
                                let mut first = true;
                                for i in 0..cur {
                                    let val = self.buffer[i];
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                                ctx.stroke();
                            }

                            // Barra apagadora preta à frente da caneta
                            if clear {
                                let head_x = cur as f64 * dx;
                                let bar_w = (erase_samples as f64 * dx).max(18.0);
                                ctx.set_fill_style_str("#000000");
                                ctx.fill_rect(head_x, 0.0, bar_w.min(width - head_x), height);
                                if head_x + bar_w > width {
                                    ctx.fill_rect(0.0, 0.0, head_x + bar_w - width, height);
                                }
                            }
                        }

                        ViewMode::Paged => {
                            let end = if self.paged_frozen {
                                self.capacity
                            } else {
                                self.head_idx.min(self.capacity)
                            };

                            if end > 0 {
                                ctx.begin_path();
                                let mut first = true;
                                for i in 0..end {
                                    let val = self.buffer[i];
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                                ctx.stroke();
                            }
                        }

                        ViewMode::TriggeredSingle => {
                            let end = match self.single_state {
                                SingleState::Armed => 0,
                                SingleState::Recording => self.head_idx.min(self.capacity),
                                SingleState::Frozen => self.capacity,
                            };

                            if end > 0 {
                                ctx.begin_path();
                                let mut first = true;
                                for i in 0..end {
                                    let val = self.buffer[i];
                                    if !val.is_nan() {
                                        let x = i as f64 * dx;
                                        let y = val_to_y(val);
                                        if first {
                                            ctx.move_to(x, y);
                                            first = false;
                                        } else {
                                            ctx.line_to(x, y);
                                        }
                                    } else {
                                        first = true;
                                    }
                                }
                                ctx.stroke();
                            }
                        }
                    }

                    // 3. Desenha Escalas Metrológicas Dinâmicas (Y_max, Y_mid, Y_min)
                    if let Some((top_str, mid_str, bot_str)) = scale_labels {
                        ctx.save();
                        ctx.set_font("10px 'Chakra Petch', monospace");
                        ctx.set_text_align("right");

                        // Topo (Y_max)
                        ctx.set_fill_style_str("rgba(255, 255, 255, 0.45)");
                        ctx.set_text_baseline("top");
                        let _ = ctx.fill_text(top_str, width - 8.0, 6.0);

                        // Centro (Y_mid / Linha de Referência)
                        let mid_y = if is_ecg_grid {
                            y_zero
                        } else {
                            height * 0.5
                        };
                        ctx.set_fill_style_str("rgba(0, 242, 254, 0.50)");
                        ctx.set_text_baseline("middle");
                        let _ = ctx.fill_text(mid_str, width - 8.0, mid_y);

                        // Base (Y_min)
                        ctx.set_fill_style_str("rgba(255, 255, 255, 0.45)");
                        ctx.set_text_baseline("bottom");
                        let _ = ctx.fill_text(bot_str, width - 8.0, height - 6.0);

                        ctx.restore();
                    }
                }
            }
        }
    }

    pub fn draw_scales(
        &self,
        min_y: f64,
        max_y: f64,
        is_ecg_grid: bool,
        top_str: &str,
        mid_str: &str,
        bot_str: &str,
    ) {
        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };

        if let Some(canvas) = document.get_element_by_id(&self.canvas_id) {
            if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                let width = canvas.client_width() as f64;
                let height = canvas.client_height() as f64;

                if let Ok(Some(ctx)) = canvas.get_context("2d") {
                    let ctx = ctx.dyn_into::<CanvasRenderingContext2d>().unwrap();
                    let range_y = max_y - min_y;
                    let y_zero = height - ((0.0 - min_y) / range_y) * height;

                    ctx.save();
                    ctx.set_font("10px 'Chakra Petch', monospace");
                    ctx.set_text_align("right");

                    // Topo (Y_max)
                    ctx.set_fill_style_str("rgba(255, 255, 255, 0.45)");
                    ctx.set_text_baseline("top");
                    let _ = ctx.fill_text(top_str, width - 8.0, 6.0);

                    // Centro (Y_mid / Linha de Referência)
                    let mid_y = if is_ecg_grid {
                        y_zero
                    } else {
                        height * 0.5
                    };
                    ctx.set_fill_style_str("rgba(0, 242, 254, 0.50)");
                    ctx.set_text_baseline("middle");
                    let _ = ctx.fill_text(mid_str, width - 8.0, mid_y);

                    // Base (Y_min)
                    ctx.set_fill_style_str("rgba(255, 255, 255, 0.45)");
                    ctx.set_text_baseline("bottom");
                    let _ = ctx.fill_text(bot_str, width - 8.0, height - 6.0);

                    ctx.restore();
                }
            }
        }
    }
}

fn format_ghost_color(hex: &str) -> String {
    if hex.starts_with('#') && hex.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            return format!("rgba({}, {}, {}, 0.45)", r, g, b);
        }
    }
    "rgba(180, 180, 180, 0.45)".to_string()
}

pub struct PvLoopPlotter {
    canvas_id: String,
    points: Vec<(f64, f64)>, // (Volume em mL, Pressão em mmHg)
    capacity: usize,         // ~600 a 800 amostras (cobre ~1.5 a 2 ciclos completos)
}

impl PvLoopPlotter {
    pub fn new(canvas_id: &str, capacity: usize) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            points: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, vol: f64, press: f64) {
        if self.points.len() >= self.capacity {
            self.points.remove(0);
        }
        self.points.push((vol, press));
    }

    pub fn reset(&mut self) {
        self.points.clear();
    }

    pub fn draw(&self, v_edv: f64, v_esv: f64, ef: f64, sv: f64) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };
        let canvas = match document.get_element_by_id(&self.canvas_id) {
            Some(c) => match c.dyn_into::<HtmlCanvasElement>() {
                Ok(c) => c,
                Err(_) => return,
            },
            None => return,
        };

        let width = canvas.client_width() as f64;
        let height = canvas.client_height() as f64;

        if width <= 0.0 || height <= 0.0 {
            return;
        }

        if canvas.width() as f64 != width || canvas.height() as f64 != height {
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
        }

        let ctx = match canvas.get_context("2d") {
            Ok(Some(c)) => match c.dyn_into::<CanvasRenderingContext2d>() {
                Ok(c) => c,
                Err(_) => return,
            },
            _ => return,
        };

        // Escalas do plano de fase (X: 0..200 mL, Y: 0..200 mmHg)
        let min_v = 0.0;
        let max_v = 200.0;
        let min_p = 0.0;
        let max_p = 200.0;

        let margin_left = 32.0;
        let margin_right = 10.0;
        let margin_top = 14.0;
        let margin_bottom = 22.0;

        let plot_w = (width - margin_left - margin_right).max(10.0);
        let plot_h = (height - margin_top - margin_bottom).max(10.0);

        let map_x = |v: f64| margin_left + ((v - min_v) / (max_v - min_v)).clamp(0.0, 1.0) * plot_w;
        let map_y = |p: f64| margin_top + (1.0 - ((p - min_p) / (max_p - min_p)).clamp(0.0, 1.0)) * plot_h;

        // 1. Fundo industrial escuro
        ctx.set_fill_style_str("#020305");
        ctx.fill_rect(0.0, 0.0, width, height);

        // 2. Grade fosforescente sutil (50, 100, 150)
        ctx.set_stroke_style_str("rgba(0, 242, 254, 0.08)");
        ctx.set_line_width(1.0);

        ctx.begin_path();
        for v in [50.0, 100.0, 150.0] {
            let x = map_x(v);
            ctx.move_to(x, margin_top);
            ctx.line_to(x, margin_top + plot_h);
        }
        for p in [50.0, 100.0, 150.0] {
            let y = map_y(p);
            ctx.move_to(margin_left, y);
            ctx.line_to(margin_left + plot_w, y);
        }
        ctx.stroke();

        // 3. Eixos de coordenadas
        ctx.set_stroke_style_str("rgba(255, 255, 255, 0.25)");
        ctx.begin_path();
        ctx.move_to(margin_left, margin_top);
        ctx.line_to(margin_left, margin_top + plot_h);
        ctx.line_to(margin_left + plot_w, margin_top + plot_h);
        ctx.stroke();

        // 4. Rótulos e Ticks numéricos
        ctx.set_fill_style_str("rgba(255, 255, 255, 0.45)");
        ctx.set_font("9px 'Chakra Petch', monospace");
        ctx.set_text_align("right");
        ctx.set_text_baseline("middle");

        for p in [0.0, 50.0, 100.0, 150.0, 200.0] {
            let y = map_y(p);
            let _ = ctx.fill_text(&format!("{:.0}", p), margin_left - 3.0, y);
        }

        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        for v in [50.0, 100.0, 150.0, 200.0] {
            let x = map_x(v);
            let _ = ctx.fill_text(&format!("{:.0}", v), x, margin_top + plot_h + 3.0);
        }

        // Rótulos de grandezas
        ctx.set_fill_style_str("rgba(0, 242, 254, 0.70)");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("P (mmHg)", margin_left + 4.0, margin_top + 2.0);

        ctx.set_text_align("right");
        ctx.set_text_baseline("bottom");
        let _ = ctx.fill_text("V (mL)", margin_left + plot_w, margin_top + plot_h - 2.0);

        // 5. Linhas Teóricas ESPVR e EDPVR
        // ESPVR (Reta de Elastância Sistólica Final: intercepta V0 ~ 15 mL)
        let v0 = 15.0;
        let p_top = 175.0;
        let v_es_top = v0 + (p_top / 2.85);
        ctx.save();
        ctx.set_stroke_style_str("rgba(56, 189, 248, 0.35)");
        ctx.set_line_width(1.0);
        let _ = ctx.set_line_dash(&js_sys::Array::of2(&wasm_bindgen::JsValue::from_f64(3.0), &wasm_bindgen::JsValue::from_f64(3.0)));
        ctx.begin_path();
        ctx.move_to(map_x(v0), map_y(0.0));
        ctx.line_to(map_x(v_es_top), map_y(p_top));
        ctx.stroke();

        ctx.set_font("8px 'Chakra Petch', monospace");
        ctx.set_fill_style_str("rgba(56, 189, 248, 0.55)");
        ctx.set_text_align("left");
        let _ = ctx.fill_text("ESPVR", map_x(v_es_top) + 2.0, map_y(p_top));
        ctx.restore();

        // EDPVR (Curva Diastólica Passiva Não-linear)
        ctx.save();
        ctx.set_stroke_style_str("rgba(168, 85, 247, 0.35)");
        ctx.set_line_width(1.0);
        let _ = ctx.set_line_dash(&js_sys::Array::of2(&wasm_bindgen::JsValue::from_f64(3.0), &wasm_bindgen::JsValue::from_f64(3.0)));
        ctx.begin_path();
        for i in 0..=25 {
            let v = 20.0 + (i as f64 / 25.0) * 160.0;
            let v_excess = (v - 112.0).max(0.0);
            let p_ed = (0.065 + 0.0028 * v_excess) * (v - 15.0).max(0.0);
            let x = map_x(v);
            let y = map_y(p_ed);
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();
        ctx.set_font("8px 'Chakra Petch', monospace");
        ctx.set_fill_style_str("rgba(168, 85, 247, 0.55)");
        ctx.set_text_align("right");
        let _ = ctx.fill_text("EDPVR", map_x(180.0), map_y(26.0));
        ctx.restore();

        // 6. Desenho do Traçado da Alça P x V
        if self.points.len() > 1 {
            ctx.save();
            ctx.set_line_cap("round");
            ctx.set_line_join("round");

            let n = self.points.len();
            for i in 0..(n - 1) {
                let progress = i as f64 / n as f64;
                let alpha = 0.15 + 0.85 * progress;
                let color = format!("rgba(46, 204, 113, {:.2})", alpha);

                ctx.set_stroke_style_str(&color);
                ctx.set_line_width(if progress > 0.85 { 2.2 } else { 1.4 });

                ctx.begin_path();
                let (v1, p1) = self.points[i];
                let (v2, p2) = self.points[i + 1];
                ctx.move_to(map_x(v1), map_y(p1));
                ctx.line_to(map_x(v2), map_y(p2));
                ctx.stroke();
            }

            // Cursor pulsante no ponto instantâneo (V(t), P(t))
            if let Some(&(cur_v, cur_p)) = self.points.last() {
                let cx = map_x(cur_v);
                let cy = map_y(cur_p);
                ctx.set_fill_style_str("#00f2fe");
                ctx.set_shadow_color("#00f2fe");
                ctx.set_shadow_blur(8.0);
                ctx.begin_path();
                let _ = ctx.arc(cx, cy, 3.5, 0.0, std::f64::consts::PI * 2.0);
                ctx.fill();
            }
            ctx.restore();
        }

        // 7. Badge de Métricas Sistólicas
        ctx.save();
        ctx.set_font("10px 'JetBrains Mono', monospace");
        ctx.set_text_align("right");
        ctx.set_text_baseline("top");
        
        ctx.set_fill_style_str("rgba(0, 242, 254, 0.90)");
        let _ = ctx.fill_text(&format!("FE: {:.1}%", ef), width - margin_right - 4.0, margin_top + 4.0);

        ctx.set_fill_style_str("rgba(255, 255, 255, 0.70)");
        let _ = ctx.fill_text(&format!("VS: {:.0}mL", sv), width - margin_right - 4.0, margin_top + 16.0);
        let _ = ctx.fill_text(&format!("VDF:{:.0} VSF:{:.0}", v_edv, v_esv), width - margin_right - 4.0, margin_top + 28.0);
        ctx.restore();
    }
}
