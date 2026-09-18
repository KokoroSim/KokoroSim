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

pub struct Plotter {
    canvas_id: String,
    buffer: Vec<f64>,
    capacity: usize,
    ghost_buffer: Option<Vec<f64>>,
    mode: ViewMode,
    head_idx: usize,
    paged_frozen: bool,
    single_state: SingleState,
    sound_markers: Vec<(usize, u8)>, // (index, sound_code)
}

impl Plotter {
    pub fn new(canvas_id: &str, capacity: usize) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            buffer: Vec::with_capacity(capacity),
            capacity,
            ghost_buffer: None,
            mode: ViewMode::Rolling,
            head_idx: 0,
            paged_frozen: false,
            single_state: SingleState::Armed,
            sound_markers: Vec::new(),
        }
    }

    pub fn set_mode(&mut self, mode: ViewMode) {
        if self.mode != mode {
            self.mode = mode;
            self.head_idx = 0;
            self.paged_frozen = false;
            self.single_state = SingleState::Armed;
            self.sound_markers.clear();
            if self.buffer.len() < self.capacity {
                self.buffer.resize(self.capacity, 0.0);
            }
        }
    }

    pub fn capture_ghost(&mut self) {
        if !self.buffer.is_empty() {
            self.ghost_buffer = Some(self.buffer.clone());
        }
    }

    #[allow(dead_code)]
    pub fn clear_ghost(&mut self) {
        self.ghost_buffer = None;
    }

    pub fn arm_single(&mut self) {
        self.single_state = SingleState::Armed;
        self.head_idx = 0;
        self.sound_markers.clear();
    }

    pub fn push(&mut self, value: f64, sound_code: u32, is_sa_fire: bool) {
        let sound_flags = (sound_code & 7) as u8;

        match self.mode {
            ViewMode::Rolling => {
                if self.buffer.len() >= self.capacity {
                    self.buffer.remove(0);
                }
                self.buffer.push(value);

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
                let erase_samples = 20;
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
                    self.buffer.resize(self.capacity, value);
                }

                if is_sa_fire {
                    // Novo batimento: reseta caneta para a margem esquerda
                    self.head_idx = 0;
                }

                self.buffer[self.head_idx] = value;
                let cur = self.head_idx;
                let erase_samples = 20;
                let erase_end = (cur + erase_samples).min(self.capacity);
                self.sound_markers.retain(|(idx, _)| *idx < cur || *idx >= erase_end);

                if sound_flags != 0 {
                    self.sound_markers.push((cur, sound_flags));
                }
                self.head_idx = (self.head_idx + 1) % self.capacity;
            }

            ViewMode::TriggeredSingle => {
                if self.buffer.len() < self.capacity {
                    self.buffer.resize(self.capacity, value);
                }

                match self.single_state {
                    SingleState::Armed => {
                        if is_sa_fire {
                            self.head_idx = 0;
                            self.single_state = SingleState::Recording;
                            self.sound_markers.clear();
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

                    // Limpa fundo e desenha linhas verticais dos eventos de áudio (se for o plotter primário do canvas)
                    if clear {
                        ctx.set_fill_style_str("#000000");
                        ctx.fill_rect(0.0, 0.0, width, height);

                        let dx = width / (self.capacity as f64 - 1.0);
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

                    let dx = width / (self.capacity as f64 - 1.0);
                    let range_y = max_y - min_y;

                    // 1. Desenha Onda Fantasma (Snapshot prévio em memória com transparência)
                    if show_ghost {
                        if let Some(ref ghost) = self.ghost_buffer {
                            if !ghost.is_empty() {
                                ctx.begin_path();
                                ctx.set_stroke_style_str(&format_ghost_color(color));
                                ctx.set_line_width(1.5);
                                ctx.set_shadow_blur(0.0);

                                for (i, &val) in ghost.iter().enumerate() {
                                    let x = i as f64 * dx;
                                    let y = height - ((val - min_y) / range_y) * height;
                                    if i == 0 {
                                        ctx.move_to(x, y);
                                    } else {
                                        ctx.line_to(x, y);
                                    }
                                }
                                ctx.stroke();
                            }
                        }
                    }

                    // 2. Desenha o traçado ativo de acordo com o ViewMode
                    ctx.set_stroke_style_str(color);
                    ctx.set_line_width(2.0);
                    ctx.set_shadow_color(color);
                    ctx.set_shadow_blur(5.0);

                    match self.mode {
                        ViewMode::Rolling => {
                            ctx.begin_path();
                            for (i, &val) in self.buffer.iter().enumerate() {
                                let x = i as f64 * dx;
                                let y = height - ((val - min_y) / range_y) * height;
                                if i == 0 {
                                    ctx.move_to(x, y);
                                } else {
                                    ctx.line_to(x, y);
                                }
                            }
                            ctx.stroke();
                        }

                        ViewMode::Sweep | ViewMode::TriggeredAuto => {
                            let erase_samples = 20;
                            let cur = self.head_idx;
                            let erase_end = (cur + erase_samples).min(self.capacity);

                            // Segmento A: da barra apagadora até o final (traço do ciclo anterior)
                            if erase_end < self.capacity {
                                ctx.begin_path();
                                for i in erase_end..self.capacity {
                                    let x = i as f64 * dx;
                                    let y = height - ((self.buffer[i] - min_y) / range_y) * height;
                                    if i == erase_end {
                                        ctx.move_to(x, y);
                                    } else {
                                        ctx.line_to(x, y);
                                    }
                                }
                                ctx.stroke();
                            }

                            // Segmento B: do início da tela até a caneta atual (traço do ciclo novo)
                            if cur > 0 {
                                ctx.begin_path();
                                for i in 0..cur {
                                    let x = i as f64 * dx;
                                    let y = height - ((self.buffer[i] - min_y) / range_y) * height;
                                    if i == 0 {
                                        ctx.move_to(x, y);
                                    } else {
                                        ctx.line_to(x, y);
                                    }
                                }
                                ctx.stroke();
                            }

                            // Barra apagadora preta à frente da caneta
                            if clear {
                                let head_x = cur as f64 * dx;
                                let bar_w = (erase_samples as f64 * dx).max(16.0);
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
                                for i in 0..end {
                                    let x = i as f64 * dx;
                                    let y = height - ((self.buffer[i] - min_y) / range_y) * height;
                                    if i == 0 {
                                        ctx.move_to(x, y);
                                    } else {
                                        ctx.line_to(x, y);
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
                                for i in 0..end {
                                    let x = i as f64 * dx;
                                    let y = height - ((self.buffer[i] - min_y) / range_y) * height;
                                    if i == 0 {
                                        ctx.move_to(x, y);
                                    } else {
                                        ctx.line_to(x, y);
                                    }
                                }
                                ctx.stroke();
                            }
                        }
                    }
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
            return format!("rgba({}, {}, {}, 0.28)", r, g, b);
        }
    }
    "rgba(150, 150, 150, 0.28)".to_string()
}
