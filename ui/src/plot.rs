use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Plotter {
    canvas_id: String,
    buffer: Vec<f64>,
    capacity: usize,
}

impl Plotter {
    pub fn new(canvas_id: &str, capacity: usize) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: f64) {
        if self.buffer.len() >= self.capacity {
            self.buffer.remove(0);
        }
        self.buffer.push(value);
    }

    pub fn draw(&self, min_y: f64, max_y: f64, color: &str, clear: bool) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(canvas) = document.get_element_by_id(&self.canvas_id) {
            if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                // Ensure canvas size matches its client display size to avoid blurriness
                let width = canvas.client_width() as f64;
                let height = canvas.client_height() as f64;
                
                if canvas.width() as f64 != width || canvas.height() as f64 != height {
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);
                }

                if let Ok(Some(ctx)) = canvas.get_context("2d") {
                    let ctx = ctx.dyn_into::<CanvasRenderingContext2d>().unwrap();
                    
                    // Clear background conditionally
                    if clear {
                        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
                        ctx.fill_rect(0.0, 0.0, width, height);
                    }

                    if self.buffer.is_empty() {
                        return;
                    }

                    // Draw grid (optional, can add later)

                    // Draw line
                    ctx.begin_path();
                    ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str(color));
                    ctx.set_line_width(2.0);
                    ctx.set_shadow_color(color);
                    ctx.set_shadow_blur(5.0);

                    let dx = width / (self.capacity as f64 - 1.0);
                    let range_y = max_y - min_y;

                    for (i, &val) in self.buffer.iter().enumerate() {
                        let x = i as f64 * dx;
                        // Map y from [min_y, max_y] to [height, 0]
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
    }
}
