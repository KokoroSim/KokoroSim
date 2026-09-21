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
                    let y_zero = height - ((0.0 - min_y) / range_y) * height;

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
                            // O offset em pixels avança 1 'dx' por amostra, com período de 's' pixels.
                            let grid_offset_x = match self.mode {
                                ViewMode::Rolling => {
                                    // No modo Rolling, a grade rola com o total de amostras recebidas
                                    (self.total_pushed as f64 * dx).rem_euclid(s)
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

                            // 1. Linhas verticais isotrópicas de tempo (a cada s = 40ms)
                            // Começamos em x negativo para cobrir a borda esquerda com o offset
                            let start_x = -s + (s - grid_offset_x).rem_euclid(s);
                            let mut x = start_x;
                            // Índice relativo para determinar quadratão (200ms = 5 quadradinhos)
                            // A conta de "qual múltiplo" usa total_pushed para manter coerência
                            let phase_count = match self.mode {
                                ViewMode::Rolling => self.total_pushed,
                                _ => self.head_idx as u64,
                            };
                            // k_base: quantos quadradinhos inteiros foram "consumidos" até o início da janela
                            let k_base_offset = ((phase_count as f64 * dx / s).floor() as i64).rem_euclid(5) as usize;
                            let mut k_rel = 0usize;
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
